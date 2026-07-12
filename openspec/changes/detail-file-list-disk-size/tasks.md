## 1. Helper

- [x] 1.1 Add `crates/dtrpg-ui/src/util/file_size.rs` with `on_disk_file_size(entry_dir: &Path, file_name: &str) -> Option<u64>`, resolving `entry_dir.join(file_name)` and calling `std::fs::metadata`, returning `None` on any error
- [x] 1.2 Add a `format_bytes(bytes: u64) -> String` helper producing the same `"{:.1} MB"`-style output as the current catalog-size formatting, localized via `size.mb`
- [x] 1.3 Unit tests: file present returns `Some(size)`, file missing returns `None`, empty/invalid `file_name` returns `None`
- [x] 1.4 Register the new module in `crate::util`'s `mod.rs`
- [x] 1.5 Add a `on_disk_suffix(bytes: u64) -> String` (or equivalent) helper building the localized `"(X.X MB on disk)"` string via `t!("detail.on_disk_suffix", size = format_bytes(bytes))`, and a composer that joins a catalog-size string with an optional on-disk suffix

## 2. Single-file entry (top-level field)

- [x] 2.1 In `detail_panel_view.rs`, at the `field_file_size` `DescriptionItem`, resolve the on-disk size when `item.status == ItemStatus::Downloaded` and `item.files` has exactly one entry
- [x] 2.2 Change the single-file field to always show the catalog size, appending the on-disk suffix (from 1.5) only when resolvable, instead of replacing the catalog size

## 3. Multi-file entry (combined top-level field)

- [x] 3.1 When `item.files.len() > 1`, compute the top-level catalog figure as `item.files.iter().map(|f| f.size_mb).sum()` instead of `item.size_mb`
- [x] 3.2 Compute the combined on-disk figure as the sum of `on_disk_file_size(...)` over files that resolve (`Some`); omit the suffix entirely if none resolve
- [x] 3.3 Switch the field's label to `t!("detail.field_total_file_size")` when `item.files.len() > 1`, keeping `t!("detail.field_file_size")` otherwise
- [x] 3.4 Add `field_total_file_size` key ("Total file size") to `crates/dtrpg-ui/i18n/en.yaml`, `fr.yaml`, `de.yaml`

## 4. Multi-item entry file list (per-row field + Items table column)

- [x] 4.1 In `render_item_metadata`'s `field_file_size` `DescriptionItem`, resolve the on-disk size for `file` using the entry's `storage_root_path`/`item.publisher`
- [x] 4.2 Change the per-file field to always show the catalog size, appending the on-disk suffix only when resolvable
- [x] 4.3 In `render_item_tier`'s `header_row`, add a fourth `TableCell` labeled `t!("detail.item_list_column_size")`
- [x] 4.4 In `render_item_tier`'s per-row `row_content`, add a fourth `flex_1` child showing that row's file catalog size plus on-disk suffix, using the same composer from 1.5
- [x] 4.5 Add `item_list_column_size` key ("Size") to `crates/dtrpg-ui/i18n/en.yaml`, `fr.yaml`, `de.yaml`

## 5. Localization

- [x] 5.1 Add `on_disk_suffix` key (`"(%{size} on disk)"`) to `crates/dtrpg-ui/i18n/en.yaml`, `fr.yaml`, `de.yaml`

## 6. Verification

- [x] 6.1 `cargo build --workspace --all-features`
- [x] 6.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] 6.3 `cargo test --workspace --all-features`
- [ ] 6.4 Launch app: place files manually at a downloaded single-file and a downloaded multi-file item's resolved paths and confirm the detail view (top-level field, Items table Size column, selected-file panel) shows catalog size plus on-disk suffix, with the multi-file entry showing a combined total
