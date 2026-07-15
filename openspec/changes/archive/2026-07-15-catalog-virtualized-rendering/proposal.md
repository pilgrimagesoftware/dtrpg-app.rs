## Why

With hundreds or thousands of catalog items, the GPUI layout engine measures and positions every item on every render frame — O(n) layout work that grows linearly with the catalog size. Users experience frame drops and input lag during any UI interaction after a large catalog loads.

## What Changes

- `CatalogView` is promoted from a pure render function (`render_catalog`) to a GPUI `View` entity that implements `Render`, allowing it to hold state (a `UniformListScrollHandle`) and access `Entity<LibraryController>` inside the `uniform_list` render closure.
- All three catalog layouts (list, thumbs, grid) use `uniform_list` from GPUI, which renders only the visible window of items rather than the full set. The layout engine only measures the rows that appear on screen.
- The grid layout tiles multiple items into rows, each row rendered as a single `uniform_list` item, so the count passed to `uniform_list` is `ceil(items.len() / items_per_row)` rather than `items.len()`.
- Grouped mode (grouped by publisher) is excluded from virtualization in this change because group headers break the uniform-height requirement; it falls back to the existing non-virtualized rendering.
- `render_catalog` free function is removed; `CatalogView::render` replaces it.
- `LibraryRootView` creates a `CatalogView` entity rather than calling `render_catalog` directly.

## Capabilities

### New Capabilities

- `catalog-virtualized-rendering`: The catalog renders only the visible window of items at any time; frame layout cost is O(visible rows) rather than O(total items).

### Modified Capabilities

<!-- none -->

## Impact

- `dtrpg-ui/src/ui/views/catalog_view.rs` — primary change: convert from a module of free functions to a `CatalogView` struct implementing GPUI `Render`; use `uniform_list` in all non-grouped layouts
- `dtrpg-ui/src/ui/views/root_view.rs` (or wherever `render_catalog` is called from the root/library view) — create `CatalogView` entity instead of calling `render_catalog`
- No changes to `LibraryController`, the service layer, or the SDK
- No new dependencies — `uniform_list` is already provided by the `gpui` crate
