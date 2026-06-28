## Why

When the library data updates (sort change, filter change, data reload), `TableState::refresh()` is called to sync sort indicators. `refresh()` calls `prepare_col_groups()` internally, which rebuilds column metadata from the delegate's `column()` method — resetting every column's width to the static values in `list_columns()`. Any column widths the user has dragged to resize are silently discarded.

## What Changes

- The `CatalogListDelegate` will track the user's current column widths in a `Vec<Pixels>`, updated whenever `TableEvent::ColumnWidthsChanged` fires.
- `CatalogListDelegate::column()` will return the stored width instead of the static default when a user-adjusted width is available.
- `CatalogView::new()` will subscribe to `TableEvent::ColumnWidthsChanged` on the `catalog_list_table` entity and write the new widths into the delegate.

## Capabilities

### New Capabilities

- `catalog-list-column-width-persistence`: The catalog list view preserves user-resized column widths across library reloads and sort updates.

### Modified Capabilities

_(none — no existing spec requirement changes)_

## Impact

- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: `CatalogListDelegate` struct, `column()` impl, `CatalogView::new()`
- No public API changes; no new dependencies
