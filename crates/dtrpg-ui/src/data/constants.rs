//! Constants

//! Well-known service name and account key constants for the credential store.
//!
//! All credential store call sites MUST use these constants rather than
//! inline strings to ensure consistent namespacing and enable targeted
//! deletion on uninstall.

/// The threshold (in `added_order`) below which an item counts as recently
/// added.
pub const RECENTLY_ADDED_THRESHOLD: u32 = 90;

pub const MONTH_ABBRS: [&str; 12] =
    ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

pub const APP_NAME: &str = "dtrpg";

/// Reverse-domain bundle identifier.
///
/// macOS convention names filesystem locations (Application Support,
/// Preferences, Caches) with the reverse-domain bundle ID rather than the bare
/// app name. Used by [`crate::data::paths`] for those directories; also reused
/// as the keyring service namespace via [`KEYRING_SERVICE`].
pub const MACOS_BUNDLE_ID: &str = "com.pilgrimagesoftware.dtrpg";

pub const COLLECTIONS_CACHE_FILE: &str = "collections_cache.json";
pub const COLLECTIONS_CACHE_TMP: &str = "collections_cache.json.tmp";
pub const CATALOG_CACHE_FILE: &str = "catalog_cache.json";
pub const CATALOG_CACHE_TMP: &str = "catalog_cache.json.tmp";
pub const CATALOG_CACHE_METADATA_FILE: &str = "catalog_cache_meta.json";
pub const AVATAR_CACHE_FILE: &str = "avatar";

/// 7 days in seconds — caches older than this are considered stale.
pub const STALE_SECS: u64 = 7 * 24 * 60 * 60;

/// Minimum interval between user-requested full catalog reloads ("Catalog >
/// Reload"), keyed off `CacheMetadata::saved_at_secs`.
///
/// Distinct from the on-disk cache's 7-day passive staleness window (see
/// [`STALE_SECS`]): that constant answers "is the cached data old enough
/// that a *passive* load should refresh it," while this one answers "was a
/// *manual* reload already attempted moments ago." 60 seconds is long enough
/// to absorb accidental double-invocations (a stuck keybinding or an
/// impatient double-click) without meaningfully delaying a deliberate second
/// reload.
pub const FORCE_RELOAD_COOLDOWN_SECS: u64 = 60;

/// Minimum interval between re-checking the same catalog item's availability
/// against the server (on-demand, via viewing its details).
///
/// Short enough that a check still feels "fresh" relative to a browsing
/// session; long enough to absorb a user repeatedly reopening the same
/// item's detail view without issuing a redundant network call each time.
pub const ITEM_CHECK_COOLDOWN_SECS: u64 = 300;

/// Minimum interval between per-item availability check batches, whether
/// triggered manually or by the automatic periodic timer. Shared by both
/// triggers so neither can flood the API by stacking on top of the other.
pub const ITEM_CHECK_BATCH_COOLDOWN_SECS: u64 = 900;

/// Maximum number of items selected into a single availability check batch.
pub const ITEM_CHECK_BATCH_SIZE: usize = 50;

/// Interval between wake-ups of the automatic periodic check-batch timer
/// loop. Independent of `ITEM_CHECK_BATCH_COOLDOWN_SECS` — this is how often
/// the loop *wakes up to ask*, not how often a batch is actually allowed to
/// run; each wake calls `request_check_batch`, which applies the real
/// cooldown gate.
pub const ITEM_CHECK_BATCH_TIMER_SECS: u64 = 300;

/// Minimum interval between cover thumbnail fetch attempts for the same item.
///
/// Absorbs re-render churn (e.g. scrolling an item back into view) without
/// hammering the thumbnail source on every frame the item happens to be
/// visible.
pub const THUMBNAIL_COOLDOWN_SECS: u64 = 300;

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

/// Maximum number of entries retained in the durable alert history log.
///
/// Unlike the transient activity panel's `recent` list, entries here never
/// expire on a timer — the oldest entry is evicted only once this cap is
/// reached.
pub const ALERT_LOG_CAP: usize = 100;

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

