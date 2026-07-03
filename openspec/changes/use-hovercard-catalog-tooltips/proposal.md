## Why

Catalog rows, activity items, and other truncated text currently rely on `gpui`'s plain
`.tooltip()` builder, which only shows a single line of text. `gpui-component` ships
`HoverCard`, a richer anchored popover that supports multi-line, styled content (icon +
title + description, key-value rows) with configurable open/close delay. Several places in
the app would benefit from a fuller preview than a plain text tooltip allows.

## What Changes

- Catalog list/thumbs/grid item rows show a `HoverCard` on hover containing the item's
  title, publisher, and status, replacing the current plain-text tooltip on truncated
  title cells.
- The detail panel's file-openers row and metadata values that use plain tooltips for
  truncated content are converted to `HoverCard` where the content benefits from more
  than one line (title + description already using `.tooltip()` stays as-is where a
  single line is sufficient).

## Capabilities

### New Capabilities

- `catalog-hovercard-tooltips`: Catalog item rows show a rich `HoverCard` (title,
  publisher, status) on hover instead of a plain single-line tooltip.

### Modified Capabilities

_(none — this is additive UI polish with no change to existing spec-level behavior)_

## Impact

- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: Replace `.tooltip()` on truncated title
  cells with `gpui_component::hover_card::HoverCard`.
- No new dependencies (`HoverCard` is already part of the `gpui-component` dependency).
