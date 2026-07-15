## Why

Downloaded files are organized on disk by publisher (`{root}/items/{publisher}/{filename}`, see `implement-real-file-downloads`), which does not reflect the user's own collections in the app. Users who group items into collections have no way to browse their downloads by collection on disk — they'd have to know which publisher folder a given item landed in. A configurable, symlink-based mirror under a `collections/` subpath lets users navigate their downloads the way they organize them in the app, without duplicating file content on disk.

## What Changes

- Add a "Create collections" storage setting (off by default) alongside the existing storage location controls in Settings.
- When enabled, completing a download for an item that belongs to one or more collections creates a symlink for that file under `{root}/collections/{collection name}/{filename}` for each collection the item belongs to, pointing at the real file under `{root}/items/{publisher}/{filename}`.
- Symlink creation uses the OS-native mechanism (`std::os::unix::fs::symlink` on macOS/Linux, `std::os::windows::fs::symlink_file` on Windows) and is best-effort: a failure to create a symlink is logged and does not fail the download or surface as a download error.
- Toggling the setting off does not retroactively remove existing symlinks; toggling it on does not retroactively create symlinks for already-downloaded items (only affects downloads completed while enabled — see design.md for alternatives considered).
- Adding an already-downloaded item to a collection (or a collection membership change) does not retroactively create/remove symlinks — this proposal only covers symlink creation at download-completion time.

## Capabilities

### New Capabilities

- `download-collections-symlinks`: a configurable, OS-native symlink mirror of downloaded files organized by the collections an item belongs to, created at download-completion time.

### Modified Capabilities

<!-- none -->

## Impact

- `crates/dtrpg-ui/src/data/storage.rs`: new `create_collections` boolean field on `StorageConfig` (persisted like `max_concurrent_downloads`), a new `collection_dir` path helper mirroring `publisher_dir`.
- `crates/dtrpg-ui/src/controllers/library.rs`: `dispatch_download`'s completion path gains a step that resolves the downloaded item's collection memberships and creates symlinks when the setting is enabled.
- `crates/dtrpg-ui/src/ui/views/settings_view.rs` (or equivalent storage settings view): a new toggle control for "Create collections".
- Platform-specific symlink creation requires a small OS-conditional helper (`#[cfg(unix)]` / `#[cfg(windows)]`), consistent with existing platform-conditional code in this crate (e.g. `reveal_in_file_manager`).
- No change to `remove_download` in this proposal — it does not delete real files today (see `confirm-download-removal`), so no symlink cleanup on removal is implemented here; stale symlinks after a manual file deletion are an accepted limitation (see design.md Risks).
