## 1. Key bindings

- [x] 1.1 In `crates/dtrpg-ui/src/ui/app/mod.rs`, add `KeyBinding::new("cmd-shift-n", AddCollection, None)` to the `cx.bind_keys([...])` call in `setup()`
- [x] 1.2 Add `KeyBinding::new("cmd-r", ReloadCatalog, None)` to the same call
- [x] 1.3 Add `KeyBinding::new("cmd-shift-r", RefreshThumbnails, None)` to the same call
- [x] 1.4 Confirm `AddCollection`, `ReloadCatalog`, `RefreshThumbnails` are already imported in `ui/app/mod.rs` (they're referenced by `build_menus`); add imports if missing

## 2. Verify no conflicts

- [x] 2.1 Grep `crates/dtrpg-ui/src/ui/app/mod.rs` for any existing `cmd-shift-n`, `cmd-r`, or `cmd-shift-r` bindings to confirm no collision before adding the new ones
- [x] 2.2 `cargo build --workspace --all-features`
- [x] 2.3 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] 2.4 `cargo test --workspace --all-features`
- [x] 2.5 `cargo fmt --all -- --check`

## 3. Manual verification

- [ ] 3.1 Launch the app and confirm the Catalog menu shows `⇧⌘N`, `⌘R`, and `⇧⌘R` next to Add Collection, Reload, and Refresh Thumbnails respectively
- [ ] 3.2 With the library window focused, press `cmd-shift-n` and confirm the sidebar's new-collection input appears
- [ ] 3.3 Press `cmd-r` and confirm the catalog loading indicator appears and a live fetch runs
- [ ] 3.4 Press `cmd-shift-r` and confirm the "Loading thumbnails…" activity item appears and thumbnails re-fetch
