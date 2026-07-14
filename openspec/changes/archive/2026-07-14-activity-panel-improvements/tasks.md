## 1. Extend ActivityController with Selection State

- [x] 1.1 Add `selected_id: Option<u64>` field to `ActivityController` (default `None`)
- [x] 1.2 Implement `ActivityController::select_activity(id: u64, cx: &mut Context<Self>)` — sets `selected_id` to `Some(id)` if it differs from current, or `None` if same id (toggle); emits `ActivityChanged`
- [x] 1.3 Add `selected_id: Option<u64>` to `ActivitySnapshot` and populate it from `ActivityController::snapshot()`

## 2. Scrollable Item List

- [x] 2.1 In `render_activity_panel`, replaced `.overflow_y_hidden()` with `.overflow_y_scrollbar()` (requires `gpui_component::scroll::ScrollableElement` trait import)

## 3. Hover Tooltips

- [x] 3.1 Added `use gpui_component::scroll::ScrollableElement` and `use gpui_component::tooltip::Tooltip` imports to `activity_panel_view.rs`
- [x] 3.2 In `render_item_row`, builds tooltip string: label only for `InProgress`/`Complete`, label + "\n" + error for `Error` items
- [x] 3.3 Attached `.tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))` to the item row div
- [x] 3.4 Each item row has `.id(format!("activity-row-{item_id}"))` for tooltip anchoring

## 4. Click-to-Expand Item Rows

- [x] 4.1 Pass `selected_id: Option<u64>` and `activity_entity: Entity<ActivityController>` into `render_item_row`
- [x] 4.2 Add `item_id: u64` parameter to `render_item_row`
- [x] 4.3 Added `.on_click()` to the item row div that calls `activity_entity.update(cx, |a, cx| a.select_activity(item_id, cx))`
- [x] 4.4 When `selected_id == Some(item_id)`, render the label div without `.truncate()` and full error message without `.truncate()`
- [x] 4.5 When `selected_id != Some(item_id)`, render as before (truncated single line)
- [x] 4.6 Updated `render_activity_panel` to pass `snap.selected_id` and `entity.clone()` to each `render_item_row` call

## 5. Refined Empty State

- [x] 5.1 Replaced minimal `render_empty` with a centered column: "○" icon (`text_2xl`), "No recent activity." (primary, `text_sm`), "Activity will appear here as operations run." (tertiary, `text_xs`)

## 6. Build and Quality

- [x] 6.1 Run `cargo check --workspace` — zero errors
- [x] 6.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — zero warnings
- [x] 6.3 Run `cargo test --workspace` — 33 tests pass

## 7. Manual Verification

- [x] 7.1 Open the activity panel with several items and confirm the list scrolls when items exceed panel height
- [x] 7.2 Hover over an item row with a long label and confirm the tooltip shows the full text
- [x] 7.3 Hover over an error item row and confirm the tooltip shows both the full label and the error message
- [x] 7.4 Click an item row and confirm it expands to show full word-wrapped text
- [x] 7.5 Click the same row again and confirm it collapses back to truncated
- [x] 7.6 Click one row then click a different row and confirm only the second is expanded
- [x] 7.7 Open the panel with no activity items and confirm the prominent empty state is shown
