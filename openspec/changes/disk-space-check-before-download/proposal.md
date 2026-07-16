## Why

Queuing a download today never checks whether the destination volume has room for it. A user
downloading a large entry, or using "Download All" on a big collection or publisher, can fill
their disk mid-download with no warning beforehand, leaving partial files and a confusing
failure. Comparing the calculated size of the files about to be queued against the storage
location's free space, and warning before committing, catches this before it happens instead of
after.

## What Changes

- Before queuing a download, calculate the total size (MB) of the not-yet-downloaded files that
  action would enqueue and compare it against the free disk space at the storage root
  (`StorageConfig::root_path()`).
- If free space is less than the calculated size, show a confirmation dialog naming the
  shortfall before proceeding; downloading only starts if the user confirms. If free space is
  sufficient (or can't be determined), queuing proceeds unchanged today.
- Applies to every user-initiated "start a download" action: the entry-level download action
  (single or multi-item entry), "Download All" for a collection, and "Download All" for a
  publisher. Per-file downloads already covered by one of those actions are not re-checked
  individually.

## Capabilities

### New Capabilities

- `download-disk-space-check`: Calculates pending download size vs. free disk space and warns
  the user with a confirmation dialog before queuing when space is insufficient.

### Modified Capabilities

*(none — the download queue, collection, and publisher actions gain a pre-flight check but
their own requirements are unchanged; the new capability's requirements describe the check
itself)*

## Impact

- `dtrpg-ui/src/controllers/library.rs`: `enqueue_download`, `download_all_for_collection`,
  `download_all_for_publisher`, and the direct per-file `enqueue_item_download` call site in the
  detail panel each need a pre-flight size/space check ahead of the actual queuing action.
- `dtrpg-ui/src/data/storage.rs`: gains a free-disk-space query for `StorageConfig::root_path()`.
- New dependency: a small cross-platform free-disk-space crate (e.g. `fs4`) — check for the
  current stable version before adding.
- UI call sites in `item_popover_view.rs`, `detail_panel_view.rs`, `catalog_view.rs`, and
  wherever the collection/publisher "Download All" actions are triggered, switch from calling
  the queuing methods directly to calling new gating wrapper methods.
- Reuses the existing `window.open_alert_dialog(...).confirm()` pattern already used for the
  "Clear cache" confirmation in `settings_advanced_view.rs`.
