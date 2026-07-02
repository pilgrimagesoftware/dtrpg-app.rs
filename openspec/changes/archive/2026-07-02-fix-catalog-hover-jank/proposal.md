## Why

Moving the mouse over the catalog or detail view triggers GPUI re-renders that run expensive work on the main thread: `visible_items()` clones the entire item list, `group_by_publisher()` re-partitions and sorts it, and the non-virtualized grouped paths build element trees for every item regardless of visibility - all on every rendered frame.

## What Changes

- Cache grouped item data (`Vec<PublisherGroup>`) in `LibraryController`, recomputed only when `LibraryChanged` fires rather than on every render pass
- Remove the per-render `visible_items()` clone in grouped modes by reading from the pre-computed cache
- Pre-compute `thumbnail_cooldown_elapsed` state (a `SystemTime::now()` call) once per render cycle instead of once per rendered item
- Use the `CoverCache` in the detail panel cover so `render_generative_cover` is only called as a fallback when no thumbnail is cached

## Capabilities

### New Capabilities

- `grouped-item-cache`: Cached `Vec<PublisherGroup>` on `LibraryController`, invalidated on `LibraryChanged`, read by `CatalogView` in grouped render paths

### Modified Capabilities

- None

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: Add `grouped_cache: Option<Vec<PublisherGroup>>` field; add `visible_grouped()` accessor; invalidate on library change
- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: Grouped render paths read `visible_grouped()` instead of cloning + grouping inline; compute `now` once and derive cooldown bool before building item elements
- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: Accept `Option<Arc<gpui::Image>>` cover parameter; use `img()` when present, `render_generative_cover` as fallback
- `crates/dtrpg-ui/src/ui/views/root_view.rs`: Pass cover image from `CoverCache` to `render_detail_panel`
