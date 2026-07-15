## Why

`LibraryItemFile::downloaded` is currently a purely optimistic flag: it's set
to `true` when a download completes (`dispatch_download`) and otherwise
trusted wherever it's read (catalog display, "On This Device" filtering,
open-file routing). Nothing ever confirms the file is still actually present
on disk. `fix-download-status-lost-on-restart` and
`fix-item-check-clears-download-status` fix the two known ways this flag
gets incorrectly *reset* to `false` (a live catalog reconcile and a
per-item availability check both used to wholesale-overwrite it), but even
with those fixes, the flag can still disagree with reality in the other
direction — e.g. a user manually deletes a downloaded file outside the app,
or restores a stale disk-image without the file, and the app keeps reporting
`Downloaded` until the user tries to open it and hits `OpenError::FileNotFound`.

## What Changes

- Add a file-presence verification pass that computes each file's expected
  on-disk path (the same `StorageConfig::path_for_publisher(publisher).join(file.name)`
  resolution `dispatch_download` already uses) and sets `downloaded` to
  whether a file actually exists there — in both directions: marks a file
  downloaded if found on disk even if the flag said otherwise, and marks it
  not-downloaded if the flag said `true` but the file is missing.
- Run this verification once per catalog load (after the catalog settles,
  whichever of the three load paths — skipped-fetch, partial-fetch, or
  full-fetch — was taken), on the background executor, then apply results
  and recompute `status`/`section_counts` for any items that changed.
- Also run it on-demand for a single item when the user selects it (the
  existing single-click popover / detail-tab open path), so an external
  change to that specific item's files is caught without waiting for the
  next full catalog load.
- This layers on top of the two existing flag-preservation fixes rather than
  replacing them — the merge-by-id logic still prevents unnecessary flicker
  from every live fetch/check; this verification pass is the ground-truth
  backstop for cases neither of those fixes addresses.

## Capabilities

### New Capabilities

- `verify-downloaded-status-against-disk`: `LibraryItemFile::downloaded`
  MUST reflect actual file presence on disk, verified on catalog load and
  on selecting an individual item, in both directions (marking downloaded
  when found, not-downloaded when missing).

### Modified Capabilities

_(none)_

## Impact

- New module `crates/dtrpg-ui/src/util/file_presence.rs`: path resolution
  and per-item verification helpers.
- `crates/dtrpg-ui/src/controllers/library.rs`: `dispatch_download`'s inline
  destination-path computation is extracted into the new shared helper;
  `start_load_inner` gains a verification pass after each of its three
  catalog-settled points; `select_item` gains an on-demand single-item
  verification.
