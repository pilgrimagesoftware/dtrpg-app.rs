## Why

`render_item_popover` is anchored at `self.last_mouse_pos`, and `CatalogView` updates
`last_mouse_pos` on every `on_mouse_move` event over the catalog area — including while
the popover is already open. The popover therefore continuously re-anchors to the current
cursor position instead of staying pinned to the location the item was clicked, making it
visibly jump around as the mouse moves after opening.

## What Changes

- The popover's anchor position is captured once, at the moment an item is selected
  (single-click), and stored separately from the continuously-updated
  `last_mouse_pos` (e.g. `popover_anchor_pos: Option<Point<Pixels>>`, set on click and
  cleared when the popover closes).
- `render_item_popover` reads the frozen anchor position instead of `last_mouse_pos`, so
  subsequent mouse movement while the popover is open no longer moves it.
- `last_mouse_pos` continues to update as before for any other use (if any); it is simply
  no longer read by the popover render path.
- The popover no longer opens directly on top of the clicked catalog entry. It anchors to
  the right of the entry's own on-screen bounds; if there isn't room to the right of the
  window, it anchors to the left of the entry instead. It is always top-aligned with the
  entry, so it never covers it.
- Grid cards and Thumbs rows report their precise bounds (`Bounds<Pixels>`) once painted
  after selection, via `LibraryController::popover_anchor_bounds`, so the anchor is
  computed against the real entry rectangle rather than the click point. List/grouped-List
  rows (rendered by the third-party `DataTable` component, whose per-row bounds aren't
  reachable from `CatalogView`) fall back to the click position; because those rows span
  nearly the full catalog width, "beside the entry" isn't achievable there regardless of
  bounds precision.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `rust-library-ui-implementation`: The single-click item popover is anchored to the
  position at which it was opened and does not move in response to subsequent mouse
  movement. It anchors beside (right, then left) the catalog entry that opened it rather
  than over it.

## Impact

- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: `CatalogView` gains a
  `popover_anchor_pos: Option<Point<Pixels>>` field, set from the click event that selects
  an item and cleared when the popover is dismissed; a new `popover_anchor_point` helper
  computes the final right-then-left anchor point from the entry's bounds and the window
  width; `render_grid_card`/`render_thumb_row`/`render_grid` gain an `is_selected` flag
  used to report the selected entry's bounds via `on_prepaint`.
- `crates/dtrpg-ui/src/controllers/library.rs`: `LibraryController` gains
  `popover_anchor_bounds: Option<Bounds<Pixels>>`, reset on every `select_item`/
  `clear_selection`, with a `set_popover_anchor_bounds` setter that only re-emits
  `LibraryChanged` when the bounds actually changed (avoids a render feedback loop).
- `crates/dtrpg-ui/src/ui/views/item_popover_view.rs`: reads the popover width/margin from
  new `ITEM_POPOVER_WIDTH`/`ITEM_POPOVER_MARGIN` constants instead of inline literals.
- `crates/dtrpg-ui/src/data/constants.rs`: adds `ITEM_POPOVER_WIDTH`/`ITEM_POPOVER_MARGIN`.
