## Why

The catalog list view's `DataTable` already carries `gpui-component`'s built-in column resize and header-sort machinery, but those features are disabled and the sort model only covers four named methods with no direction control. Enabling them in one coordinated change makes the list view genuinely useful as a data browser and gives the sort selector a complete UI.

## What Changes

- **Title column header label**: rename from "Title / Kind" to "Title".
- **Kind indicator in title cell**: replace the inline kind text with a small text-tag badge (2–3 char abbreviation of the category, e.g. "CR" for Core Rulebook, "SUP" for Supplement, "ADV" for Adventure).
- **Column resizing**: enable `TableState::col_resizable(true)` and mark applicable columns as `resizable(true)` so the user can drag column dividers to resize them.
- **Column header sorting**: enable `TableState::sortable(true)`, call `Column::sortable()` on the sortable data columns, and implement `CatalogListDelegate::perform_sort` to propagate header-click sorts to the controller.
- **`SortMethod::Custom` variant**: add a new `Custom` variant to carry the column key for header-driven sorts. Named dropdown sorts remain as-is.
- **`SortDirection` enum**: add `Ascending` / `Descending` to `util/sort.rs`; expose it in the controller snapshot; update `sort_items` to respect it.
- **Toolbar sort selector — Custom entry**: show a "Custom" checked item in the dropdown when the current sort is `SortMethod::Custom`.
- **Toolbar sort selector — direction items**: add a separator + "Ascending" / "Descending" checked items to the dropdown; these auto-update when sort direction changes (from either a column header click or direct selection).
- **Column header ↔ toolbar sync**: when the user clicks a column header, the sort method and direction in the controller update; when the user picks a named sort from the toolbar, the DataTable column header indicators update via `TableState::refresh`.

## Capabilities

### New Capabilities

- `catalog-list-column-resize`: columns in the list view are draggable to custom widths.
- `catalog-list-column-sort`: clicking a column header sorts the catalog by that column; the active column shows an ascending/descending indicator.
- `catalog-list-sort-direction`: sort direction (ascending/descending) is a first-class concept in the sort model and is selectable from the toolbar sort menu.
- `catalog-list-kind-badge`: the item kind is displayed as a short text badge in the title cell rather than a text label.

### Modified Capabilities

## Impact

- `crates/dtrpg-ui/src/util/sort.rs` — add `SortDirection`, `SortMethod::Custom`, update `sort_items` signature
- `crates/dtrpg-ui/src/controllers/library.rs` — add `sort_direction` to snapshot; add `set_sort_direction` and `set_custom_sort` mutations
- `crates/dtrpg-ui/src/ui/views/catalog_view.rs` — update `list_columns()`, `render_td(col=0)`, `CatalogListDelegate::perform_sort`, `column()`, add `LibraryChanged` subscription to trigger `TableState::refresh`
- `crates/dtrpg-ui/src/ui/views/toolbar_view.rs` — update `render_sort_selector` dropdown items
