## Why

`render_detail_tab_content`'s outer container was already row-direction (gpui's
`Style::default()` sets `flex_direction: Row`, and the container had no explicit
`.flex_col()` overriding it) — the cover already rendered left of the info column, which
already scrolled independently via its own `.overflow_y_scrollbar()`. What was actually
wrong: the cover hugged the left edge of the tab with no inset, unlike the info column's
`.p(px(20.0))`, and the row direction relied on an implicit default rather than being
stated explicitly.

## What Changes

- Make the detail tab's outer container's row direction explicit (`.flex_row()`) instead
  of relying on gpui's default.
- Add left padding to the cover column to match the info column's inset, so it doesn't
  hug the tab's left edge.
- No narrow-width fallback: dropped from scope per explicit decision - the row split
  applies at all tab widths.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `rust-library-ui-implementation`: Detail view layout is thumbnail-left / info-right,
  with the cover column properly inset from the tab's left edge.

## Impact

- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: `render_detail_tab_content`'s
  outer container gains an explicit `.flex_row()`; the cover column gains
  `.pl(px(20.0))`.
