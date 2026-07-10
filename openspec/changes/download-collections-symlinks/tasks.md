## 1. Storage config

- [ ] 1.1 Add a `create_collections: bool` field to `StorageConfigFile`/`StorageConfig` in `crates/dtrpg-ui/src/data/storage.rs`, with `#[serde(default)]` so it defaults to `false` for existing config files, persisted through the existing `save()`/`load()` round-trip
- [ ] 1.2 Add `StorageConfig::create_collections(&self) -> bool` and `set_create_collections(&mut self, enabled: bool)` (saving on set, mirroring `set_max_concurrent_downloads`)
- [ ] 1.3 Add a `collection_dir(root: &Path, collection_name: &str) -> PathBuf` helper (mirrors `publisher_dir`: `root.join("collections").join(sanitize_path_component(collection_name))`); make `sanitize_path_component` non-private (or reuse via existing visibility) so this helper can share it
- [ ] 1.4 Unit tests: `create_collections` defaults to `false`, round-trips through save/load, `collection_dir` sanitizes a name containing a path separator and never escapes root

## 2. Symlink creation

- [ ] 2.1 Add a small platform-conditional helper module (e.g. `crates/dtrpg-ui/src/util/symlink.rs`) with `#[cfg(unix)]`/`#[cfg(windows)]` implementations of `create_symlink(target: &Path, link: &Path) -> std::io::Result<()>`, using `std::os::unix::fs::symlink` and `std::os::windows::fs::symlink_file` respectively
- [ ] 2.2 Before calling the platform symlink function, check `link.symlink_metadata().is_ok()` and skip (no-op, no error) if the link path already exists
- [ ] 2.3 On symlink creation failure, log via `tracing::warn!` including the target and link paths; never propagate the error to the caller

## 3. Wire into the download completion path

- [ ] 3.1 In `LibraryController::dispatch_download`'s success branch (`crates/dtrpg-ui/src/controllers/library.rs`), after a successful `download_item` result and before/alongside the item status update, resolve which of `self.collections` the item belongs to using `crate::util::matching::{collection_member_id, member_ids_contain}`
- [ ] 3.2 If `StorageConfig::load().create_collections()` is `true`, for each matching collection create a symlink at `collection_dir(&root, &collection.name).join(file.name.as_ref())` pointing at the already-resolved download destination path
- [ ] 3.3 Ensure this step runs only on successful, non-cancelled downloads (same branch that currently marks the item `Downloaded`)

## 4. Settings UI

- [ ] 4.1 Add a "Create collections" toggle control to `crates/dtrpg-ui/src/ui/views/settings_storage_view.rs`, alongside the existing storage location/concurrency controls, wired to `StorageConfig::create_collections()`/`set_create_collections`
- [ ] 4.2 Add translation entries for the toggle's label (and tooltip, if one documents the Windows symlink-privilege limitation from design.md)

## 5. Verification

- [ ] 5.1 `cargo build --workspace --all-features`
- [ ] 5.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 5.3 `cargo test --workspace --all-features`
- [ ] 5.4 Launch app: enable "Create collections", download an item that belongs to a collection, confirm a symlink appears under `{root}/collections/{collection name}/` and resolves to the real file
- [ ] 5.5 Launch app: download an item that belongs to no collection with the setting enabled, confirm no `collections/` entry is created for it
