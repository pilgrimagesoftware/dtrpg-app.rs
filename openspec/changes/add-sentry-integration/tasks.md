## 1. Dependencies and feature flag

- [ ] 1.1 Add `sentry` and `sentry-tracing` to `[workspace.dependencies]` with
      `default-features = false` plus the minimal feature set needed (native-tls or rustls
      backend, panic capture).
- [ ] 1.2 Add a `sentry` feature to `crates/dtrpg-core/Cargo.toml` that enables the optional
      `sentry`/`sentry-tracing` dependencies. Confirm the feature is not part of any default
      feature set.
- [ ] 1.3 Add `DTRPG_SENTRY_DSN`, `DTRPG_SENTRY_ENVIRONMENT`, `DTRPG_SENTRY_RELEASE` constants to
      `crates/dtrpg-core/src/constants.rs`.

## 2. Runtime initialization

- [ ] 2.1 Add a `#[cfg(feature = "sentry")]` module or function in `logging.rs` that reads
      `DTRPG_SENTRY_DSN` and, if present, calls `sentry::init` with environment/release taken from
      `DTRPG_SENTRY_ENVIRONMENT`/`DTRPG_SENTRY_RELEASE` (defaulting to `"production"` and
      `CARGO_PKG_VERSION`).
- [ ] 2.2 Add the `sentry_tracing::layer()` to the existing `tracing_subscriber::registry()` chain
      in `init()`, alongside the stderr and file layers, only when Sentry initialized
      successfully.
- [ ] 2.3 Add a `#[cfg(not(feature = "sentry"))]` fallback path that performs no Sentry setup.
- [ ] 2.4 Emit exactly one INFO log line at the end of `init()` stating the resulting Sentry
      status (active / disabled-no-dsn / disabled-not-compiled).
- [ ] 2.5 Extend `logging::init()`'s return type (or add a sibling guard type) to keep the Sentry
      client guard alive for the process lifetime, matching the existing `WorkerGuard` pattern in
      `main.rs`.

## 3. Tests

- [ ] 3.1 Add a unit test that builds without the `sentry` feature and asserts the "disabled -
      not compiled in" log path is exercised (e.g. via a test-only log capture).
- [ ] 3.2 Add a unit test (feature-gated, `cargo test --features sentry`) asserting that
      `DTRPG_SENTRY_DSN` unset results in the "disabled - no DSN" path.
- [ ] 3.3 Add a unit test (feature-gated) asserting that a set `DTRPG_SENTRY_DSN` results in the
      "active" path, using a fake/local DSN that does not require network access (Sentry's DSN
      parsing does not require reachability to construct the client).

## 4. CI wiring

- [ ] 4.1 Add a `SENTRY_DSN` repository secret (document the requirement; actual secret value is
      set by a maintainer in GitHub repo settings, not committed).
- [ ] 4.2 Update `.github/workflows/build.yaml` to build with `--features sentry` and pass
      `DTRPG_SENTRY_DSN: ${{ secrets.SENTRY_DSN }}` (and `DTRPG_SENTRY_RELEASE` from the bumped
      version) into the build/test steps.
- [ ] 4.3 Confirm `.github/workflows/pr.yaml` is unchanged and does not reference the `sentry`
      feature or any Sentry secret.

## 5. Documentation

- [ ] 5.1 Document `DTRPG_SENTRY_DSN`, `DTRPG_SENTRY_ENVIRONMENT`, `DTRPG_SENTRY_RELEASE`, and the
      `sentry` feature flag in `README.md`, including the explicit statement that source builds
      without these values never initialize Sentry.
- [ ] 5.2 Update `crates/dtrpg-core/src/logging.rs` module-level doc comment to mention the
      optional Sentry layer and its activation conditions.

## 6. Verification

- [ ] 6.1 Run `cargo build` and `cargo test` with no feature flags; confirm no Sentry code is
      compiled in and the disabled log line is emitted.
- [ ] 6.2 Run `cargo build --features sentry` with no `DTRPG_SENTRY_DSN` set; confirm the app
      starts normally and logs "disabled - no DSN".
- [ ] 6.3 Run `cargo run --features sentry` with a test DSN set; confirm an intentionally
      triggered ERROR-level `tracing` event appears as a Sentry issue.
- [ ] 6.4 Run `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt --all --
      --check`.
