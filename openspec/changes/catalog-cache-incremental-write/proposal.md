## Why

`save_catalog_cache` is only called once after the entire live fetch has completed and the
in-memory catalog has been swapped in (`catalog-live-merge`'s atomic-swap design). If the
app quits or crashes partway through a load — a large library can take a while — nothing
is written to disk at all, so the next launch either shows no cache (first run) or a
cache that is a full load-cycle stale, even though a complete set of pages may already
have been fetched from the API by the time the app closed.

## What Changes

- The background load task checkpoints the accumulating page buffer to disk periodically
  (e.g. every N pages, or on a time interval) via `save_catalog_cache`, independent of the
  in-memory atomic swap that still happens only once the full fetch completes.
- Checkpoint writes use the same atomic `.tmp`-then-rename pattern `save_catalog_cache`
  already uses, so a crash mid-checkpoint-write cannot corrupt the cache file.
- The final post-fetch save is unchanged; checkpointing is purely an additional safety net
  for interrupted loads and does not change the atomic in-memory swap behavior from
  `catalog-live-merge`.

## Capabilities

### New Capabilities

- `catalog-cache-checkpointing`: The catalog cache file is checkpointed to disk
  periodically during a live load, not only after the full fetch completes, so an
  interrupted load still leaves a recent (if partial) cache for the next startup.

### Modified Capabilities

_(none — `catalog-disk-cache`'s existing read-on-startup and post-load-save behavior is
unchanged; this adds an additional write point)_

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: The page-receive loop periodically spawns
  a `save_catalog_cache` call against the accumulating buffer (not the in-memory
  `self.catalog`, which is not updated until the full swap) at a configurable page-count or
  time interval.
- `crates/dtrpg-ui/src/data/catalog_cache.rs`: No changes expected — `save_catalog_cache`
  is already safe to call repeatedly.
