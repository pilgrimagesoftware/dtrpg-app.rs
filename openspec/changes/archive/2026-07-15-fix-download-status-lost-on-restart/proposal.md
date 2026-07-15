## Why

On restart, `LibraryController::start_load` loads the cached catalog (with each
item's correct `files[*].downloaded` flags and `status`) and then reconciles it
against a fresh live SDK fetch via `reconcile_catalog`. For any item present in
both, `reconcile_catalog` replaces the cached item wholesale with the live one
(`item = live_item`). Live items are always mapped with `downloaded: false` for
every file (`crates/dtrpg-core/src/services/sdk/library/mapping.rs`), since the
API has no concept of local downloads. The result: every previously-downloaded
entry flips back to `Cloud` status as soon as the startup live fetch completes,
even though the files are still on disk.

A second, independent bug produces the same symptom: `save_catalog_cache` is
only ever called from `start_load_inner`'s full- and partial-fetch completion
paths. The download-completion handler (`dispatch_download`'s spawned task)
sets `file.downloaded = true` and calls `item.recompute_status()` on the
in-memory catalog, but never persists that change to disk. If the on-disk
cache is still within its 7-day freshness window and the remote item count
hasn't changed, `start_load_inner`'s auto-load policy skips the live fetch
entirely and shows the on-disk cache as-is — so a restart shows `Cloud` even
though `reconcile_catalog` is fixed, because the disk cache was never
rewritten to record the download in the first place.

## What Changes

- `reconcile_catalog` preserves each matched item's existing per-file
  `downloaded` flags (matched by file `id`) when replacing it with the live
  item's fields, then recomputes the entry-level `status` from the merged
  file list, instead of discarding local download state.
- Files present only in the existing (cached) item's file list but absent from
  the live item are dropped along with the rest of the stale live-only fields,
  matching current reconcile-by-id semantics — only download state on
  still-present files is preserved.
- The download-completion handler in `dispatch_download` calls
  `save_catalog_cache` immediately after a successful (non-cancelled)
  download, so the on-disk cache reflects the download without waiting for
  the next live fetch.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `catalog-live-data-swap`: Add a requirement that reconciling a matched
  catalog item against a live fetch preserves the existing item's per-file
  downloaded state instead of resetting it to not-downloaded.
- `real-file-download-transfer`: Add a requirement that a successful download
  persists the updated catalog to the on-disk cache immediately, not only on
  the next live fetch.

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: `reconcile_catalog` gains
  per-file `downloaded` merge logic and a `recompute_status()` call for
  matched items; `dispatch_download`'s completion handler gains a
  `save_catalog_cache` call on successful downloads.
