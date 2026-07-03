## Why

The grouped list presentation (list view with "group by publisher" on) does not use
`DataTable` at all — `CatalogView` falls back to hand-rolled flex rows
(`render_grouped_list_header` / `render_grouped_list_row`) built from `PublisherGroup`
data, bypassing the virtualized, delegate-based `Table` used by the ungrouped list path.
This diverges from column widths, sorting, and alignment already implemented on
`CatalogListDelegate`, and is the likely cause of laggy scrolling reported against large
grouped catalogs. `gpui-component`'s `List`/`Table` delegate trait already exposes a
native sections API (`sections_count`, `items_count(section)`, `render_section_header`,
`render_section_footer`) purpose-built for this case.

## What Changes

- `CatalogListDelegate` implements the sections API: `sections_count` returns the number
  of distinct publishers in the current view, `items_count(section)` returns each
  publisher's item count, and `render_section_header` renders the publisher name and item
  count (replacing `render_group_header` / `render_grouped_list_header`).
- The grouped list presentation renders through the same `DataTable` instance used for the
  ungrouped path instead of the hand-rolled `div()` tree, so it is virtualized and shares
  column widths/sort/resize behavior with the ungrouped list.
- `render_grouped_list_header` and `render_grouped_list_row` (and the now-unused manual
  grouping render path in `catalog_view.rs`) are removed once the delegate-based sections
  path is in place; `group_by_publisher` / `PublisherGroup` continue to back
  `sections_count` / `items_count` lookups.

## Capabilities

### New Capabilities

- `catalog-list-sections`: Grouped list presentation renders publisher sections through
  `CatalogListDelegate`'s native sections API, sharing the same virtualized `DataTable`
  and column behavior as the ungrouped list.

### Modified Capabilities

- `grouped-item-cache`: The cached `Vec<PublisherGroup>` on `LibraryController` now backs
  delegate section lookups (`sections_count`, `items_count`) instead of driving a
  hand-rolled render tree.

## Impact

- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: `CatalogListDelegate` sections
  implementation; removal of `render_grouped_list_header`, `render_grouped_list_row`, and
  the `(CatalogPresentation::List, true)` hand-rolled match arm.
- Directly addresses the "largest remaining item" hand-rolled grouped list noted in
  project notes and the `gpui-component`-first UI policy in this crate's `AGENTS.md`.
