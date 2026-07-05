## 1. Data layer

- [ ] 1.1 Add `pub fn avatar_cached() -> bool` to `dtrpg-ui/src/data/avatar.rs` (wraps `avatar_cache_path().exists()`), avoiding duplication of the avatar cache path elsewhere.
- [ ] 1.2 Add a `CacheCounts { metadata_items: usize, cover_thumbnails: usize, avatar_cached: bool }` struct (or equivalent) in `dtrpg-ui/src/controllers/settings.rs` or a shared data module.

## 2. Controller

- [ ] 2.1 Add a `SettingsController` method (e.g. `cache_counts()`) that reads `load_cache_metadata(&cache_dir()).map(|m| m.item_count).unwrap_or(0)` for the metadata count, counts entries via `std::fs::read_dir(covers_dir())` for the cover count, and calls `avatar_cached()` for the avatar indicator.
- [ ] 2.2 Add a `SettingsController` method (e.g. `open_cache_folder()`) that creates `app_cache_dir()` if missing and calls `reveal_in_file_manager(&app_cache_dir())`, mirroring `reveal_storage_location`.
- [ ] 2.3 Extend `SettingsSnapshot` to include the computed `CacheCounts` (or expose it as a separate accessor read directly by the view) so the Advanced section can render it.

## 3. View

- [ ] 3.1 In `dtrpg-ui/src/ui/views/settings_advanced_view.rs`, add a "Cache details" subsection above or alongside "Clear cache" showing metadata item count, cover thumbnail count, and an avatar-cached indicator.
- [ ] 3.2 Add an "Open cache folder" `Button` that calls the new `open_cache_folder()` controller method on click.
- [ ] 3.3 Add i18n keys for the new labels (cache details title, per-type labels, open-folder button) following the existing `t!("settings....")` pattern used elsewhere in this file.

## 4. Verification

- [ ] 4.1 Manually verify: populate the cache (load the library, browse a few covers, sign in to populate avatar), open Advanced settings, confirm counts match reality.
- [ ] 4.2 Manually verify: click "Clear cache", confirm counts drop to zero/empty on next render.
- [ ] 4.3 Manually verify: click "Open cache folder", confirm the OS file manager opens at the app cache directory (test on at least one platform; verify the directory-creation fallback when cache is empty).
- [ ] 4.4 Run `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`, and `cargo test --all-features --workspace`.
