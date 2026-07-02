## Context

GPUI rebuilds the element tree on every render pass. Mouse movement over interactive elements (`.cursor_pointer()`, `.on_click()`) can trigger re-renders because GPUI tracks hover state per-element. When a render fires, `CatalogView::render()` does:

1. `controller.read(cx).visible_items()` - scans the full catalog, filters, clones matches, and sorts. Called multiple times per render cycle (`visible_items_count()` and `visible_items_slice()` both call it internally).
2. `group_by_publisher(items)` - builds a `HashMap`, partitions all items, then sorts by name. This runs even for ungrouped modes in the branch-not-taken code.
3. `visible_items()` clone (again) for the grouped paths - all matching items cloned a second time.
4. Element tree construction for ALL items in grouped mode (non-virtualized) regardless of whether they're visible on screen.

The `visible_items()` result also feeds `visible_items_count()` and `snapshot().total_pages`, creating 3-4 full scan+filter+sort+clone passes per render frame.

## Goals / Non-Goals

**Goals:**
- Cache `visible_items()` on `LibraryController`; compute once, reuse across the same render cycle and across re-renders until filter/search/sort/catalog changes
- Cache `group_by_publisher()` output in `CatalogView`; invalidate only on `LibraryChanged`
- Use cached cover image in the detail panel rather than always rendering generative cover

**Non-Goals:**
- Virtualizing the grouped catalog paths (separate effort; pagination already limits item count)
- Profiling or tracing infrastructure

## Decisions

### Cache `visible_items()` on `LibraryController` as a dirty-flagged field

`visible_items()` is called 3-4 times per render through `visible_items_count()`, `visible_items_slice()`, and the grouped path. Adding `items_cache: Option<Vec<LibraryItem>>` to `LibraryController` and rebuilding it lazily (or eagerly on every mutation) eliminates the redundant work. Eager rebuild on mutation is simpler and correct - it runs once when filter/sort/catalog changes, not once per render.

_Alternative considered_: Cache in `CatalogView` only. Rejected because `LibraryController` owns the source data and `visible_items_count()` is also used by `CatalogListDelegate::rows_count()` which has no view-level cache.

### Cache `group_by_publisher()` in `CatalogView`

`CatalogView` already subscribes to `LibraryChanged`. Add `grouped_cache: Option<Vec<PublisherGroup>>` to `CatalogView`; set it to `None` in the subscription handler. `render()` populates it if `None`, then reads it for grouped render paths.

_Alternative considered_: Precompute groups in `LibraryController`. Rejected - grouping is a view-level concern; the controller shouldn't depend on `PublisherGroup`.

### Detail panel: pass `Option<Arc<gpui::Image>>` from caller

`render_detail_panel` currently calls `render_generative_cover` unconditionally. The caller (`LibraryRootView::render`) already has access to `cx` and can look up the item's cover from `CoverCache` before calling. Change the signature to accept `cover_image: Option<Arc<gpui::Image>>` and render `img()` when `Some`, generative cover as fallback.

## Risks / Trade-offs

- [Stale cache] If a mutation path forgets to invalidate `items_cache`, views will show stale data. Mitigation: write a single `invalidate_cache()` private method called at every mutation site; add a test asserting count changes after mutation.
- [Memory] `items_cache` holds a full clone of the filtered item list. For a 10k-item catalog filtered to 200 items, that's ~200 `LibraryItem` structs cached. Acceptable; `LibraryItem` is ~300 bytes so 200 items ≈ 60 KB.

## Open Questions

- None; implementation is straightforward given existing `LibraryChanged` subscription in `CatalogView`.
