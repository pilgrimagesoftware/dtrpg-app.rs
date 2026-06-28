## 1. Revert previous partial fix

- [x] 1.1 In `catalog_view.rs`, in `render_list_row`, revert the `div().w(px(24.0)).flex().items_center().justify_center()` wrapper around `render_status` — restore the bare `.child(render_status(status, &colors))` call

## 2. Add shared column definitions

- [x] 2.1 Add `fn list_columns() -> Vec<Column>` to `catalog_view.rs`, returning 8 `Column` entries with the widths specified in the design (title 300px resizable, publisher 130px, system 110px, pages 60px, size 60px, added 80px, status 24px non-resizable non-selectable, reveal 28px non-resizable non-selectable)
- [x] 2.2 Add the necessary imports: `gpui_component::table::{Column, DataTable, TableDelegate, TableEvent, TableState}` and `gpui_component::{Size, Sizable, table::{TableHeader, TableBody, TableRow, TableHead, TableCell}}`

## 3. Implement CatalogListDelegate

- [x] 3.1 Add `struct CatalogListDelegate` with fields `controller: Entity<LibraryController>`, `storage_root: PathBuf`, `columns: Vec<Column>`
- [x] 3.2 Implement `TableDelegate` for `CatalogListDelegate`:
  - `columns_count()` → `self.columns.len()`
  - `column(ix)` → `self.columns[ix].clone()`
  - `rows_count()` → `self.controller.read(cx).visible_items().len()`
  - `render_td(row_ix, col_ix)` → match on col_ix: 0=title+kind cell, 1=publisher, 2=system/line, 3=pages, 4=size_mb, 5=year, 6=status (using `render_status`), 7=reveal button or empty div
  - `render_tr(row_ix)` → default (returns `div().id(("row", row_ix))`)

## 4. Wire CatalogListDelegate into CatalogView

- [x] 4.1 Add `catalog_list_table: Entity<TableState<CatalogListDelegate>>` field to `CatalogView`
- [x] 4.2 In `CatalogView::new()`, construct `CatalogListDelegate` and create `catalog_list_table` via `cx.new(|window, cx| TableState::new(delegate, window, cx).row_selectable(true).col_selectable(false).col_movable(false).col_resizable(false).sortable(false))`
- [x] 4.3 In `CatalogView::new()`, subscribe to `TableEvent` on `catalog_list_table`: on `SelectRow(row_ix)`, read `visible_items()[row_ix].id` from the controller and call `controller.select_item(id, cx)`

## 5. Replace ungrouped list render arm

- [x] 5.1 In `CatalogView::render()`, in the `(CatalogPresentation::List, false)` match arm, remove the `render_list_header` child and the `uniform_list("catalog-list", ...)` child
- [x] 5.2 Replace with `DataTable::new(&self.catalog_list_table).with_size(Size::Size(density.row_text_height)).bordered(false).scrollbar_visible(true, false)` as the single child of `root.px(pad_side)`

## 6. Replace grouped list render arm

- [x] 6.1 In the `(CatalogPresentation::List, true)` match arm, replace the `render_list_header` + `render_list_row` calls with raw flex div rows using widths from `list_columns()`:
  - `render_grouped_list_header(colors, cols)` for the header row
  - `render_grouped_list_row(item, colors, density, entity, storage_root, cols)` for each data row

## 7. Add render_list_table_cells helper

- [x] 7.1 Add `fn render_grouped_list_header(colors, cols)` and `fn render_grouped_list_row(item, colors, density, entity, storage_root, cols)` helpers — column widths from `list_columns()` shared between header and rows
- [x] 7.2 The reveal cell (col 7) includes the `on_click` handler for `reveal_in_file_manager` (same logic as before, moved into this function)

## 8. Delete obsolete functions

- [x] 8.1 Delete `fn render_list_header(...)` — replaced by DataTable header and `render_grouped_list_header`
- [x] 8.2 Delete `fn render_list_row(...)` — replaced by `render_td` in the delegate and `render_grouped_list_row`

## 9. Verification

- [x] 9.1 Run `cargo check --all-targets` and confirm no compile errors
- [x] 9.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings (pre-existing `library.rs:99` expect_used error is not introduced by this change)
- [ ] 9.3 Launch the app in ungrouped list view; confirm the Publisher, System, Pages, Size, and Added header labels align exactly with the data values in each row
- [ ] 9.4 Switch to grouped list view; confirm the same column alignment holds under each group header
- [ ] 9.5 Confirm clicking a row in ungrouped list view selects the item (detail panel updates)
- [ ] 9.6 Confirm clicking the reveal arrow (↗) for a downloaded item opens Finder/Explorer
