## 1. `ItemListDelegate` and columns

- [x] 1.1 In `detail_panel_view.rs`, define `item_list_columns() -> Vec<Column>` with Name (resizable, majority default width), Type (resizable, narrow default), Status (resizable, narrow default)
- [x] 1.2 Define `struct ItemListDelegate { controller: Entity<LibraryController>, entry_id: Arc<str>, columns: Vec<Column>, user_widths: Vec<Option<Pixels>> }`
- [x] 1.3 Implement `TableDelegate for ItemListDelegate`: `columns_count`, `rows_count` (reads `controller.read(cx).item_by_id(&entry_id).files.len()`), `column` (applies `user_widths` override, mirroring `CatalogListDelegate::column`), `render_th`, `render_td` (renders Name/Type/Status cells for `item.files[row_ix]`, styled to match the current cell text — `text_sm`, `text_primary`/`text_secondary`)
- [x] 1.4 Do not implement `perform_sort` / leave `sortable(false)` — sorting is out of scope

## 2. Cache and wiring in `TabsController`

- [x] 2.1 Add `item_list_tables: HashMap<Arc<str>, Entity<TableState<ItemListDelegate>>>` field to `TabsController`
- [x] 2.2 In `TabsController::close_detail_tab`, remove the corresponding entry from `item_list_tables`
- [x] 2.3 Add a helper (e.g. `fn item_list_table(tabs: &Entity<TabsController>, controller: &Entity<LibraryController>, entry_id: &Arc<str>, window: &mut Window, cx: &mut App) -> Entity<TableState<ItemListDelegate>>`) that looks up the cache, or creates-and-inserts a new `TableState` (`row_selectable(true)`, `col_resizable(true)`, `sortable(false)`) plus its `TableEvent` subscription on cache miss
- [x] 2.4 In the subscription set up by the helper, handle `TableEvent::SelectRow(row_ix)` by calling `controller.update(cx, |ctrl, cx| ctrl.select_item_file(Arc::clone(entry_id), *row_ix, cx))`
- [x] 2.5 On cache-miss creation, if `controller.read(cx).selected_item_file(entry_id)` is already `Some(ix)`, call `state.set_selected_row(ix, cx)` once so a reopened tab shows the right row highlighted

## 3. Rewire `render_item_tier` / `render_detail_tab_content`

- [x] 3.1 Change `render_item_tier` signature to accept `tabs: &Entity<TabsController>, window: &mut Window, cx: &mut App` (replacing `cx: &App`); fetch/create the cached `TableState` entity via the Section 2 helper
- [x] 3.2 Replace the `Table`/`TableHeader`/`TableRow`/`TableCell` construction with `DataTable::new(&item_list_table)`
- [x] 3.3 Delete the per-row wrapping-`div`-with-`col_span(2)` click workaround (`detail_panel_view.rs:321-373`) — row click/selection is now handled by `TableState::row_selectable(true)` and the `TableEvent::SelectRow` subscription
- [x] 3.4 Update `render_detail_tab_content`'s signature the same way (`cx: &App` → `window: &mut Window, cx: &mut App`, plus a `tabs: &Entity<TabsController>` parameter) and thread the new parameters through to `render_item_tier`
- [x] 3.5 Update the call site in `root_view.rs` (`render_detail_tab_content(&item, ...)`) to pass `window`, `cx` (mutable), and `self.tabs.clone()` (or equivalent already-available field)

## 4. Verification

- [x] 4.1 Run `cargo check --all-targets` and `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 4.2 Launch the app; open a multi-item entry's detail tab; confirm the item list renders with Name/Type/Status columns
- [x] 4.3 Drag a column divider; confirm the column resizes and neighboring columns reflow
- [x] 4.4 Click a row; confirm it highlights and the item metadata area below updates to that file
- [x] 4.5 Resize a column, close the tab, reopen it for the same entry; confirm columns reset to default widths and the previously selected row (if any) is re-highlighted
- [x] 4.6 Open detail tabs for two different multi-item entries; confirm each has independent column widths and selection (no cross-entry state bleed)
