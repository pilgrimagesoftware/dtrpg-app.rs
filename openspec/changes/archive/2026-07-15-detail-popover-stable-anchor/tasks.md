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

- [x] 4.1 Single-click a catalog item to open the popover, move the mouse around the
      catalog area, and confirm the popover stays put
- [x] 4.2 Close the popover and click a different item; confirm it opens at the new
      item's position

## 5. Anchor Beside the Entry, Not Over It

- [x] 5.1 Add `ITEM_POPOVER_WIDTH`/`ITEM_POPOVER_MARGIN` constants and use them in
      `item_popover_view.rs` in place of inline literals
- [x] 5.2 Add `LibraryController::entry_bounds: HashMap<Arc<str>, Bounds<Pixels>>`, with a
      `set_entry_bounds` setter that only re-emits `LibraryChanged` when an entry's bounds
      actually changed, and an `entry_bounds(id)` lookup
- [x] 5.3 Report every visible Grid card's/Thumbs row's precise bounds via `on_prepaint`,
      unconditionally (not gated on selection) — see task 5.7 for why
- [x] 5.4 Add `popover_anchor_point` in `catalog_view.rs`: prefers the entry's right edge,
      falls back to its left edge when the popover wouldn't fit within the window on the
      right, and is always top-aligned with the entry
- [x] 5.5 `render()`'s popover call site looks up `entry_bounds(&item.id)` for the selected
      item, falling back to the existing click-position (`popover_anchor_pos`) as a
      zero-size rectangle when unset — this remains the only anchor for List/grouped-List
      rows, whose bounds aren't reachable from the third-party `DataTable` component
- [x] 5.6 `cargo check --workspace` / `cargo clippy --all-targets --all-features -- -D
      warnings` / `cargo test --workspace` / `cargo +nightly fmt --all -- --check`
- [x] 5.7 Fix: reporting bounds only for the selected entry (first cut of 5.2/5.3) caused
      the popover to flash at the click position, then jump to the entry's bounds once
      `on_prepaint` fired a frame later. Changed to report bounds for *every* visible Grid
      card/Thumbs row continuously, keyed by item id, so the clicked entry's bounds are
      already known at click time (it was necessarily visible, and therefore already
      painted, before it could be clicked) — eliminating the flash

## 6. Manual Verification — Anchor Side

- [x] 6.1 Single-click a Grid card or Thumbs row with room to its right; confirm the
      popover opens beside it (not over it), top-aligned with the entry, immediately —
      with no flash at the click point first
- [x] 6.2 Single-click an entry near the right edge of the window; confirm the popover
      falls back to opening on the entry's left instead of being clipped or covering it
