## Why

When the SDK returns the first live catalog page, the controller clears the pre-populated cache and replaces it with only that single page — causing a visible flash where the full cached catalog is discarded and the catalog temporarily shows fewer items than before the fetch began. The cache should remain visible until the complete live dataset is ready to replace it atomically.

## What Changes

- Accumulate all live SDK pages into a local buffer during the fetch instead of appending them directly to the catalog
- Replace the full catalog with the accumulated buffer in a single update once all pages have arrived
- Remove the `first_page` / `catalog.clear()` pattern that caused the clobber

## Capabilities

### New Capabilities

- `catalog-live-data-swap`: The catalog loading flow accumulates live SDK pages in a local buffer and swaps the full catalog atomically when the fetch completes, leaving cached data visible throughout.

### Modified Capabilities

_(none — the cache pre-population behavior is unchanged)_

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: The page-receive loop collects into a local `Vec<LibraryItem>` buffer; a single `ctrl.set_catalog(live_items, cx)` call replaces the catalog atomically after the channel closes. On error, the cached catalog is left unchanged.
