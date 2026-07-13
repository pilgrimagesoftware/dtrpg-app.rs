//! Build-time identifying information, captured by `build.rs` and exposed
//! here for the About settings section.

/// Short git commit hash the binary was built from, or `"unknown"` if `git`
/// wasn't available or the source wasn't a git checkout at build time.
pub const GIT_HASH: &str = env!("DTRPG_GIT_HASH");

/// UTC build date as `YYYY-MM-DD`.
pub const BUILD_DATE: &str = env!("DTRPG_BUILD_DATE");

/// Target triple the binary was compiled for (e.g. `aarch64-apple-darwin`).
pub const TARGET: &str = env!("DTRPG_TARGET");
