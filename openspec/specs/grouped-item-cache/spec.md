# grouped-item-cache Specification

## Purpose
TBD - created by archiving change fix-catalog-hover-jank. Update Purpose after archive.
## Requirements
### Requirement: LibraryController caches visible items
`LibraryController` SHALL maintain an internal cache of the filtered, sorted visible item list. The cache SHALL be recomputed eagerly whenever the catalog, filter, search query, or sort settings change. Render-path accessors (`visible_items()`, `visible_items_count()`, `visible_items_slice()`) SHALL read from the cache rather than recomputing the full scan each time.

#### Scenario: Cache populated after filter change
- **WHEN** `set_filter` is called on the controller
- **THEN** `visible_items()` returns the filtered result without re-scanning the catalog

#### Scenario: Multiple accessors share one computation
- **WHEN** `visible_items_count()` and `visible_items_slice()` are called in the same render cycle after the last mutation
- **THEN** the catalog scan runs exactly once (no duplicate filter/sort work)

#### Scenario: Cache invalidated on catalog load
- **WHEN** new catalog data is loaded into the controller
- **THEN** `visible_items()` returns items from the new catalog

### Requirement: CatalogView caches grouped publisher data
`CatalogView` SHALL cache the output of `group_by_publisher()` between renders. The cache SHALL be invalidated when a `LibraryChanged` event is received. During `render()`, grouped paths SHALL read from the cache rather than calling `group_by_publisher()` directly.

#### Scenario: Groups not recomputed on hover
- **WHEN** the user moves the mouse over the catalog without any data change
- **THEN** `group_by_publisher()` is not called during re-renders triggered by that mouse movement

#### Scenario: Cache cleared on library change
- **WHEN** a `LibraryChanged` event fires (e.g., after a download completes)
- **THEN** the next render pass recomputes grouped data from the updated item list

### Requirement: Detail panel uses cached cover image
`render_detail_panel` SHALL accept an `Option<Arc<gpui::Image>>` parameter. When `Some`, it SHALL render the cached image using `img()` with `ObjectFit::Cover`. When `None`, it SHALL fall back to `render_generative_cover`.

#### Scenario: Cached thumbnail shown in detail panel
- **WHEN** an item is selected and its cover is present in `CoverCache`
- **THEN** the detail panel displays the cached thumbnail image rather than the generative cover

#### Scenario: Generative cover shown when no thumbnail cached
- **WHEN** an item is selected and its cover is absent from `CoverCache`
- **THEN** the detail panel displays the generative cover as before

