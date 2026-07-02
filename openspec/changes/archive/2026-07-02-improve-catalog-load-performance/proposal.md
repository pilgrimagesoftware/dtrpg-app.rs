## Why

Three compounding issues cause the UI to feel unresponsive while the catalog loads: the main thread performs O(n) sidebar-stat recomputation on every API page, `visible_items()` re-filters and re-sorts the full growing catalog on every render frame triggered during load, and `LibraryChanged` fires once per page so these costs pile up repeatedly throughout the load. As the catalog grows each successive page is more expensive than the last.

## Status: superseded by other landed changes

Both problems this proposal targets have already been fixed by other changes that landed independently:

- **Visible-items cache**: fully implemented by `fix-catalog-hover-jank` (archived as `2026-07-02-fix-catalog-hover-jank`). `LibraryController` has an `items_cache: Option<Vec<LibraryItem>>` field, invalidated eagerly at every mutation site that affects the filtered/sorted set (`set_filter`, `set_search_query`, `clear_search_query`, `set_sort`, `set_sort_direction`, `set_catalog`, `append_catalog_page`, `replace_service`, `reload`, `toggle_download`, thumbnail-fetch completion). `visible_items()` reads the cache instead of re-scanning on every call — this matches (and is verified against) the `catalog-load-responsiveness` spec's "Visible items list not recomputed on unchanged state" requirement.
- **Per-page render pressure during live fetch**: fixed by `catalog-live-merge`, which changed the background load loop to accumulate all live SDK pages into a local buffer and swap the full catalog in a single `set_catalog()` call once the fetch completes, instead of appending each page to the catalog as it arrives. This is strictly better than this proposal's originally-planned 500 ms batched-flush timer — it produces exactly one main-thread render pass for the entire live fetch instead of one per 500 ms window. Implementing the timer-based buffering as originally proposed here would have been a regression against `catalog-live-merge`'s already-shipped behavior.
- The only remaining call site that appends catalog data incrementally is the disk-cache pre-population step, and it already delivers the full cached list in a single call (not paginated), so there is nothing left to batch there either.

No code changes were made under this proposal; it is archived as satisfied by prior work. See `fix-catalog-hover-jank` and `catalog-live-merge` for the implementations.

## What Changes

<!-- superseded — see Status section above -->

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

<!-- none — these are internal implementation improvements with no observable behavior change beyond improved responsiveness -->

## Impact

<!-- superseded — no changes made under this proposal; see fix-catalog-hover-jank and catalog-live-merge -->
