## 1. Data layer

- [x] 1.1 Add `pub fn avatar_cached() -> bool` to `dtrpg-ui/src/data/avatar.rs` (wraps `avatar_cache_path().exists()`), avoiding duplication of the avatar cache path elsewhere.
- [x] 1.2 Add a `CacheCounts { metadata_items: usize, cover_thumbnails: usize, avatar_cached: bool }` struct (or equivalent) in `dtrpg-ui/src/controllers/settings.rs` or a shared data module.
- [x] 1.3 Move `STALE_SECS` from `dtrpg-ui/src/data/catalog_cache.rs` into `dtrpg-ui/src/data/constants.rs` as `pub const STALE_SECS: u64`; update `catalog_cache.rs`'s `is_stale` to import it from `constants`; update the doc comment on `constants.rs`'s `FORCE_RELOAD_COOLDOWN_SECS` that currently references `catalog_cache::STALE_SECS` by path.
- [x] 1.4 Move `THUMBNAIL_COOLDOWN_SECS` from `dtrpg-ui/src/data/library.rs` into `dtrpg-ui/src/data/constants.rs` as `pub const THUMBNAIL_COOLDOWN_SECS: u64`; update `library.rs`'s `thumbnail_cooldown_elapsed` to import it from `constants`.

## 2. Controller

- [x] 2.1 Add a `SettingsController` method (e.g. `cache_counts()`) that reads `load_cache_metadata(&cache_dir()).map(|m| m.item_count).unwrap_or(0)` for the metadata count, counts entries via `std::fs::read_dir(covers_dir())` for the cover count, and calls `avatar_cached()` for the avatar indicator.
- [x] 2.2 Add a `SettingsController` method (e.g. `open_cache_folder()`) that creates `app_cache_dir()` if missing and calls `reveal_in_file_manager(&app_cache_dir())`, mirroring `reveal_storage_location`.
- [x] 2.3 Extend `SettingsSnapshot` to include the computed `CacheCounts` (or expose it as a separate accessor read directly by the view) so the Advanced section can render it.

## 3. View

- [x] 3.1 In `dtrpg-ui/src/ui/views/settings_advanced_view.rs`, add a local `format_static_duration(secs: u64) -> String` helper rendering "60 seconds" / "5 minutes" / "15 minutes" / "7 days" style output (distinct from `activity_panel_view.rs`'s elapsed-time `format_duration`).
- [x] 3.2 Add a local `stat_row(label: impl Into<SharedString>, value: impl Into<SharedString>, description: impl Into<SharedString>, colors: &ColorTokens) -> impl IntoElement` helper: bold label + value on one line, tertiary-color one-line description beneath.
- [x] 3.3 Add a "Cache details" subsection above or alongside "Clear cache" using `stat_row` for: metadata item count, cover thumbnail count, avatar-cached indicator, catalog cache staleness window, manual reload cooldown, item availability check cooldown, item check batch cooldown, item check batch timer interval, thumbnail retry cooldown.
- [x] 3.4 Add an "Open cache folder" `Button` that calls the new `open_cache_folder()` controller method on click.
- [x] 3.5 Add i18n keys (English, German, French — `crates/dtrpg-ui/i18n/{en,de,fr}.yaml`) for: the cache details section title, each of the 9 data point labels, each of the 9 data point descriptions, and the open-folder button, following the existing `t!("settings....")` pattern used elsewhere in this file.

## 4. Verification

- [x] 4.1 Manually verify: populate the cache (load the library, browse a few covers, sign in to populate avatar), open Advanced settings, confirm counts match reality.
- [x] 4.2 Manually verify: click "Clear cache", confirm counts drop to zero/empty on next render.
- [x] 4.3 Manually verify: click "Open cache folder", confirm the OS file manager opens at the app cache directory (test on at least one platform; verify the directory-creation fallback when cache is empty).
- [x] 4.4 Manually verify: all 6 timing values render with correct human-readable durations and each of the 9 data points (3 counts + 6 timings) shows both a label and a description.
- [x] 4.5 Run `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`, and `cargo test --all-features --workspace`.
