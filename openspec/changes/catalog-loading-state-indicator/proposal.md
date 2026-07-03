## Why

`gpui-component`'s `List` and `Table` delegates expose a `loading()` hook that renders a
built-in skeleton view while data is not yet available, but `CatalogListDelegate` never
overrides it, so the list presentation shows an empty table with column headers and no
rows during the initial catalog fetch. `adopt-gpui-component-primitives` already covers
showing a centered `Spinner` for the catalog's thumbs/grid empty-pane case; this change
covers the list presentation's native skeleton loading state and extends the same signal
to other content views (sidebar publisher/collection menus, activity panel) that currently
show an empty section with no indication a fetch is in progress.

## What Changes

- `CatalogListDelegate::loading()` returns `true` while the catalog's initial load is in
  flight (mirrors the same `is_loading` state used to trigger the grid/thumbs `Spinner`),
  so the list presentation shows `gpui-component`'s built-in skeleton rows instead of an
  empty table.
- The sidebar's Publishers and Collections sections show a small inline loading indicator
  (compact `Spinner` or skeleton row) in place of "no items" text while the initial
  catalog/collections fetch has not yet completed.

## Capabilities

### New Capabilities

- `list-loading-skeleton`: The catalog list presentation shows a built-in skeleton loading
  view via the `Table` delegate's `loading()` hook while the initial fetch is in flight.

### Modified Capabilities

_(none)_

## Impact

- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: `CatalogListDelegate::loading()`
  override reading `LibraryController`'s existing loading state.
- `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`: Publisher/Collection section bodies show
  a loading indicator instead of an empty-state message while loading.
- Builds on `adopt-gpui-component-primitives` (grid/thumbs `Spinner`); this change covers
  the list presentation and sidebar sections it does not.
