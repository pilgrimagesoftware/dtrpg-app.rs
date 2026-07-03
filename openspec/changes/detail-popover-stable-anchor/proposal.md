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

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `rust-library-ui-implementation`: The single-click item popover is anchored to the
  position at which it was opened and does not move in response to subsequent mouse
  movement.

## Impact

- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: `CatalogView` gains a
  `popover_anchor_pos: Option<Point<Pixels>>` field, set from the click event that selects
  an item and cleared when the popover is dismissed; `render_item_popover` call site reads
  this field instead of `self.last_mouse_pos`.
