## Context

`render_activity_panel` in `activity_panel_view.rs` renders an `ActivitySnapshot`. The item list uses `.max_h(px(300.0)).overflow_y_hidden()`, clipping overflow without scroll. All item text uses `.truncate()`. The `ActivityController` has no selection state. The empty state exists but is a single small line of tertiary-colored text.

## Goals / Non-Goals

**Goals:**

- Make all items reachable via scroll when the list exceeds panel height.
- Make full item text accessible without resizing the panel.
- Let users pin an item's detail by clicking it.
- Give the empty state a visual weight proportional to the catalog empty states.

**Non-Goals:**

- Persistent expansion state across panel open/close cycles.
- Multi-item expansion (only one item expanded at a time).
- Animated expand/collapse.

## Decisions

### 1. Scrollable list via `overflow_y_scroll()`

Replace `.max_h(px(300.0)).overflow_y_hidden()` with `.max_h(px(300.0)).overflow_y_scroll()`. gpui's `overflow_y_scroll` renders a native scroll region. No additional wrapper needed.

### 2. Hover tooltip using gpui's `.tooltip()` builder

gpui provides `.tooltip(move |window, cx| Tooltip::text(text, window, cx))` on any element. Each item row gets a tooltip containing the full label and (for error items) the error message joined with a newline. This is zero additional state — purely a render-side concern.

*Alternative considered*: Show the full text on hover via a `.hover(|s| s)` style change that removes truncation. Rejected: gpui's `hover` modifier applies CSS-like property overrides but cannot toggle `text-overflow`; the approach would require two sibling elements (one truncated, one full) with visibility toggled — more complex for no benefit over a tooltip.

### 3. Click-to-expand via `ActivityController::selected_id`

Add `selected_id: Option<u64>` to `ActivityController`. A new `select_activity(id: u64, cx)` method sets it (or clears it if the same id is clicked again — toggle). `deselect_activity(cx)` clears it unconditionally. `ActivitySnapshot` gains `selected_id: Option<u64>`.

When `snap.selected_id == Some(item.id)`, the row renders without `.truncate()` and with `.whitespace_normal()` so text wraps. The expanded row also shows the full error message without truncation. A second click on the same row clears the selection and re-truncates.

`select_activity` emits `ActivityChanged` so the root view re-renders.

*Alternative considered*: Store selection in local render state inside `activity_panel_view.rs`. Rejected: gpui view state requires an entity; `render_activity_panel` is a free function, so controller state is the natural home.

### 4. Empty state refinement

Replace the single `text_xs` line with a centered column containing a larger icon (e.g. "○" or "✓" at `text_2xl`) and two lines: "No recent activity." (primary) and "Activity will appear here as operations run." (tertiary/xs). Matches the visual weight of `render_empty_state` in `catalog_view.rs`.

## Risks / Trade-offs

- **Tooltip text duplication**: The tooltip repeats text that is visible (truncated) in the row. This is standard desktop UX and acceptable.
- **`selected_id` persists across item resolution**: If an in-progress item is selected and it resolves, the item moves from `in_progress` to `recent` but keeps the same id. The `selected_id` remains valid and the expanded view will show the now-completed item — this is correct behavior.
- **`select_activity` adds one more method to `ActivityController`**: The controller is small; this is not a concern.
