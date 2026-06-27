## Context

The detail panel is rendered by `render_detail_panel`, a free function that hard-codes `w(px(320.0))` and positions the panel absolutely (`absolute().right_0().top_0().bottom_0()`). It is added as the third flex child of the root `div` in `LibraryRootView::render`, where the root div uses `relative()` positioning — so the absolute panel sits over the catalog content on the right side.

The panel body's scrollable region uses `.overflow_y_hidden()`, meaning any content taller than the remaining viewport height after the cover image is silently clipped. Text elements have no wrapping issues from explicit `.truncate()` calls, but wrapping may not take effect if the GPUI layout engine sees the text's containing block as unconstrained in width.

`LibraryController` already owns `selected_item`; adding `detail_panel_width` there keeps all panel-related state in one controller and surfaces it through the existing `LibrarySnapshot` → `render_detail_panel` call chain.

## Goals / Non-Goals

**Goals:**
- Allow the user to drag the left edge of the panel to resize it between 240 px and 600 px.
- Preserve the chosen width across item selections and re-renders within a session.
- Scroll (not clip) panel content that exceeds the visible height.
- Wrap long text (title, description, metadata values) within the panel's current width.

**Non-Goals:**
- Persisting the panel width to disk across app restarts.
- Snapping to preset widths or keyboard shortcuts for width changes.
- Making the panel collapsible via the drag handle (closing is still via the ✕ button).
- Changing the panel from absolutely-positioned to a flex sibling that pushes the catalog.

## Decisions

### Width state lives in LibraryController

`LibraryController` gains `detail_panel_width: f32` (default 320.0). `set_detail_panel_width(width: f32, cx)` clamps to `[240.0, 600.0]` and emits `LibraryChanged`. `LibrarySnapshot` gains `detail_panel_width: f32`.

**Why not LibraryRootView**: `LibraryRootView` implements `Render` and the drag handler closure must capture an `Entity<LibraryController>` to call back into it — the same pattern already used for `clear_selection`. Storing width there is consistent with how other interactive panel state (filter, sort, selection) is handled.

**Why not persist to disk**: The width preference is low-stakes and the profile config feature is not yet wired. Adding disk persistence can be a follow-on.

### Drag handle: left-edge div with on_drag

A 6 px wide div is placed at the left edge of the panel (absolutely positioned: `absolute().left_0().top_0().bottom_0().w(px(6.0))`). It uses:
- `cursor_col_resize()` to show the resize cursor on hover
- `on_drag((), move |_drag, window, cx| { ... })` — GPUI's `on_drag` fires a closure on each mouse-move during a drag

Inside the `on_drag` closure, the mouse position delta relative to the drag start is used to compute the new panel width. GPUI drag events on an element provide the current `MouseMoveEvent` position. The drag start x-position can be captured via `on_mouse_down`:

```rust
div()
    // ... sizing/positioning ...
    .cursor_col_resize()
    .on_drag(DragState::default(), |drag_state, window, cx| {
        // delta = drag_state.start_x - window.mouse_position().x
        // new_width = original_width + delta
        // entity.update(cx, |ctrl, cx| ctrl.set_detail_panel_width(new_width, cx));
    })
```

**Implementation note**: Check the exact GPUI drag API at the pinned Zed commit (`crates/gpui/src/elements/div.rs` — `on_drag`, `DragMoveEvent`, `MouseDownEvent`). In recent GPUI, the pattern is:

```rust
.on_mouse_down(MouseButton::Left, move |event, _window, cx| {
    // record start state via cx.set_global or similar
})
.on_mouse_move(move |event, _window, cx| {
    // if dragging, compute delta and update width
})
.on_mouse_up(MouseButton::Left, move |_, _, cx| {
    // clear drag state
})
```

An alternative — if GPUI's higher-level `on_drag` is available and clean — is to use it instead. Either approach is acceptable; the task for implementation should verify the API before committing.

**Why a left-edge drag handle rather than a toggle button**: A drag handle provides continuous, proportional control and is the expected interaction for resizable panels. It is also the pattern Zed uses for its panels, which this app already follows in other respects.

### Scrollable body: overflow_y_scrollbar

`.overflow_y_hidden()` on the panel body div becomes `.overflow_y_scrollbar()` from `gpui_component::scroll::ScrollableElement`. This is the same fix applied in other views (catalog, sidebar) — the import and usage pattern already exists in `activity_panel_view.rs`.

### Text wrapping

In GPUI, a text node inside a flex column with a constrained parent width wraps by default when the parent's width is known at layout time. The panel's outer div has an explicit width (`w(px(width))`), so children in the column should wrap without additional attributes.

However, the metadata table uses `justify_between` on each row, which places the label on the left and value on the right in a single line. Long values will not wrap in this layout because the row is a single flex row. Fix: change the value div to have `flex_shrink()` and allow text wrapping, or switch to a two-row layout (label above, value below) for the metadata table.

Preferred fix: add `.min_w_0()` and `.flex_wrap()` (or check if GPUI has `.text_wrap()`) to the value div in each metadata row. If the GPUI API does not expose explicit text wrapping, switching to a stacked label/value layout is the fallback.

## Risks / Trade-offs

- [Risk] GPUI's drag event API may differ from the pattern described above. → Mitigation: task 3.4 explicitly requires checking the pinned Zed source before writing drag code; the fallback is mouse_down/mouse_move/mouse_up on the handle div.
- [Risk] The absolute-positioned panel at 320 px currently overlaps the catalog by 320 px without the catalog knowing. Making it wider (up to 600 px) increases this overlap. Users may want to see the catalog fully when the panel is wide. → Accepted trade-off for this change; moving the panel to be a flex sibling that shrinks the catalog is a separate, more invasive layout change.
- [Risk] Storing drag start position requires some temporary state. Using `cx.set_global` for drag state is one approach; alternatively `LibraryController` can hold an `Option<f32>` drag start x-position. → Prefer an approach that does not pollute `LibraryController` with transient mouse state; use a `DragHandleState` struct as the `on_drag` value type if GPUI supports it cleanly.
- [Risk] `.overflow_y_scrollbar()` requires `gpui_component::scroll::ScrollableElement` trait in scope. → Already used in other view files; same import pattern applies.
