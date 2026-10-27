pub const APPLICATION_KEY_ENV: &str = "DTRPG_APPLICATION_KEY";
pub const ACCESS_TOKEN_ENV: &str = "DTRPG_ACCESS_TOKEN";
pub const REFRESH_TOKEN_ENV: &str = "DTRPG_REFRESH_TOKEN";
pub const REFRESH_TOKEN_TTL_ENV: &str = "DTRPG_REFRESH_TOKEN_TTL";
pub const API_BASE_URL_ENV: &str = "DTRPG_API_BASE_URL";

// These four are only read from `logging::init_sentry_client`, which is
// compiled only under the `sentry` feature; allow dead-code lint in default
// builds (`cargo check`/`cargo clippy` without `--tests`).

/// Sentry DSN. Presence of this variable (with the `sentry` feature compiled
/// in) is what switches Sentry reporting on; absence leaves it off.
#[cfg_attr(not(feature = "sentry"), allow(dead_code))]
pub const SENTRY_DSN_ENV: &str = "DTRPG_SENTRY_DSN";
/// Optional override for the Sentry environment tag. Defaults to
/// `"production"`.
#[cfg_attr(not(feature = "sentry"), allow(dead_code))]
pub const SENTRY_ENVIRONMENT_ENV: &str = "DTRPG_SENTRY_ENVIRONMENT";
/// Optional override for the Sentry release tag. Defaults to
/// `CARGO_PKG_VERSION`.
#[cfg_attr(not(feature = "sentry"), allow(dead_code))]
pub const SENTRY_RELEASE_ENV: &str = "DTRPG_SENTRY_RELEASE";
#[cfg_attr(not(feature = "sentry"), allow(dead_code))]
pub const SENTRY_DEFAULT_ENVIRONMENT: &str = "production";

pub const DEFAULT_COLOR: &str = "#2E3A45";
pub const BYTES_PER_MB: f64 = 1_048_576.0;

/// Base URL that relative cover image paths (`OrderProductInfo::image`,
/// `thumbnail`, etc.) are appended to in order to form a fetchable thumbnail
/// URL.
pub const DTRPG_IMAGES_BASE_URL: &str = "https://api.drivethrurpg.com/images/";

/// Total attempts (including the first, non-retry attempt) `download_item`
/// makes before giving up on a retryable transfer failure.
pub const MAX_DOWNLOAD_ATTEMPTS: u32 = 4;
/// Base delay in seconds for the first download retry's exponential backoff.
pub const DOWNLOAD_RETRY_BASE_DELAY_SECS: u64 = 2;
/// Maximum delay in seconds any single download retry will wait.
pub const DOWNLOAD_RETRY_MAX_DELAY_SECS: u64 = 30;

/// How many matching titles the smart collection editor lists as a preview.
pub const SMART_PREVIEW_TITLE_LIMIT: usize = 50;

/// Repository bug and feature reports are filed against, as `owner/name`.
pub const REPORT_REPO: &str = "pilgrimagesoftware/dtrpg-app.rs";
/// Most lines of the application log a report carries.
pub const REPORT_LOG_TAIL_LINES: usize = 200;
/// Most bytes read from the end of the application log for a report.
pub const REPORT_LOG_TAIL_BYTES: usize = 24 * 1024;
/// Most screenshots a report can name.
pub const REPORT_MAX_SCREENSHOTS: usize = 5;
/// Image extensions GitHub renders inline, which a report accepts.
pub const REPORT_SCREENSHOT_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif"];
/// Where a `gh` installed outside the PATH of a Finder-launched app lives.
pub const GH_FALLBACK_PATHS: &[&str] = &["/opt/homebrew/bin/gh", "/usr/local/bin/gh"];
