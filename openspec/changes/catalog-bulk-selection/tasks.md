## 1. Controller: Selection State

- [ ] 1.1 Add `selection_mode: bool` (default `false`) and `selected_ids: HashSet<Arc<str>>` (default empty) fields to `LibraryController`
- [ ] 1.2 Add `SelectionChanged` event struct to `data/events.rs` and implement `EventEmitter<SelectionChanged>` for `LibraryController`
- [ ] 1.3 Add `toggle_selection_mode(&mut self, cx)` — flips `selection_mode`, clears `selected_ids` when turning off, emits `SelectionChanged`
- [ ] 1.4 Add `select_item(id: &str, cx)` — inserts into `selected_ids`, emits `SelectionChanged`
- [ ] 1.5 Add `deselect_item(id: &str, cx)` — removes from `selected_ids`, emits `SelectionChanged`
- [ ] 1.6 Add `select_all(cx)` — inserts all IDs from `visible_items()` current page into `selected_ids`, emits `SelectionChanged`
- [ ] 1.7 Add `deselect_all(cx)` — clears `selected_ids`, emits `SelectionChanged`
- [ ] 1.8 Add `select_by_pattern(pattern: &str, field: SelectionField, cx)` — adds IDs of visible-page items whose `field` contains `pattern` (case-insensitive); no-op on empty pattern; emits `SelectionChanged`; `SelectionField` is a new enum `{ Title, Publisher, System }` in `data/enums.rs`
- [ ] 1.9 Add `selection_count() -> usize` — returns `selected_ids.len()`
- [ ] 1.10 Add `is_selected(id: &str) -> bool` accessor
- [ ] 1.11 Add `clear_selection_and_exit_mode(cx)` helper used by all bulk-action dispatch paths — clears `selected_ids`, sets `selection_mode = false`, emits `SelectionChanged`

## 2. Snapshot Propagation

- [ ] 2.1 Add `selection_mode: bool`, `selected_ids: HashSet<Arc<str>>`, and `selection_count: usize` fields to `LibrarySnapshot`
- [ ] 2.2 Populate these fields in `LibraryController::snapshot()`
- [ ] 2.3 Subscribe to `SelectionChanged` in `LibraryRootView::new` the same way `LibraryChanged` is subscribed — call `cx.notify()` on the view

## 3. Controller: Bulk Actions

- [ ] 3.1 Add `bulk_download(cx)` — for each ID in `selected_ids` where the item is not locally downloaded, call `download_item(id, cx)` (existing method or equivalent); call `clear_selection_and_exit_mode(cx)` when done
- [ ] 3.2 Add `bulk_remove_download(cx)` — for each ID in `selected_ids` where the item is locally downloaded, call `std::fs::remove_dir_all` on `StorageConfig::load().path_for_item(id)`; update item status; call `clear_selection_and_exit_mode(cx)`
- [ ] 3.3 Add `bulk_fetch_thumbnail(cx)` — enqueue each selected item ID into the thumbnail queue (same as the existing single-item thumbnail path); call `clear_selection_and_exit_mode(cx)`
- [ ] 3.4 Add `bulk_add_to_collection(collection_id: &str, cx)` — add each selected item to the given collection; call `clear_selection_and_exit_mode(cx)`
- [ ] 3.5 Add `bulk_remove_from_collection(collection_id: &str, cx)` — remove each selected item from the given collection if it belongs to it; call `clear_selection_and_exit_mode(cx)`
- [ ] 3.6 Add `bulk_open(cx)` — for each selected item that has a local download, call `reveal_in_file_manager` on its path; call `clear_selection_and_exit_mode(cx)`

## 4. Toolbar: Selection Mode Toggle

