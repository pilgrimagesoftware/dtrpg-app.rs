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
