## Context

`crates/dtrpg-core/src/logging.rs::init()` sets up the process-wide `tracing` subscriber (stderr
+ rolling file layers) and is called once at the top of `main()`. There is no external error
reporting today. Two build paths exist:

- **CI release builds**: `.github/workflows/build.yaml`, triggered on push to `master`/`develop`,
  produces the artifacts (`cargo package`, docs, and eventually app bundles) that end users
  download.
- **Source builds**: `pr.yaml` (`cargo build`/`cargo clippy`/`cargo fmt --check` on PRs) and any
  contributor running `cargo build`/`cargo run` locally.

Neither path currently sets or reads any Sentry-related configuration.

## Goals / Non-Goals

**Goals:**
- Sentry is initialized if and only if a DSN is supplied at runtime AND the binary was compiled
  with the `sentry` feature enabled.
- A source build with no injected values never contacts Sentry, never fails to compile or run,
  and clearly logs that reporting is disabled.
- CI can produce a release build with Sentry wired in without hardcoding any DSN in the
  repository.
- Adding Sentry does not change existing local logging behavior (stderr + rolling file layers
  stay as-is).

**Non-Goals:**
- Symbol upload / debug-info management for crash symbolication (can be a follow-up; out of
  scope here).
- User-facing opt-out UI/consent flow — this change only covers the build/runtime plumbing.
- Sentry integration for the Swift app (`dtrpg-app/swift`) — tracked separately if needed.
- Building a full desktop-bundle release pipeline (dmg/AppImage/msi) — this change only adds the
  Sentry inputs to whatever workflow currently produces distributable release artifacts.

## Decisions

**Cargo feature gate (`sentry`), default-off.**
`dtrpg-core` gets an optional dependency on the `sentry` crate (`sentry-tracing` for the
`tracing` bridge) behind a `sentry` feature. Source builds (`cargo build`, `cargo test`, `cargo
run`) never enable it unless the builder explicitly passes `--features sentry`. This guarantees
the dependency isn't even compiled in for ordinary contributor builds, not just inert at runtime.
Alternative considered: always compile the dependency in and gate only at runtime via the DSN
env var. Rejected — it still pulls the crate and its transitive deps (and the `curl`/native-tls
dependency chain) into every contributor build, and the requirement explicitly separates
"built by CI with values injected" from "built by user from source."

**Runtime configuration via environment variables, read once at startup.**
New constants in `constants.rs`:
- `SENTRY_DSN_ENV = "DTRPG_SENTRY_DSN"` — presence is the on/off switch.
- `SENTRY_ENVIRONMENT_ENV = "DTRPG_SENTRY_ENVIRONMENT"` — optional, defaults to `"production"`.
- `SENTRY_RELEASE_ENV = "DTRPG_SENTRY_RELEASE"` — optional, defaults to the crate's
  `CARGO_PKG_VERSION` at compile time.

This mirrors the existing pattern for `DTRPG_APPLICATION_KEY` / `DTRPG_API_BASE_URL` etc., so it
is consistent with how the app already receives CI/user-supplied configuration.
Alternative considered: a config file entry. Rejected — secrets belong in environment variables
injected by CI, not committed or user-edited config files, and this matches the credential/env
patterns already in the codebase.

**Initialization lives in `logging.rs`, alongside the existing subscriber setup.**
`logging::init()` gains a `#[cfg(feature = "sentry")]` branch: if `DTRPG_SENTRY_DSN` is set, call
`sentry::init(...)` and add a `sentry_tracing::layer()` to the existing `tracing_subscriber`
registry (in addition to, not instead of, the stderr/file layers). The Sentry client guard is
returned alongside the existing `WorkerGuard` so both stay alive for the process lifetime.
If the feature is not compiled in, or it is compiled in but no DSN is present, `init()` logs one
INFO line explaining which case applies and proceeds exactly as today.
Alternative considered: a separate `sentry.rs` module invoked from `main()` independently of
`logging.rs`. Rejected — Sentry's own tracing bridge needs to be one layer among the others in
the same `tracing_subscriber::registry()` call; splitting it out risks two competing
`Registry::init()` calls (the second panics).

