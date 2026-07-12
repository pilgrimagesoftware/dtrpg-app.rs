## Why

The expanded detail tab's item list (`render_item_tier` in `detail_panel_view.rs`) renders Name/Type/Status columns with fixed equal `flex_1` widths using the stateless `Table` primitive. File names are frequently truncated while the Type and Status columns sit mostly empty, and the user has no way to reclaim that space.

## What Changes

- Migrate the item list from the stateless `Table`/`TableHeader`/`TableRow`/`TableCell` primitives to `DataTable` with a `TableDelegate`, mirroring the pattern already used for the catalog list view (`catalog-list-column-sort-and-resize`).
- Enable `TableState::col_resizable(true)` and mark the Name, Type, and Status columns `resizable(true)` so the user can drag column dividers to resize them.
- Preserve existing row-selection behavior (clicking a row selects it and updates the item metadata area in place) using `DataTable`'s built-in row selection instead of the current single-div-per-row click workaround.

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- `rust-catalog-entry-detail-view`: the item list's Name, Type, and Status columns are user-resizable via drag handles on the column dividers.

## Impact

- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: `render_item_tier` replaces the `Table`-based item list with a `DataTable<ItemListDelegate>`; adds an `ItemListDelegate` struct implementing `TableDelegate` (`columns_count`, `rows_count`, `column`, `render_td`, `render_th`) for the three columns; row click/selection moves from the current wrapping-div-per-row hack to `TableState::row_selectable(true)` with a selection-change hook that calls `LibraryController::select_item_file`.
- No controller or data model changes — selection state (`selected_item_file`/`select_item_file`) already exists and is reused as-is.
- No persistence of column widths across app restarts (matches the catalog list's existing scope).
