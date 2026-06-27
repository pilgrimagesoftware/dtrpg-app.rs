## Why

The right-side detail panel is locked at 320 px with no way to make it wider, and its body uses `.overflow_y_hidden()` so content that overflows the visible height is simply clipped. Users with long titles or descriptions see truncated content and cannot resize the panel to reveal more.

## What Changes

- A drag handle div is added to the left edge of the detail panel. Dragging it adjusts the panel width within enforced min/max bounds (240 px – 600 px). The width is persisted in `LibraryController` state and included in `LibrarySnapshot` so the panel renders at the user's chosen width across re-renders.
- `.overflow_y_hidden()` on the panel body is replaced with `.overflow_y_scrollbar()` (from `gpui_component::scroll::ScrollableElement`) so content that exceeds the panel height becomes scrollable rather than clipped.
- Text in the title, publisher, line, and description rows is allowed to wrap — any inadvertent `whitespace_nowrap` or `.truncate()` is removed from those elements so GPUI's default wrapping applies within the panel's width.
- The metadata table value column gains explicit text wrapping so long values (e.g., a long publisher name) do not overflow the row.

## Capabilities

### New Capabilities

- `detail-panel-resizable`: The detail panel width is user-adjustable via a drag handle on its left edge, with minimum and maximum constraints.
- `detail-panel-text-wrapping`: Text within the detail panel body wraps at the current panel width and the body scrolls when content exceeds the visible height.

### Modified Capabilities

<!-- none -->

## Impact

- `dtrpg-ui/src/controllers/library.rs` — add `detail_panel_width: f32` field with default 320.0; add getter and `set_detail_panel_width` mutator (clamped); expose in `LibrarySnapshot`
- `dtrpg-ui/src/ui/views/detail_panel_view.rs` — accept `width: f32` parameter; replace hard-coded `px(320.0)` with `px(width)`; add drag-handle div on left edge; fix `.overflow_y_hidden()` → `.overflow_y_scrollbar()`; fix text wrapping in title, description, and metadata value cells
- `dtrpg-ui/src/ui/views/root_view.rs` — pass `snap.detail_panel_width` to `render_detail_panel`
- No changes to the service layer or SDK
