## Why

The Advanced settings section only offers a "Clear cache" button with no visibility into what's actually cached. Users can't tell how much metadata, cover art, or other cached data has accumulated before deciding to clear it, and there's no way to inspect the cache contents directly on disk.

## What Changes

- The Advanced section gains a "Cache details" area showing per-type counts of cached data: catalog/collections metadata entries, cached cover thumbnails, and whether the avatar image is cached.
- Add an "Open cache folder" button that reveals the app cache directory (`app_cache_dir()`) in the OS's native file manager (Finder/Explorer/file manager), reusing the existing `reveal_in_file_manager` helper already used for the storage location.
- Counts are computed on demand (when the Advanced section is shown / refreshed) rather than tracked continuously, since cache contents only change via explicit user or app actions.
- "Clear cache" continues to work as-is; after clearing, the displayed counts refresh to reflect the now-empty cache.
- The same "Cache details" area also surfaces the app's cache-related timeouts and cooldowns as static, read-only values: catalog cache staleness window, manual reload cooldown, item availability check cooldown, item check batch cooldown, item check batch timer interval, and thumbnail retry cooldown. These are fixed build-time constants (`data/constants.rs`), not something the user configures — the goal is visibility into why the app behaves the way it does (e.g. "why didn't my reload do anything just now"), not a settings form.
- Every data point in the section (counts and timings alike) gets a concise short label plus a one-line explanatory description, shown as small secondary text beneath the label — not just a bare number.

## Capabilities

### New Capabilities

- `settings-cache-details`: Advanced settings displays counts of cached data by type (metadata entries, cover thumbnails, avatar cache presence), the cache-related timeout/cooldown constants that govern cache and check behavior, and provides a button to reveal the cache folder in the OS file manager. Every data point carries a short label and an explanatory description.

### Modified Capabilities

(none)

## Impact

- `dtrpg-ui/src/controllers/settings.rs`: add a method to compute current cache counts (reading `cache_dir()`, `covers_dir()`, and the avatar cache file) and a method to open the cache folder via `reveal_in_file_manager(&app_cache_dir())`; extend `SettingsSnapshot` with the computed counts.
- `dtrpg-ui/src/ui/views/settings_advanced_view.rs`: add a "Cache details" subsection listing the counts, the timing constants, and an "Open cache folder" button, alongside the existing "Clear cache" button. Each stat row shows a label, a value, and a short description line.
- `dtrpg-ui/src/data/paths.rs`: no changes — `app_cache_dir()`, `cache_dir()`, `covers_dir()` already exist and are reused as-is.
- `dtrpg-ui/src/data/constants.rs`: gains `STALE_SECS` (moved from `catalog_cache.rs`) and `THUMBNAIL_COOLDOWN_SECS` (moved from `library.rs`), consolidating all cooldown/timeout constants in one place per this repo's Rust conventions ("always put constants in a single location in a crate"). Both become `pub` so the settings view can display them; call sites update their imports.
- No changes to persisted data formats, credential storage, or SDK/service layers.
