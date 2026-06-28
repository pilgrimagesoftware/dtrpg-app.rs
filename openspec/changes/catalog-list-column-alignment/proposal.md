## Why

The catalog list view header (`render_list_header`) allocates a fixed `w(px(24.0))` slot for the status column, but `render_list_row` inserts the status glyph as a bare child with no width constraint. The status element's natural width is 7px (a filled dot) or text-width (a ☁ glyph) — both narrower than 24px. Because the row also contains a `flex_1` title column, the shortfall is absorbed there: the title column is wider in rows than in the header, and every subsequent fixed-width column (Publisher, System, Pages, Size, Added, and the reveal icon) starts at a different x position than its header label.

## What Changes

- Wrap `render_status(status, &colors)` in `render_list_row` with a `div().w(px(24.0))` fixed-width container so the status slot matches the header's allocation
- Center the status glyph within that container

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- `rust-main-window-library-layout`: List view status column in data rows SHALL have a fixed width matching the header's status column allocation

## Impact

- `catalog_view.rs`: `render_list_row` — one-line change wrapping `render_status(...)` in a `div().w(px(24.0)).flex().items_center().justify_center()`
- No other files, no controller changes, no data model changes
