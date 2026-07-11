## Why

Today the entry-level download button and the download queue only ever fetch a multi-item
entry's *first* file (`item.files.first()`), and the detail tab's item list has no per-row
download action at all. A user with a bundle like Moria (book + map sheet) can currently only
ever download the book — there is no way to get the other items short of downloading them
one-by-one from outside the app, and no way to see per-item download progress or status.

## What Changes

- The entry-level "Download" button on a multi-item catalog entry now enqueues every one of the
  entry's items as separate, independently tracked downloads, instead of only the first file.
- **BREAKING**: the download queue and its supporting state (`download_cancel_flags`,
  `download_activity_ids`, cancellation, activity-panel identity) are re-keyed from
  "one entry = one download" to "one file = one download" (`(entry_id, file_index)`), since an
  entry can now have more than one download in flight or queued at once.
- Each row in the detail tab's per-item list gets its own download button/status affordance, so a
  single item within a multi-item entry can be downloaded, cancelled, or re-downloaded
  independently of its siblings.
- The entry's overall download status (the badge/icon shown on the catalog card and the detail
  tab's entry-level button) reflects the aggregate of its items' individual download states
  (e.g. none downloaded / some downloaded / all downloaded) rather than a single boolean.
- Single-item entries are unaffected in outward behavior: with one file, "download the entry" and
  "download the item" are the same action.

## Capabilities

### New Capabilities

(none — this extends existing download-queue and catalog-entry-detail-view capabilities rather
than introducing a new one)

### Modified Capabilities

- `download-queue`: the queue's unit of work changes from one-per-entry to one-per-file; an entry
  with N items can have up to N downloads queued/active/cancellable independently, each with its
  own activity panel entry.
- `rust-catalog-entry-detail-view`: the per-item list gains a download action per row and a
  per-item download status affordance; the entry-level status/download button reflects aggregate
  per-item state instead of a single entry-wide flag.

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: `download_queue`, `download_cancel_flags`,
  `download_activity_ids`, `enqueue_download`, `remove_download`, `dispatch_download`,
  `drain_download_queue`, and the entry-level download button's click handler all move from
  `Arc<str>` (entry id) keys to a per-file key.
- `crates/dtrpg-ui/src/data/library.rs`: `LibraryItem`/`LibraryItemFile` need a way to represent
  per-file download status (today `ItemStatus` lives only on `LibraryItem`).
- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: `render_item_tier`'s per-item rows gain a
  download button/status cell; the entry-level download button's enabled/label state is derived
  from aggregate item status.
- `crates/dtrpg-ui/src/ui/views/catalog_view.rs`: entry-level download affordances (list row,
  grid tile) that currently read a single `ItemStatus` need to derive the same aggregate state.
- `crates/dtrpg-core/src/services/sdk/library/*`: `LibraryService::download_item` is already
  file-scoped (`order_product_id` + file `index`); no service-layer contract change expected,
  only how the controller calls it (once per file instead of once per entry).
- Builds on the not-yet-merged `implement-real-file-downloads` change, which introduces
  `LibraryItemFile::index` and the real `download_item` SDK-backed fetch this change now calls
  once per item instead of once per entry.
