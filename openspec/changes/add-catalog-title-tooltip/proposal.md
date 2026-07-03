## Why

Catalog item titles are truncated with an ellipsis in the grid, list, grouped list, and thumbnail layouts whenever
the title is wider than its column or card slot. Users have no way to read the full title without opening the
item, which slows down browsing and scanning for a specific product.

## What Changes

- Add a tooltip that shows the full item title when the rendered title text is truncated in the catalog grid card,
  flat list row, grouped list row, and thumbnail row.
- The tooltip only appears when the title is actually truncated (i.e. the rendered text is narrower than its full
  content), not on every hover.
- No tooltip is shown for titles that fit fully within their allotted space.

## Capabilities

### New Capabilities
- `catalog-title-tooltip`: Truncation-aware tooltip on catalog item title text across grid, flat list, grouped
  list, and thumbnail views.

### Modified Capabilities
(none — no existing requirements change; this adds new behavior only)

## Impact

- Affected code: `crates/dtrpg-ui/src/ui/views/catalog_view.rs` (grid card title in `render_grid_card`, flat list
  row title in `render_td`, grouped list row title in `render_grouped_list_row`, thumbnail row title in
  `render_thumb_row`).
- No API, data model, or SDK changes.
