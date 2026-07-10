## 1. Helper

- [ ] 1.1 Add `crates/dtrpg-ui/src/util/file_size.rs` with `on_disk_file_size(entry_dir: &Path, file_name: &str) -> Option<u64>`, resolving `entry_dir.join(file_name)` and calling `std::fs::metadata`, returning `None` on any error
- [ ] 1.2 Add a `format_bytes(bytes: u64) -> String` helper (or reuse an existing formatter if one already exists in `crate::util`) producing the same `"{:.1} MB"`-style output as the current catalog-size formatting, so the displayed value doesn't visually jump between unit conventions
- [ ] 1.3 Unit tests: file present returns `Some(size)`, file missing returns `None`, empty/invalid `file_name` returns `None`
- [ ] 1.4 Register the new module in `crate::util`'s `mod.rs`

## 2. Single-file entry (top-level field)

- [ ] 2.1 In `detail_panel_view.rs`, at the `field_file_size` `DescriptionItem` (~line 771), resolve the on-disk size when `item.status == ItemStatus::Downloaded` and `item.files` has exactly one entry, falling back to `item.size_mb` otherwise

## 3. Multi-item entry file list

- [ ] 3.1 In `render_item_metadata`'s `field_file_size` `DescriptionItem` (~line 436), resolve the on-disk size for `file` using the entry's `storage_root_path`/`item.id`, falling back to `file.size_mb` when unresolved

## 4. Verification

- [ ] 4.1 `cargo build --workspace --all-features`
- [ ] 4.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 4.3 `cargo test --workspace --all-features`
- [ ] 4.4 Launch app: place a file manually at a downloaded item's resolved path and confirm the detail view shows its real size instead of the catalog size
