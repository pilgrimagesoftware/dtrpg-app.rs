## Why

`render_detail_tab_content` currently stacks the cover thumbnail above the title,
publisher, description, and actions in a single `.flex_col()` — the cover is centered at
the top of a scrolling column with all text below it. The intended layout is a
side-by-side split: thumbnail on the left, information on the right, so the cover stays
visible while the description and metadata scroll independently.

## What Changes

- The detail tab's outer container becomes `.flex_row()`: a fixed-width left column
  holding the cover (and its "refresh thumbnail" overlay button) and a flexible right
  column holding publisher, title, status icon, line, description, actions, and metadata.
- The left column no longer scrolls; the right column keeps its existing
  `.overflow_y_scrollbar()` behavior so long descriptions/metadata scroll independently of
  the cover.
- At narrow tab widths (below a minimum threshold) the layout falls back to the current
  stacked (cover-above-info) arrangement so the cover is not squeezed unreadably thin.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `rust-library-ui-implementation`: Detail view layout is thumbnail-left / info-right at
  normal widths, falling back to stacked layout below a minimum tab width.

## Impact

- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: `render_detail_tab_content`
  restructured from a single `.flex_col()` to a `.flex_row()` split with a fixed-width
  cover column and a flexible, independently scrollable info column; width-based fallback
  to the current stacked layout.
