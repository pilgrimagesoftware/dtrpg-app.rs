## Why

The Advanced settings section only offers a "Clear cache" button with no visibility into what's actually cached. Users can't tell how much metadata, cover art, or other cached data has accumulated before deciding to clear it, and there's no way to inspect the cache contents directly on disk.

## What Changes

- The Advanced section gains a "Cache details" area showing per-type counts of cached data: catalog/collections metadata entries, cached cover thumbnails, and whether the avatar image is cached.
- Add an "Open cache folder" button that reveals the app cache directory (`app_cache_dir()`) in the OS's native file manager (Finder/Explorer/file manager), reusing the existing `reveal_in_file_manager` helper already used for the storage location.
- Counts are computed on demand (when the Advanced section is shown / refreshed) rather than tracked continuously, since cache contents only change via explicit user or app actions.
- "Clear cache" continues to work as-is; after clearing, the displayed counts refresh to reflect the now-empty cache.

## Capabilities

### New Capabilities

- `settings-cache-details`: Advanced settings displays counts of cached data by type (metadata entries, cover thumbnails, avatar cache presence) and provides a button to reveal the cache folder in the OS file manager.

### Modified Capabilities

(none)

## Impact

- `dtrpg-ui/src/controllers/settings.rs`: add a method to compute current cache counts (reading `cache_dir()`, `covers_dir()`, and the avatar cache file) and a method to open the cache folder via `reveal_in_file_manager(&app_cache_dir())`; extend `SettingsSnapshot` with the computed counts.
- `dtrpg-ui/src/ui/views/settings_advanced_view.rs`: add a "Cache details" subsection listing the counts and an "Open cache folder" button, alongside the existing "Clear cache" button.
- `dtrpg-ui/src/data/paths.rs`: no changes — `app_cache_dir()`, `cache_dir()`, `covers_dir()` already exist and are reused as-is.
- No changes to persisted data formats, credential storage, or SDK/service layers.
