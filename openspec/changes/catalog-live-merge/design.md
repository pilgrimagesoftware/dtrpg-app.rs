## Context

`LibraryController::new` spawns a task that:
1. Reads the disk cache and calls `append_catalog_page` to pre-populate the in-memory catalog
2. Opens an mpsc channel and spawns the paginated SDK fetch
3. Receives pages one-by-one; on the **first** page it calls `ctrl.catalog.clear()` then `append_catalog_page`
4. On subsequent pages it only calls `append_catalog_page`
5. On success: saves the catalog to disk

Step 3 is the clobber: cached items disappear from the UI the moment the first live page arrives, replaced by however many items fit in one page. If the catalog has 200 items cached and the first page contains 10, the user briefly sees 10 items.

## Goals / Non-Goals

**Goals:**
- Keep the cached catalog visible in the UI for the entire duration of the live fetch
- Replace the catalog atomically with the complete live dataset when all pages are received
- Leave the cached catalog unchanged in memory if the fetch fails

**Non-Goals:**
- Merging / diffing individual items between cache and live data (a full replacement is correct)
- Changing the disk cache write behavior (already only written on full success)
- Changing the pre-population from disk behavior

## Decisions

### Accumulate into a local `Vec`, then do a single atomic swap

Collect received pages into a `live_items: Vec<LibraryItem>` allocated in the spawn closure. After the receive loop exits (channel closed), call one `this.update` that assigns `ctrl.catalog = live_items` and calls `ctrl.recompute(cx)` (or equivalent). Remove `first_page` flag and `catalog.clear()`.

**Why not per-page update?** The current per-page update is what causes the flash. Each incremental append leaves the catalog in a partially-loaded state that the UI renders. A single swap avoids all intermediate states.

**Why not merge by item id?** A full replacement is simpler, correct, and matches what the disk cache already stores. Merging would be needed only if we wanted to preserve local state (e.g. in-progress downloads) across a refresh — which is a separate concern.

### Add `set_catalog` helper to `LibraryController`

Extract the logic of replacing `self.catalog` and recomputing derived state (section counts, publisher list) into a `fn set_catalog(&mut self, items: Vec<LibraryItem>, cx: &mut Context<Self>)`. This is cleaner than accessing `ctrl.catalog` directly from the spawn closure and makes the intent clear.

### Error path: no change

If the SDK returns an error, `live_items` is dropped and `ctrl.catalog` still holds the cached data. No additional code needed.

## Risks / Trade-offs

- **Increased transient memory**: The full live catalog is held in both `live_items` (local buffer) and `ctrl.catalog` (old cached data) simultaneously for the brief moment of the swap. Acceptable — catalogs are not large enough for this to matter.
- **No incremental updates during fetch**: The sidebar counts and publisher list won't update until the swap completes. This is the trade-off for avoiding the flash. The activity indicator already communicates that a fetch is in progress.
