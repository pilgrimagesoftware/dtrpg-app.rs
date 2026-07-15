## 1. LibraryController slice method

- [x] 1.1 Add `pub fn visible_items_count(&self) -> usize` to `LibraryController` that returns `self.visible_items().len()` — used by `CatalogView::render` to pass the item count to `uniform_list` without cloning the full Vec
- [x] 1.2 Add `pub fn visible_items_slice(&self, range: std::ops::Range<usize>) -> Vec<LibraryItem>` to `LibraryController` that returns `self.visible_items().get(range).map(|s| s.to_vec()).unwrap_or_default()`

## 2. CatalogView struct

- [x] 2.1 Define a `CatalogView` struct in `catalog_view.rs` with fields: `controller: Entity<LibraryController>`, `scroll_handle: gpui::UniformListScrollHandle`, and `last_grid_row_count: usize` (initialized to 0, used to detect item-count changes for grid)
- [x] 2.2 Implement `CatalogView::new(controller: Entity<LibraryController>) -> Self` initializing the struct
- [x] 2.3 Add `use gpui::{uniform_list, UniformListScrollHandle};` imports at the top of `catalog_view.rs`

## 3. List layout virtualization

- [x] 3.1 Move the list-row render logic from the existing `render_catalog` list arm into a free function `fn render_list_row(item: &LibraryItem, cx: &mut App) -> impl IntoElement` (or similar) so the `uniform_list` closure can call it
- [x] 3.2 Replace the list-layout `div()` + items loop in `render_catalog` with a `uniform_list("catalog-list", item_count, move |range, _window, cx| { ... })` that reads items via `cx.read_entity(&controller, |ctrl, _| ctrl.visible_items_slice(range))` and maps each to `render_list_row`
- [x] 3.3 Chain `.track_scroll(&scroll_handle)` and `.overflow_y_scrollbar()` on the `uniform_list` element (`.overflow_y_scrollbar()` from `gpui_component::scroll::ScrollableElement`)

## 4. Thumbs layout virtualization

- [x] 4.1 Move the thumb-card render logic into a free function `fn render_thumb_card(item: &LibraryItem, cx: &mut App) -> impl IntoElement`
- [x] 4.2 Replace the thumbs-layout loop with `uniform_list("catalog-thumbs", item_count, move |range, _window, cx| { ... })` mapping each item to `render_thumb_card`
- [x] 4.3 Chain `.track_scroll(&scroll_handle)` and `.overflow_y_scrollbar()` on the thumbs `uniform_list`

## 5. Grid layout virtualization

- [x] 5.1 Determine `items_per_row` using the existing column-count logic; compute `row_count = item_count.div_ceil(items_per_row)`
- [x] 5.2 Move the grid card render logic into `fn render_grid_card(item: &LibraryItem, cx: &mut App) -> impl IntoElement`
- [x] 5.3 Replace the grid loop with `uniform_list("catalog-grid", row_count, move |row_range, _window, cx| { ... })` where the closure maps each row index `r` to a horizontal `div` containing cards for items `(r * items_per_row)..((r + 1) * items_per_row).min(item_count)`; read items in bulk via `ctrl.visible_items_slice(row_range.start * items_per_row..row_range.end * items_per_row)`
- [x] 5.4 Chain `.track_scroll(&scroll_handle)` and `.overflow_y_scrollbar()` on the grid `uniform_list`

## 6. CatalogView Render impl

- [x] 6.1 Implement `gpui::Render for CatalogView` — in `render(&mut self, _window: &mut Window, cx: &mut App)`, read `item_count` from `self.controller.read(cx).visible_items_count()`, match on the current layout mode from the snapshot or controller, and delegate to the appropriate `uniform_list` builder
- [x] 6.2 Pass `&self.scroll_handle` into each `uniform_list` call via `.track_scroll()`; the handle is reused across renders

## 7. Wire CatalogView into the parent view

- [x] 7.1 In `LibraryRootView` (or `LibraryView`, wherever `render_catalog` is currently called): create a `CatalogView` entity with `cx.new(|_| CatalogView::new(library_controller.clone()))` during view construction; store the entity on the parent view struct
- [x] 7.2 In the parent view's `render`, replace the `render_catalog(...)` call with `self.catalog_view.clone()` (which GPUI renders automatically as a child entity) or call `cx.render(self.catalog_view.clone())`
- [x] 7.3 Remove the `render_catalog` free function (or mark it `#[cfg(test)]` if any test uses it, then delete after tests are updated)

## 8. Verify

- [x] 8.1 Run `cargo check --all-targets` and confirm no compile errors
- [x] 8.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any warnings
- [x] 8.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [x] 8.4 Manually launch the app, load a large catalog (500+ items), and confirm scrolling in list layout is smooth with no visible lag
- [x] 8.5 Switch to thumbs layout and confirm smooth scrolling
- [x] 8.6 Switch to grid layout and confirm smooth scrolling and correct card tiling
- [x] 8.7 Switch to grouped layout and confirm all items and group headers appear (non-virtualized fallback)
- [x] 8.8 Click an item in list layout and confirm detail view opens; right-click and confirm context menu appears
- [x] 8.9 Click an item in thumbs layout and confirm detail view opens; right-click and confirm context menu appears
