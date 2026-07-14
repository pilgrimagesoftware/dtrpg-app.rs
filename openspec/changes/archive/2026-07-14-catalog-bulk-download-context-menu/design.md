## Context

Every catalog entry already has a per-item "Download" context menu action
(`catalog_view.rs`, `ItemStatus::Cloud` branch) that calls
`LibraryController::enqueue_download(id, title, cx)`. That method already resolves
`missing_file_indices` for the item and skips files that are already downloaded or already
queued/in-flight (`enqueue_item_download`'s doc comment: "No-op if the item/file does not
exist, the file is already downloaded, or that specific file is already queued/in flight").
There is no bulk equivalent — downloading every item in a collection or from a publisher
requires opening each entry and clicking Download individually.

Collections and publisher groups are both already-materialized groupings of `LibraryItem`s:
- Collection membership: `CollectionEntry.member_ids: Arc<[u64]>`, matched against an item via
  `util::matching::member_ids_contain(member_ids, item.order_product_id, item.product_id)`.
- Publisher grouping: `util::publisher::group_by_publisher` groups `LibraryItem`s by exact
  `item.publisher` string match; `catalog_view.rs` caches this as `CatalogView::grouped_items`.

## Goals / Non-Goals

**Goals:**
- One click enqueues downloads for every not-yet-downloaded item in a collection or publisher
  group, reusing the existing per-item download entry point and queue — no new queue mechanics.
- Consistent skip behavior with the existing entry-level download action: already-downloaded,
  already-queued, and already-active items/files are left alone, not re-enqueued or duplicated.

**Non-Goals:**
- No new bulk-specific queue, priority tier, or cancellation-group concept. Each item's files
  still queue and cancel independently, exactly as they do today.
- No progress UI beyond what the existing activity panel already shows per enqueued file
  (`download-queue`'s "each download MUST have a named activity panel entry" requirement already
  covers this — a 40-item batch produces 40 (or fewer) activity entries, same as if the user had
  clicked Download 40 times).
- Not addressing `catalog-bulk-selection` (multi-select download action) — that's a different
  action (explicit selection) triggering a different, not-yet-implemented capability. This change
  only covers "download everything in this collection" / "download everything from this
  publisher," not arbitrary multi-select.

## Decisions

### Reuse `enqueue_download` per matching item; no new dedup logic

`LibraryController` gains two new methods:

```rust
pub fn download_all_for_collection(&mut self, collection_id: u64, cx: &mut Context<Self>)
pub fn download_all_for_publisher(&mut self, publisher: &str, cx: &mut Context<Self>)
```

Each collects the matching items' `(id, title)` pairs first (to avoid borrowing `self.catalog`
immutably while calling a `&mut self` method in the same loop), then calls
`self.enqueue_download(&id, title, cx)` once per item. Because `enqueue_download` already
no-ops on fully-downloaded items and `enqueue_item_download` already no-ops on
already-queued/in-flight files, no additional skip/dedup logic is needed in the new methods —
this mirrors exactly how the existing entry-level download action already behaves for a
multi-item entry, just fanned out across many entries instead of many files within one entry.

_Alternative considered_: Build a combined list of missing `(item_id, file_index)` pairs and
enqueue them directly via `enqueue_item_download`, skipping the per-item `enqueue_download` call.
Rejected — it duplicates logic `enqueue_download` already owns for no benefit, and diverges from
the single well-tested entry point every other download action goes through.

### Collection lookup: filter `self.catalog` by `member_ids_contain`, not the active sidebar filter

`download_all_for_collection` looks up the `CollectionEntry` by id from `self.collections`,
then filters `self.catalog` directly with `member_ids_contain`, rather than temporarily setting
`SidebarFilter::Collection` and reading back the filtered view. This is a pure filter-and-act
operation with no UI side effect (it must not change what's currently displayed in the catalog
just because the user downloaded a different collection than the one they're viewing).

### Publisher header context menu: new interaction on `render_group_header`

`render_group_header(publisher, count, colors)` currently renders a plain, non-interactive
`div()`. It gains a `.context_menu(...)` following the exact same `PopupMenuItem` /
`ContextMenuExt` pattern already used on collection sidebar rows and per-item catalog rows —
no new context-menu primitive, just a new attachment point. The publisher name (`Arc<str>`,
already owned by the header's caller) is captured into the closure alongside the controller
`Entity<LibraryController>`.

## Risks / Trade-offs

- [Risk] A publisher or collection with hundreds of items could enqueue hundreds of downloads
  at once, all waiting behind the shared `max_concurrent_downloads` limit. -> Mitigation: this
  is identical to the existing shared-queue behavior for any burst of downloads; the queue
  already handles arbitrary backlog sizes and the activity panel already scrolls
  (`activity-panel-improvements`). No new capacity concern introduced.
- [Risk] Publisher matching is an exact string match (`item.publisher == publisher`); if two
  differently-cased or whitespace-variant publisher strings exist for what a user considers "the
  same" publisher, some items could be silently excluded. -> Mitigation: this exactly matches
  today's existing grouping/filtering behavior (`group_by_publisher`, `SidebarFilter::Publisher`)
  — not a new inconsistency introduced by this change.
