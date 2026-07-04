## 1. Freeze the Anchor Position

- [x] 1.1 Add `popover_anchor_pos: Option<Point<Pixels>>` to `CatalogView`
- [x] 1.2 Set `popover_anchor_pos` from the click event position at the moment an item is
      selected (single-click), not from `last_mouse_pos` at render time
- [x] 1.3 Clear `popover_anchor_pos` when the popover closes (selection cleared, item
      double-clicked into a tab, or explicit close button)

## 2. Update the Render Path

- [x] 2.1 `render_item_popover` call site reads `popover_anchor_pos` instead of
      `self.last_mouse_pos`
- [x] 2.2 Confirm `last_mouse_pos` (if still needed elsewhere) is unaffected

## 3. Build and Quality

- [x] 3.1 `cargo check --workspace`
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Single-click a catalog item to open the popover, move the mouse around the
      catalog area, and confirm the popover stays put
- [ ] 4.2 Close the popover and click a different item; confirm it opens at the new
      item's position

## 5. Anchor Beside the Entry, Not Over It

- [x] 5.1 Add `ITEM_POPOVER_WIDTH`/`ITEM_POPOVER_MARGIN` constants and use them in
      `item_popover_view.rs` in place of inline literals
- [x] 5.2 Add `LibraryController::popover_anchor_bounds: Option<Bounds<Pixels>>`, reset on
      `select_item`/`clear_selection`, with a `set_popover_anchor_bounds` setter that only
      re-emits `LibraryChanged` when the bounds changed
- [x] 5.3 Report the selected Grid card's/Thumbs row's precise bounds via `on_prepaint`,
      gated on a new `is_selected` parameter threaded through `render_grid_card` /
      `render_thumb_row` / `render_grid`
- [x] 5.4 Add `popover_anchor_point` in `catalog_view.rs`: prefers the entry's right edge,
      falls back to its left edge when the popover wouldn't fit within the window on the
      right, and is always top-aligned with the entry
- [x] 5.5 `render()`'s popover call site uses `popover_anchor_bounds()` when set, falling
      back to the existing click-position (`popover_anchor_pos`) as a zero-size rectangle
      otherwise — this remains the only anchor for List/grouped-List rows, whose bounds
      aren't reachable from the third-party `DataTable` component
- [x] 5.6 `cargo check --workspace` / `cargo clippy --all-targets --all-features -- -D
      warnings` / `cargo test --workspace` / `cargo +nightly fmt --all -- --check`

## 6. Manual Verification — Anchor Side

- [ ] 6.1 Single-click a Grid card or Thumbs row with room to its right; confirm the
      popover opens beside it (not over it), top-aligned with the entry
- [ ] 6.2 Single-click an entry near the right edge of the window; confirm the popover
      falls back to opening on the entry's left instead of being clipped or covering it
