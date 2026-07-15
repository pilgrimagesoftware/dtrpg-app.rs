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

## What Changes

- `reconcile_catalog` preserves each matched item's existing per-file
  `downloaded` flags (matched by file `id`) when replacing it with the live
  item's fields, then recomputes the entry-level `status` from the merged
  file list, instead of discarding local download state.
- Files present only in the existing (cached) item's file list but absent from
  the live item are dropped along with the rest of the stale live-only fields,
  matching current reconcile-by-id semantics — only download state on
  still-present files is preserved.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `catalog-live-data-swap`: Add a requirement that reconciling a matched
  catalog item against a live fetch preserves the existing item's per-file
  downloaded state instead of resetting it to not-downloaded.

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: `reconcile_catalog` gains
  per-file `downloaded` merge logic and a `recompute_status()` call for
  matched items.
