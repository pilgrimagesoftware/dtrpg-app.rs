## Why

The app has no crash/error reporting today: `logging.rs` writes WARN+ to stderr and a local
rolling log file, but nothing leaves the user's machine. Official CI-built releases should
report crashes and errors to Sentry so regressions are visible before users file bug reports.
Source builds (contributors, `cargo run` during development) must not silently phone home to a
project Sentry DSN they didn't provide, and must not fail to build or run if Sentry credentials
are absent.

## What Changes

- Add a `sentry` crate dependency (with `tracing` integration) behind a `sentry` Cargo feature,
  default-off, so `cargo build`/`cargo test` from source never pull in or activate it unless
  explicitly opted in.
- Read the Sentry DSN (and optional environment/release overrides) from environment variables at
  process startup. No DSN present, or the `sentry` feature not compiled in, means Sentry is never
  initialized.
- Add a `tracing` layer that forwards ERROR (and optionally WARN) level events and panics to
  Sentry when initialized, alongside the existing stderr/file layers in `logging.rs`.
- Log a single INFO line at startup stating whether Sentry reporting is active or disabled, and
  why (feature not compiled in / DSN not set / initialized).
- Inject the Sentry DSN as a GitHub Actions secret in the release build workflow
  (`.github/workflows/build.yaml`), compiled in only for tagged/release builds, not PR builds.
- Document the required environment variables and the opt-in feature flag in the crate's
  `README.md`.

## Capabilities

### New Capabilities
- `crash-reporting`: conditional Sentry initialization driven by build-time feature flag and
  runtime environment variables, with explicit fallback logging when disabled.

### Modified Capabilities
(none — no existing capability's requirements change)

## Impact

- `Cargo.toml` (workspace): new optional dependency `sentry`, new `sentry` feature on
  `dtrpg-core`.
- `crates/dtrpg-core/src/logging.rs`: add Sentry tracing layer, conditional on feature + env var.
- `crates/dtrpg-core/src/constants.rs`: new env var name constants (DSN, environment, release).
- `crates/dtrpg-core/src/main.rs`: no functional change, `logging::init()` return type may grow to
  carry Sentry guard alongside the existing `WorkerGuard`.
- `.github/workflows/build.yaml` (and any release/packaging workflow): build release artifacts
  with `--features sentry` and inject `SENTRY_DSN` (and related) as a repository/environment
  secret.
- `README.md`: document the opt-in behavior for contributors building from source.
