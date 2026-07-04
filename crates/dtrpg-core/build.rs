//! Embeds Sentry configuration supplied at build time into the compiled
//! binary when the `sentry` feature is enabled.
//!
//! CI release builds set `DTRPG_SENTRY_DSN` (and optionally
//! `DTRPG_SENTRY_ENVIRONMENT`/`DTRPG_SENTRY_RELEASE`) as build-time
//! environment variables; this script forwards them into compile-time
//! `rustc-env` variables so the shipped binary carries the configuration
//! without needing an installer or launcher to set runtime environment
//! variables. `logging::init` still checks the runtime environment first,
//! so contributors can override these values locally without rebuilding.
//!
//! Without the `sentry` feature, or without these variables set at build
//! time, this script is a no-op.

fn main() {
    println!("cargo:rerun-if-env-changed=DTRPG_SENTRY_DSN");
    println!("cargo:rerun-if-env-changed=DTRPG_SENTRY_ENVIRONMENT");
    println!("cargo:rerun-if-env-changed=DTRPG_SENTRY_RELEASE");

    if std::env::var("CARGO_FEATURE_SENTRY").is_err() {
        return;
    }

    let dsn = std::env::var("DTRPG_SENTRY_DSN").unwrap_or_default();
    println!("cargo:rustc-env=DTRPG_SENTRY_DSN_BUILTIN={dsn}");

    let environment = std::env::var("DTRPG_SENTRY_ENVIRONMENT").unwrap_or_default();
    println!("cargo:rustc-env=DTRPG_SENTRY_ENVIRONMENT_BUILTIN={environment}");

    let release = std::env::var("DTRPG_SENTRY_RELEASE").unwrap_or_default();
    println!("cargo:rustc-env=DTRPG_SENTRY_RELEASE_BUILTIN={release}");
}
