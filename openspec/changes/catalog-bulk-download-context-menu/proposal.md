## Why

Downloading every item in a collection or from a publisher today requires opening each catalog
entry individually and triggering its own download action — tedious for collections or
publishers with more than a handful of entries. Users need a single action that queues every
not-yet-downloaded item in a collection or publisher group at once.

## What Changes

- Add a "Download All" action to the collection sidebar entry's existing right-click context menu
  (alongside "Reload" and "Delete"), which enqueues a download for every not-yet-downloaded item
  that belongs to that collection.
- Add a right-click context menu to publisher group headers in the catalog's grouped-by-publisher
  view (Grid/Thumbs/List), with a "Download All" action that enqueues a download for every
  not-yet-downloaded item under that publisher. This is a new interaction — publisher group
  headers currently have no context menu.
- Both actions reuse the existing per-item `enqueue_download` entry point and the existing
  bounded download queue — no changes to queue mechanics, concurrency limits, or per-file
  cancellation.
- Items already downloaded, queued, or actively downloading are skipped rather than re-enqueued,
  matching the existing entry-level "download all files in this entry" behavior.

## Capabilities

### New Capabilities

- `publisher-context-menu`: Publisher group headers in the catalog gain a right-click context
  menu with a "Download All" action, mirroring `collection-context-menu`'s existing pattern.

### Modified Capabilities

- `collection-context-menu`: Adds a "Download All" action to the collection sidebar entry's
  context menu.

## Impact

- `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`: Add a "Download All" `PopupMenuItem` to the
  existing collection row `context_menu`.
- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: Add a `context_menu` to `render_group_header`
  (used by grouped Grid/Thumbs rendering and the `GroupedCatalogListDelegate` header row) with a
  "Download All" action, capturing the publisher name.
- `crates/dtrpg-ui/src/controllers/library.rs`: Add `download_all_for_collection(collection_id)`
  and `download_all_for_publisher(publisher)` methods that filter `self.catalog` (via
  `member_ids_contain` for collections, exact publisher match for publishers) and call
  `enqueue_download` per matching, not-yet-downloaded item.
- No changes to `download-queue` mechanics, the SDK, or the API contract — this only adds two new
  call sites into the existing single-item download entry point.
