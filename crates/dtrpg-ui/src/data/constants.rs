//! Constants

//! Well-known service name and account key constants for the credential store.
//!
//! All credential store call sites MUST use these constants rather than
//! inline strings to ensure consistent namespacing and enable targeted
//! deletion on uninstall.

/// The threshold (in `added_order`) below which an item counts as recently added.
pub const RECENTLY_ADDED_THRESHOLD: u32 = 90;

pub const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub const MONTH_ABBRS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub const APP_NAME: &str = "dtrpg";

/// Reverse-domain bundle identifier.
///
/// macOS convention names filesystem locations (Application Support, Preferences,
/// Caches) with the reverse-domain bundle ID rather than the bare app name. Used
/// by [`crate::data::paths`] for those directories; also reused as the keyring
/// service namespace via [`KEYRING_SERVICE`].
pub const MACOS_BUNDLE_ID: &str = "com.pilgrimagesoftware.dtrpg";

pub const COLLECTIONS_CACHE_FILE: &str = "collections_cache.json";
pub const COLLECTIONS_CACHE_TMP: &str = "collections_cache.json.tmp";
pub const CATALOG_CACHE_FILE: &str = "catalog_cache.json";
pub const CATALOG_CACHE_TMP: &str = "catalog_cache.json.tmp";
pub const CATALOG_CACHE_METADATA_FILE: &str = "catalog_cache_meta.json";
pub const AVATAR_CACHE_FILE: &str = "avatar";

/// Reverse-DNS service namespace used for all keyring entries.
pub const KEYRING_SERVICE: &str = MACOS_BUNDLE_ID;

/// Account key for the DriveThruRPG API key credential.
///
/// This is the only credential persisted to the keychain. Access tokens and
/// refresh tokens are kept in memory and re-acquired at startup.
pub const KEYRING_API_KEY: &str = "api-key";

pub const RECENT_CAP: usize = 25;
pub const EXPIRY_SECS: u64 = 15;
pub const ERROR_EXPIRY_SECS: u64 = 120;

/// Default width of the detail panel, in pixels.
pub const DETAIL_PANEL_DEFAULT_WIDTH: f32 = 320.0;
/// Minimum allowed detail panel width, in pixels.
pub const DETAIL_PANEL_MIN_WIDTH: f32 = 240.0;
/// Maximum allowed detail panel width, in pixels.
pub const DETAIL_PANEL_MAX_WIDTH: f32 = 600.0;
/// Maximum width of the detail panel's cover thumbnail, in pixels.
///
/// The thumbnail no longer grows with the panel past this size; instead it
/// stays centered horizontally at the top of the panel.
pub const DETAIL_PANEL_COVER_MAX_WIDTH: f32 = DETAIL_PANEL_DEFAULT_WIDTH;
