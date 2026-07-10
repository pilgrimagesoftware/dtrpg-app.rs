## 1. Sort model — util/sort.rs

- [x] 1.1 Add `SortDirection` enum (`Ascending`, `Descending`, `#[default] Ascending`) to `util/sort.rs`
- [x] 1.2 Add `SortMethod::Custom { col_key: &'static str }` variant
- [x] 1.3 Update `sort_items(items, method, direction: SortDirection)` signature; flip comparisons when `Descending`; add comparators for `Custom { col_key: "system" }` (by `line`), `Custom { col_key: "size" }` (by `size_mb`), `Custom { col_key: "added" }` (by `added_order`); remove the previously hardcoded descending direction on `PageCount`

## 2. Controller — library.rs

- [x] 2.1 Add `sort_direction: SortDirection` field to `LibraryController` internal state and `LibrarySnapshot`; default to `SortDirection::Ascending`
- [x] 2.2 Add `set_sort_direction(direction: SortDirection, cx)` mutation on `LibraryController`; set the field and emit `LibraryChanged`
- [x] 2.3 Update `sort_items` call-sites in `LibraryController` (visible_items, visible_items_slice, visible_items_count) to pass `self.sort_direction`
- [x] 2.4 Update `LibrarySnapshot::sort_direction` field to be included in `snapshot()` return value

## 3. Catalog view — column definitions and kind badge

- [x] 3.1 In `catalog_view.rs`, rename the first column in `list_columns()` from `"Title / Kind"` to `"Title"`; change its `resizable(true)` flag (already set); remove the `resizable(false)` flag on all other non-status/reveal data columns (Publisher, System, Pages, Size, Added)
- [x] 3.2 Add `fn kind_badge(kind: &str) -> &'static str` in `catalog_view.rs` that maps kind strings to 2–3 char abbreviations per the spec table
- [x] 3.3 In `CatalogListDelegate::render_td(col=0)`, replace the `div().text_xs().text_color(colors.text_tertiary).whitespace_nowrap().child(item.kind)` child with a styled badge div using `kind_badge(&item.kind)` — small text, rounded pill background, `flex_none`

## 4. Catalog view — delegate sort integration

- [x] 4.1 Update `CatalogListDelegate::column(col_ix, cx)` to read the controller snapshot and return the column with `sort` set to `ColumnSort::Ascending`, `ColumnSort::Descending`, or `ColumnSort::Default` based on whether this column matches the current sort method and direction
- [x] 4.2 Implement `CatalogListDelegate::perform_sort(col_ix, sort: ColumnSort, _window, cx)`:
  - Map `col_ix` → `SortMethod` (see design column mapping table)
  - Map `ColumnSort::Ascending` → `SortDirection::Ascending`, `ColumnSort::Descending` → `SortDirection::Descending`
  - When `ColumnSort::Default` (third click): set `SortMethod::Title` + `SortDirection::Ascending` (reset to default)
  - Call `controller.update(cx, |ctrl, cx| { ctrl.set_sort(method, cx); ctrl.set_sort_direction(direction, cx); })`
- [x] 4.3 Add `gpui_component::table::ColumnSort` to the import list in `catalog_view.rs`

## 5. Catalog view — TableState configuration and refresh subscription

- [x] 5.1 In `CatalogView::new()`, change `TableState` builder from `.sortable(false).col_resizable(false)` to `.sortable(true).col_resizable(true)`
- [x] 5.2 In `CatalogView::new()`, add a subscription to `LibraryChanged` that calls `this.catalog_list_table.update(cx, |state, cx| state.refresh(cx))` — this propagates toolbar sort changes to the column header indicators

## 6. Toolbar — sort selector dropdown

- [x] 6.1 In `toolbar_view.rs`, update `render_sort_selector` to receive `sort_direction: SortDirection` as a parameter (pass it from `render_toolbar`; read it from the controller snapshot in the root view)
- [x] 6.2 Add a "Custom" item: `PopupMenuItem::new("Custom").checked(matches!(current, SortMethod::Custom { .. })).disabled(true)` — shown as a read-only indicator; insert it after the existing four named items, above the direction section
- [x] 6.3 Add a separator + "Ascending" / "Descending" items:
  ```
  PopupMenuItem::separator()
  PopupMenuItem::new("Ascending").checked(direction == SortDirection::Ascending).on_click(...)
  PopupMenuItem::new("Descending").checked(direction == SortDirection::Descending).on_click(...)
  ```
  Where `on_click` calls `entity.update(cx, |ctrl, cx| ctrl.set_sort_direction(dir, cx))`
- [x] 6.4 Update the `render_toolbar` call site to pass `snap.sort_direction` to `render_sort_selector`; add `SortDirection` and `SortMethod::Custom` to imports in `toolbar_view.rs`

## 7. Grouped list view — kind badge consistency

- [x] 7.1 In `catalog_view.rs`, update `render_grouped_list_row` to use `kind_badge(&item.kind)` in the title cell badge (same div styling as the DataTable cell) instead of the inline kind text span

## 8. Verification

- [x] 8.1 Run `cargo check --all-targets` — no compile errors
- [x] 8.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings
- [x] 8.3 Launch app in list view; confirm column headers show the neutral sort icon (⇕); confirm dragging a column border resizes it
- [x] 8.4 Click the "Publisher" column header; confirm rows sort descending by publisher and the sort dropdown shows "Publisher" checked
- [x] 8.5 Click "Publisher" again; confirm ascending sort and the dropdown "Ascending" item is checked
- [x] 8.6 Click "Publisher" a third time; confirm sort resets to Title ascending
- [x] 8.7 Click the "System" column header; confirm the dropdown shows "Custom" as the active indicator
- [x] 8.8 Select "Descending" from the dropdown; confirm the active sort reverses; select "Ascending" and confirm it reverses again
- [ ] 8.9 Select "Pages" from the dropdown; confirm the Pages column header shows the active sort indicator and the Title column returns to neutral
- [ ] 8.10 Confirm each row's title cell shows a kind badge ("CR", "SUP", "ADV", etc.) and the badge does not wrap or displace the title