- [ ] 4.1 In `toolbar_view.rs`, add a `Button::new("selection-mode-toggle").ghost()` with icon `IconName::CheckSquare` (or equivalent) and tooltip "Select items"; when `selection_mode` is active, render with `.selected(true)` or an equivalent active variant
- [ ] 4.2 Wire the toggle button's `on_click` to `lib_entity.update(cx, |ctrl, cx| ctrl.toggle_selection_mode(cx))`
- [ ] 4.3 Pass `selection_mode` and `selection_count` into `render_toolbar` from `LibraryRootView::render`; update the function signature accordingly

## 5. Toolbar: Bulk-Action Bar

- [ ] 5.1 Create `crates/dtrpg-ui/src/ui/views/bulk_action_bar_view.rs` with a `render_bulk_action_bar` function; add it to the `views` module
- [ ] 5.2 Add an `InputState` entity for the pattern-match text field to `LibraryRootView`; wire its `InputEvent::Change` to a new field `ctrl.set_pattern_draft(value, cx)` (or store the draft directly in the view and pass it to the bar renderer)
- [ ] 5.3 In `render_bulk_action_bar`, render a horizontal flex row containing: selection-count label, Select All button, Deselect All button, pattern-match `Input` + field `Button::dropdown_menu` (Title / Publisher / System) + "Select Matches" button
- [ ] 5.4 In the same bar row, render the six bulk-action buttons: "Download", "Remove Download", "Fetch Thumbnail", "Add to Collection" (dropdown_menu listing all collections), "Remove from Collection" (dropdown_menu listing collections containing a selected item), "Open"
- [ ] 5.5 Wire each button's `on_click` to the corresponding `LibraryController` bulk method
- [ ] 5.6 Wire "Add to Collection" dropdown items to `bulk_add_to_collection(collection_id, cx)`; each `PopupMenuItem` is one collection from `snap.collections`
- [ ] 5.7 Wire "Remove from Collection" dropdown items to `bulk_remove_from_collection(collection_id, cx)`; filter to collections that intersect `snap.selected_ids`
- [ ] 5.8 In `LibraryRootView::render`, include `render_bulk_action_bar(...)` as a conditional child of the toolbar column when `snap.selection_mode && snap.selection_count > 0`

## 6. Catalog Views: Per-Item Checkboxes

- [ ] 6.1 In `catalog_view.rs` `render_thumb_row`, when `snap.selection_mode` is true, overlay a `Checkbox` (or styled `Button`) in the top-left corner of each thumb card; checked state is `snap.selected_ids.contains(&item.id)`; `on_click` calls `select_item` or `deselect_item`
- [ ] 6.2 In `catalog_view.rs` `render_grid_card`, apply the same checkbox overlay as thumb view
- [ ] 6.3 In `catalog_view.rs`, for the ungrouped list view (DataTable), add a leading checkbox column; when `snap.selection_mode` is false, the column width is 0 (hidden); when true, it is 28px and renders a checkbox per row
- [ ] 6.4 Verify that the grouped list view does NOT render checkboxes (excluded per design decision)

## 7. Build and Lint

- [ ] 7.1 Run `cargo check --workspace` — no errors
- [ ] 7.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings

## 8. Manual Verification

- [ ] 8.1 Selection mode toggle activates and deactivates correctly; checkboxes appear/disappear in all three views
- [ ] 8.2 Individual item checkboxes toggle selection state and count updates
- [ ] 8.3 Select All selects all visible items on current page; Deselect All clears them
- [ ] 8.4 Pattern-match control adds matching items to selection for each of the three fields
- [ ] 8.5 Bulk Download queues items via activity panel and clears selection
- [ ] 8.6 Bulk Remove Download removes local directories and clears selection
- [ ] 8.7 Bulk Fetch Thumbnail enqueues items and clears selection
- [ ] 8.8 Add to Collection dropdown shows all collections; selecting one assigns items
- [ ] 8.9 Remove from Collection dropdown shows only relevant collections; selecting one removes items
- [ ] 8.10 Bulk Open opens downloaded files; cloud-only items are skipped
- [ ] 8.11 Bulk-action bar is hidden when selection is empty; appears on first selection