/// Maximum character count for a detail tab's title before it is truncated
/// with an ellipsis.
///
/// The tab strip has no fixed per-tab width, so a long catalog item title
/// otherwise stretches its tab (and the whole strip) rather than eliding —
/// see [`crate::util::text::truncate_with_ellipsis`].
pub const DETAIL_TAB_TITLE_MAX_CHARS: usize = 40;

/// Width of the single-click item popover, in pixels.
pub const ITEM_POPOVER_WIDTH: f32 = 260.0;
/// Gap between the item popover and the catalog entry it's anchored beside.
pub const ITEM_POPOVER_MARGIN: f32 = 8.0;

/// Default body-font family, serif-leaning to match the app's old-style body
/// text. Only a starting point — the Appearance settings page lets the user
/// pick any font actually installed on their system (see
/// `cx.text_system().all_font_names()`), not just this default.
#[cfg(target_os = "macos")]
pub const DEFAULT_BODY_FONT: &str = "Hoefler Text";
/// Default body-font family, serif-leaning to match the app's old-style body
/// text.
#[cfg(target_os = "windows")]
pub const DEFAULT_BODY_FONT: &str = "Georgia";
/// Default body-font family, serif-leaning to match the app's old-style body
/// text.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub const DEFAULT_BODY_FONT: &str = "Liberation Serif";

/// Default value-font family, sans-serif, used to visually distinguish data
/// values (e.g. Advanced settings' "Cache details" rows) from the default
/// serif body font.
#[cfg(target_os = "macos")]
pub const DEFAULT_VALUE_FONT: &str = "Gotham";
/// Default value-font family, sans-serif, used to visually distinguish data
/// values from the default serif body font.
#[cfg(target_os = "windows")]
pub const DEFAULT_VALUE_FONT: &str = "Segoe UI";
/// Default value-font family, sans-serif, used to visually distinguish data
/// values from the default serif body font.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub const DEFAULT_VALUE_FONT: &str = "DejaVu Sans";

/// Default label-font family, sans-serif, used to visually distinguish
/// field/row labels (e.g. the detail tab's metadata labels) from the default
/// serif body font.
#[cfg(target_os = "macos")]
pub const DEFAULT_LABEL_FONT: &str = "Gotham";
/// Default label-font family, sans-serif, used to visually distinguish
/// field/row labels from the default serif body font.
#[cfg(target_os = "windows")]
pub const DEFAULT_LABEL_FONT: &str = "Segoe UI";
/// Default label-font family, sans-serif, used to visually distinguish
/// field/row labels from the default serif body font.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub const DEFAULT_LABEL_FONT: &str = "DejaVu Sans";

/// Default monospace-font family, used for fixed-width data such as the
/// masked API key hint.
#[cfg(target_os = "macos")]
pub const DEFAULT_MONO_FONT: &str = "Menlo";
/// Default monospace-font family, used for fixed-width data such as the
/// masked API key hint.
#[cfg(target_os = "windows")]
pub const DEFAULT_MONO_FONT: &str = "Consolas";
/// Default monospace-font family, used for fixed-width data such as the
/// masked API key hint.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub const DEFAULT_MONO_FONT: &str = "Liberation Mono";

/// Default shared UI text size, in points/pixels — matches
/// `gpui`'s and `gpui_component`'s own stock default so nothing visibly
/// changes until the user adjusts it in Settings > Appearance.
pub const DEFAULT_UI_TEXT_SIZE: f32 = 16.0;
/// Minimum shared UI text size the Appearance page's stepper allows.
pub const MIN_UI_TEXT_SIZE: f32 = 12.0;
/// Maximum shared UI text size the Appearance page's stepper allows —
/// generous enough to meaningfully help low-vision users without breaking
/// layouts sized around the default.
pub const MAX_UI_TEXT_SIZE: f32 = 28.0;
/// Monospace text renders at this fraction of the shared UI text size,
/// matching `gpui_component::Theme`'s stock 13px-on-16px default ratio, so
/// code-like content (e.g. the masked API key hint) stays visually smaller
/// than body text even as the shared size changes.
pub const MONO_SIZE_RATIO: f32 = 13.0 / 16.0;
