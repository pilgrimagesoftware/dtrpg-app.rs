## 1. Panel width state in LibraryController

- [ ] 1.1 Add `detail_panel_width: f32` field to `LibraryController`; initialize to `320.0`
- [ ] 1.2 Add `pub fn detail_panel_width(&self) -> f32` getter
- [ ] 1.3 Add `pub fn set_detail_panel_width(&mut self, width: f32, cx: &mut Context<Self>)` that clamps `width` to `240.0..=600.0` and emits `LibraryChanged`
- [ ] 1.4 Add `detail_panel_width: f32` field to `LibrarySnapshot` and populate it in `LibraryController::snapshot()`

## 2. Scrollable body fix

- [ ] 2.1 Add `use gpui_component::scroll::ScrollableElement;` to the imports in `detail_panel_view.rs`
- [ ] 2.2 Change `.overflow_y_hidden()` on the panel body div to `.overflow_y_scrollbar()`

## 3. Text wrapping fixes

- [ ] 3.1 Verify that the title div and description div have no `.truncate()` or `.whitespace_nowrap()` calls; remove any that are present
- [ ] 3.2 In the metadata table (`render_metadata_table`), add `.min_w_0()` to the value div in each row so the flex layout allows the value cell to shrink and wrap; check if GPUI provides `.text_wrap()` or similar and apply it to the value div if available — otherwise restructure each row to stack label above value (label on top, value below) so wrapping occurs naturally
- [ ] 3.3 Confirm that the publisher, title, and line divs in the identity section at the top of the body do not have width-constraining styles that would prevent wrapping (e.g., `whitespace_nowrap`)

## 4. Drag handle

- [ ] 4.1 Inspect the GPUI source at `.cargo/git/checkouts/zed-*/crates/gpui/src/elements/div.rs` (search for `on_drag`, `DragMoveEvent`, `on_mouse_down`, `on_mouse_move`) to determine the correct API for a drag-resize handle at the pinned commit; document which approach is available
- [ ] 4.2 Add a `DragResizeState` or equivalent struct (if needed for the on_drag value type) to hold the panel width at drag-start; alternatively use a simpler `on_mouse_down` / `on_mouse_move` approach if `on_drag` is not suitable
- [ ] 4.3 In `render_detail_panel`, add a drag handle div: `absolute().left_0().top_0().bottom_0().w(px(6.0)).cursor_col_resize()` with hover visual feedback (a subtle highlight on the 6 px strip)
- [ ] 4.4 Wire the drag/mouse events on the handle div so that dragging left/right calls `entity.update(cx, |ctrl, cx| ctrl.set_detail_panel_width(new_width, cx))` where `new_width` is computed from the mouse delta relative to the drag-start x-position
- [ ] 4.5 Add `use gpui::{MouseButton, ...}` and any other imports needed for the drag events

## 5. Accept width parameter in render_detail_panel

- [ ] 5.1 Add a `width: f32` parameter to `render_detail_panel`
- [ ] 5.2 Replace `.w(px(320.0))` with `.w(px(width))` on the outer panel div
- [ ] 5.3 Update the cover image width variable from `let cover_w = 320.0_f32;` to `let cover_w = width;`
- [ ] 5.4 In `root_view.rs`, update the call to `render_detail_panel` to pass `snap.detail_panel_width` as the `width` argument

## 6. Verify

- [ ] 6.1 Run `cargo check --all-targets` and confirm no compile errors
- [ ] 6.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any warnings
- [ ] 6.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 6.4 Manually launch the app, select an item, and confirm the drag handle appears on the left edge of the detail panel; drag it left and right and confirm the panel resizes smoothly between 240 px and 600 px
- [ ] 6.5 Confirm the panel width is preserved after selecting a different catalog item
- [ ] 6.6 Select an item with a long title or description and confirm text wraps rather than overflowing or being cut off
- [ ] 6.7 Scroll the panel body and confirm content beyond the visible height is reachable (title, description, actions, metadata table all accessible)
- [ ] 6.8 Select an item with a long metadata value (e.g., a long publisher name) and confirm the value wraps within the right column of the metadata table rather than overflowing
