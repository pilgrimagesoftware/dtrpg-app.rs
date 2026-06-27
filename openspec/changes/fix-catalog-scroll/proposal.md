## Why

The catalog content area and the sidebar publisher list both use `.overflow_y_hidden()`, which clips overflowing content instead of scrolling it. When the catalog has more items than fit in the visible area, they are invisible and unreachable. Fixing this makes the full library browsable.

## What Changes

- The catalog root container changes from `.overflow_y_hidden()` to `.overflow_y_scrollbar()` so all three layouts (list, thumbs, grid) scroll vertically.
- The sidebar scrollable body changes from `.overflow_y_hidden()` to `.overflow_y_scrollbar()` so a long publisher list (and future collections list) scrolls instead of clipping.

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

<!-- none — this is a layout bug fix with no spec-level behavioral change -->

## Impact

- `dtrpg-ui/src/ui/views/catalog_view.rs` — change `.overflow_y_hidden()` to `.overflow_y_scrollbar()` on the root container; add `use gpui_component::scroll::ScrollableElement`
- `dtrpg-ui/src/ui/views/sidebar_view.rs` — change `.overflow_y_hidden()` to `.overflow_y_scrollbar()` on the scrollable body; add `use gpui_component::scroll::ScrollableElement`
