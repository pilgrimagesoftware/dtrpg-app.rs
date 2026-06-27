## 1. Extend ActivityController with Selection State

- [ ] 1.1 Add `selected_id: Option<u64>` field to `ActivityController` (default `None`)
- [ ] 1.2 Implement `ActivityController::select_activity(id: u64, cx: &mut Context<Self>)` — sets `selected_id` to `Some(id)` if it differs from current, or `None` if same id (toggle); emits `ActivityChanged`
- [ ] 1.3 Add `selected_id: Option<u64>` to `ActivitySnapshot` and populate it from `ActivityController::snapshot()`

## 2. Scrollable Item List

- [ ] 2.1 In `render_activity_panel`, replace `.overflow_y_hidden()` with `.overflow_y_scroll()` on the item list container

## 3. Hover Tooltips

- [ ] 3.1 Add a `use gpui::Tooltip;` import (or equivalent) to `activity_panel_view.rs`
- [ ] 3.2 In `render_item_row`, build a tooltip string: label only for `InProgress`/`Complete`, label + "\n" + error for `Error` items
- [ ] 3.3 Attach `.tooltip(move |window, cx| Tooltip::text(tooltip_text.clone(), window, cx))` to the item row `div`
- [ ] 3.4 Give each item row a unique element id (e.g. `format!("activity-row-{}", item.id)`) so tooltips can be anchored per-row

## 4. Click-to-Expand Item Rows

- [ ] 4.1 Pass `selected_id: Option<u64>` and `activity_entity: Entity<ActivityController>` into `render_item_row`
- [ ] 4.2 Add an `id: u64` parameter to `render_item_row`
- [ ] 4.3 Add `.on_click()` to the item row div that calls `activity_entity.update(cx, |a, cx| a.select_activity(id, cx))`
- [ ] 4.4 When `selected_id == Some(id)`, render the label div without `.truncate()` and with natural word wrap; for error items also render the full error message without `.truncate()`
- [ ] 4.5 When `selected_id != Some(id)`, render as before (truncated single line)
- [ ] 4.6 Update `render_activity_panel` to pass `snap.selected_id` and `entity.clone()` to each `render_item_row` call

## 5. Refined Empty State

- [ ] 5.1 Replace the minimal `render_empty` function body with a centered column containing a `text_2xl` icon ("○") and two text lines: "No recent activity." (primary, `text_sm`) and "Activity will appear here as operations run." (tertiary, `text_xs`)

## 6. Build and Quality

- [ ] 6.1 Run `cargo check --workspace` and fix any compilation errors
- [ ] 6.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any warnings
- [ ] 6.3 Run `cargo test --workspace` and confirm all tests pass

## 7. Manual Verification

- [ ] 7.1 Open the activity panel with several items and confirm the list scrolls when items exceed panel height
- [ ] 7.2 Hover over an item row with a long label and confirm the tooltip shows the full text
- [ ] 7.3 Hover over an error item row and confirm the tooltip shows both the full label and the error message
- [ ] 7.4 Click an item row and confirm it expands to show full word-wrapped text
- [ ] 7.5 Click the same row again and confirm it collapses back to truncated
- [ ] 7.6 Click one row then click a different row and confirm only the second is expanded
- [ ] 7.7 Open the panel with no activity items and confirm the prominent empty state is shown
