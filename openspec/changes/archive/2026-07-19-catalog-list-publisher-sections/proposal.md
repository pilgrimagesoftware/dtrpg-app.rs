## Why

The grouped list presentation (list view with "group by publisher" on) does not use
`DataTable` at all — `CatalogView` falls back to hand-rolled flex rows
(`render_grouped_list_header` / `render_grouped_list_row`) built from `PublisherGroup`
data, bypassing the virtualized, delegate-based `Table` used by the ungrouped list path.
This diverges from column widths, sorting, and alignment already implemented on
`CatalogListDelegate`, and is the likely cause of laggy scrolling reported against large
grouped catalogs.

`gpui-component`'s `List`/`ListDelegate` exposes a native row-sections API
(`sections_count`, `items_count(section)`, `render_section_header`, `render_section_footer`)
— but that API belongs to `ListDelegate` only. `TableDelegate` (which backs `DataTable`,
and which `CatalogListDelegate` implements to get `DataTable`'s column width/sort/resize
behavior) has no equivalent, on both the pinned `gpui-component` revision
(`be4c5d30e0a51d5bfb2df93477a05050a50bf889`) and upstream `HEAD` — verified directly against
both `crates/ui/src/table/delegate.rs` sources. Switching the grouped list to `List` would
gain sections but lose `Table`'s shared column machinery, which is the actual point of this
change. The virtualization goal is instead met by giving the grouped list its own
`TableDelegate` (`GroupedCatalogListDelegate`) over a flattened header/item row list.

## What Changes

- `GroupedCatalogListDelegate` (a second `TableDelegate` implementation, alongside the
  existing `CatalogListDelegate`) renders the grouped list from a flattened `GroupedRow`
  list (`Header { publisher, count }` / `Item(LibraryItem)`) built by
  `CatalogView::grouped_items` from `group_by_publisher`. Header rows render the publisher
  name and item count in column 0 (other columns render blank, keeping cell alignment with
  data rows); item rows render through the same `render_list_item_cell` helper the
  ungrouped path uses.
- The grouped list presentation renders through its own `DataTable::new(&self.catalog_grouped_list_table)`
  instance instead of the hand-rolled `div()` tree, so it is virtualized. It shares column
  *definitions* (`list_columns()`) with the ungrouped table; user-resized column widths are
  now propagated between the two tables' delegates on `TableEvent::ColumnWidthsChanged` so a
  resize in either presentation is reflected in the other.
- `render_grouped_list_header` and `render_grouped_list_row` (and the now-unused manual
  grouping render path in `catalog_view.rs`) are removed; `group_by_publisher` /
  `PublisherGroup` now back `CatalogView::grouped_items`, which caches the grouping and the
  flattened `GroupedRow` list it feeds into `catalog_grouped_list_table`.
- Row selection (`TableEvent::SelectRow`, `DoubleClickedRow`) and the context menu ignore
  `GroupedRow::Header` rows — only `GroupedRow::Item` rows select an item or open a context
  menu; the header row instead offers a "download all for this publisher" context action.

## Capabilities

### New Capabilities

- `catalog-list-sections`: Grouped list presentation renders publisher sections through a
  dedicated `GroupedCatalogListDelegate` over a flattened header/item row list, giving it
  the same `DataTable` virtualization and column-width/sort/resize sharing as the ungrouped
  list.

### Modified Capabilities

- `grouped-item-cache`: The cached `Vec<PublisherGroup>` on `CatalogView` (`grouped_cache`)
  now also backs the flattened `GroupedRow` list pushed into `catalog_grouped_list_table`,
  instead of driving a hand-rolled render tree. Invalidated on `LibraryChanged`, same as
  before.

## Impact

- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: `GroupedCatalogListDelegate` and
  `GroupedRow`; `CatalogView::grouped_items`; removal of `render_grouped_list_header`,
  `render_grouped_list_row`, and the hand-rolled `(CatalogPresentation::List, true)` match
  arm; column-width propagation between `catalog_list_table` and
  `catalog_grouped_list_table`.
- Directly addresses the "largest remaining item" hand-rolled grouped list noted in
  project notes and the `gpui-component`-first UI policy in this crate's `AGENTS.md`.
