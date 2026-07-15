## 1. Metadata Table Reorder

- [x] 1.1 In `detail_panel_view.rs`, change `render_metadata_table`'s `DescriptionList` to
      `.columns(2)` and reorder items: system + released on one row, format + file size on
      the next row
- [x] 1.2 Move the category row to last, prefixing its value with `IconName::Folder`
- [x] 1.3 Keep the conditional `pages` and `date_added` rows appended after category,
      unchanged in content

## 2. Cover Overflow Containment

- [x] 2.1 Add `.min_w_0()` to `render_detail_tab_content`'s outer `.flex_row()` container

## 3. Build and Quality

- [x] 3.1 `cargo check --workspace`
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [x] 4.1 Open a detail tab and confirm system/released share a row, format/file-size share
      a row, and category is last with a folder icon
- [x] 4.2 Resize the sidebar/tab narrower and confirm the cover no longer paints past the
      tab content boundary into the sidebar
