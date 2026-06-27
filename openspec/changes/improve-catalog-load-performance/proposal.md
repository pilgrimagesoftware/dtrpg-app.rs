## Why

Three compounding issues cause the UI to feel unresponsive while the catalog loads: the main thread performs O(n) sidebar-stat recomputation on every API page, `visible_items()` re-filters and re-sorts the full growing catalog on every render frame triggered during load, and `LibraryChanged` fires once per page so these costs pile up repeatedly throughout the load. As the catalog grows each successive page is more expensive than the last.

## What Changes

- **Batch page flushes**: The background load task accumulates items across pages and flushes to the UI at most once every 500 ms (or when a flush-size threshold is reached), instead of once per API page. This reduces the number of main-thread interruptions and re-renders during load from O(pages) to O(seconds / 0.5).
- **Visible-items cache**: `LibraryController` caches the result of `visible_items()` and only recomputes it when the filter, sort, search query, or catalog content actually changes. Render passes during load read the cache; the cache is invalidated by mutation methods, not by the render path.
- **Sidebar stats deferred to batch boundary**: `section_counts` and `publisher_entries` are recomputed only when the accumulated batch is flushed to the UI (i.e., at most once per 500 ms), not on every individual page arrival.

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

<!-- none — these are internal implementation improvements with no observable behavior change beyond improved responsiveness -->

## Impact

- `dtrpg-ui/src/controllers/library.rs` — add `visible_cache: Option<Vec<LibraryItem>>` field; invalidate cache in all mutation methods; serve cache in `snapshot()`; replace per-page `LibraryChanged` with batched flush timer
- `dtrpg-ui/src/controllers/library.rs` — the background load task accumulates items in a local buffer and flushes on a 500 ms cadence using `cx.spawn` + `async_cx.background_executor().spawn` timer
- No changes to the service layer, SDK, or view rendering code
