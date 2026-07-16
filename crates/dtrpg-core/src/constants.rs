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

/// Retry attempts (including the first) for catalog synchronization requests.
/// See [`crate::services::retry`].
pub const CATALOG_SYNC_MAX_ATTEMPTS: u32 = 4;
/// Base backoff delay (seconds) for catalog synchronization retries.
pub const CATALOG_SYNC_RETRY_BASE_DELAY_SECS: u64 = 2;
/// Maximum backoff delay (seconds) for catalog synchronization retries.
pub const CATALOG_SYNC_RETRY_MAX_DELAY_SECS: u64 = 30;

/// Retry attempts (including the first) for cover/avatar image-cache fetches.
pub const IMAGE_CACHE_MAX_ATTEMPTS: u32 = 4;
/// Base backoff delay (seconds) for image-cache fetch retries.
pub const IMAGE_CACHE_RETRY_BASE_DELAY_SECS: u64 = 2;
/// Maximum backoff delay (seconds) for image-cache fetch retries.
pub const IMAGE_CACHE_RETRY_MAX_DELAY_SECS: u64 = 30;

/// Minimum interval between fresh-install catalog initialization requests,
/// gating redundant requests per [`crate::services::retry`]'s caller in
/// `dtrpg-ui`'s `LibraryController`.
pub const CATALOG_FRESH_INSTALL_MIN_REQUEST_INTERVAL_SECS: u64 = 60;

/// How long a network-monitor reachability check result stays cached before a
/// new check is triggered.
pub const NETWORK_MONITOR_CACHE_TTL_SECS: u64 = 5;

/// Interval for the recurring long-running-session catalog-staleness timer.
pub const CATALOG_REFRESH_TIMER_INTERVAL_SECS: u64 = 3600;