**CI injects the DSN as a secret, only for the release build job.**
`build.yaml` (the workflow that produces release artifacts on push to `master`/`develop`) adds a
`SENTRY_DSN: ${{ secrets.SENTRY_DSN }}` environment variable to the build step and compiles with
`--features sentry`. `pr.yaml` (PR validation) is left untouched — it never sets the secret and
never enables the feature, so PR builds compile without Sentry, matching source-build behavior.
Passing the secret through as `DTRPG_SENTRY_DSN` at build time only sets the environment for that
CI job's own processes; the *shipped binary* still reads `DTRPG_SENTRY_DSN` at its own runtime
startup, so the value must also be present in the environment the packaged app runs in. Two
options for getting it there:
1. Bake it in via a build-time `env!("DTRPG_SENTRY_DSN")` read in a `build.rs`, embedding the DSN
   as a compile-time constant in the shipped binary.
2. Ship it as a runtime environment variable the launcher/installer sets.

This design picks **option 1** (compile-time embed via `build.rs` + `env!`, only when the
`sentry` feature is active) because Sentry DSNs are not secret in the security sense (they are
designed to be embedded in shipped clients; see Sentry's own docs) and because the app has no
installer-level mechanism today to inject runtime environment variables into a launched GUI app
bundle. The env-var *interface* (`DTRPG_SENTRY_DSN`) is kept as the source of truth so:
- Contributors building from source can still override it at runtime for local testing
  (`DTRPG_SENTRY_DSN=... cargo run --features sentry`).
- CI's injection point is a single environment variable, regardless of whether it ends up
  embedded at compile time or read at process start.

## Risks / Trade-offs

- **[Risk]** Embedding the DSN at compile time means it is visible to anyone who inspects the
  shipped binary → **Mitigation**: Sentry DSNs are designed to be public/embeddable (they only
  permit event submission, not read access); document this explicitly so it isn't mistaken for a
  secret leak.
- **[Risk]** `sentry-tracing`'s layer could swallow or slow down event delivery on flaky networks
  → **Mitigation**: Sentry's transport is async and non-blocking by default; no explicit action
  needed beyond using the crate's default transport.
- **[Risk]** Forgetting to also gate the `build.rs` embed behind the `sentry` feature would leak
  an empty/placeholder DSN into non-Sentry builds → **Mitigation**: `build.rs` only emits the
  `cfg`/`env!` wiring when `CARGO_FEATURE_SENTRY` is set, verified by a test build without the
  feature confirming no Sentry initialization log line appears.
- **[Trade-off]** No symbol upload / release-health wiring in this change, so crash reports may
  show unsymbolicated stack traces for release builds. Acceptable for an initial rollout;
  tracked as a follow-up.

## Migration Plan

1. Add the optional dependency and feature flag; land with no CI changes so it's inert.
2. Wire runtime init in `logging.rs` gated on the feature + DSN presence; verify locally with
   `cargo run --features sentry` plus a test DSN (Sentry's local relay or a throwaway project).
3. Add the `SENTRY_DSN` repository secret and update `build.yaml` to build with
   `--features sentry` and inject the secret.
4. Ship one release, confirm events arrive in Sentry, confirm a source build (no feature, no
   env var) logs "Sentry disabled" and never attempts network access.
5. Rollback: revert the `build.yaml` change (stop building with `--features sentry`) or unset the
   `SENTRY_DSN` secret — either fully disables Sentry with no code change required.

## Open Questions

- Does the eventual desktop-bundle packaging step (dmg/AppImage/msi, not yet built) need its own
  handling, or will it consume the same `build.yaml`-produced binary? Revisit when that pipeline
  exists.
- Should WARN-level events also go to Sentry (as breadcrumbs) in addition to ERROR-level events
  (as issues)? Default in this change: ERROR+ create Sentry events, WARN+ are attached as
  breadcrumbs for context, matching `sentry-tracing`'s recommended defaults.
