## Context

The download queue (`crates/dtrpg-ui/src/controllers/library.rs`) and the SDK-backed fetch
(`crates/dtrpg-core/src/services/sdk/library/download.rs`, from the not-yet-merged
`implement-real-file-downloads` change) are both already file-scoped at the lowest level —
`LibraryService::download_item(order_product_id, index, dest, cancel)` fetches exactly one file.
The gap is entirely in the controller layer above it: `dispatch_download` always resolves
`item.files.first()`, and every piece of in-flight-download bookkeeping
(`download_queue`, `download_cancel_flags`, `download_activity_ids`) is keyed on the entry's
`Arc<str>` id alone, so there is no way to represent "two different files of the same entry are
both downloading right now."

`LibraryItem.status: ItemStatus` (`Downloaded` | `Cloud`) is entry-level only. Every mapped item
starts `Cloud` (`mapping.rs`); nothing re-checks the filesystem — status is tracked purely as
in-memory/cached state, set to `Downloaded` on a successful `dispatch_download` completion and
back to `Cloud` on `remove_download`. 37 call sites across `catalog_view.rs`, `library.rs`, and
sidebar filtering read this binary status directly (list/grid badges, context menu items, the
Downloaded/Cloud sidebar section counts, Read/Reveal button enablement).

## Goals / Non-Goals

**Goals:**
- Let the entry-level download button enqueue every not-yet-downloaded item in a multi-item
  entry as independent, separately cancellable downloads.
- Give each row in the detail tab's item list its own download button and status affordance.
- Keep the 37 existing `ItemStatus` call sites working unchanged in spirit: "Downloaded" still
  means "fully available offline," "Cloud" still means "not yet." Filtering/sidebar semantics do
  not change.

**Non-Goals:**
- Persisting or resuming downloads across app restarts (unchanged from today).
- Changing how single-item entries behave — with one file, "download the entry" and "download
  the item" stay the same action end to end.
- Building a general N-state status enum. Per-item detail is additive, not a replacement for the
  binary entry-level status.

## Decisions

### Per-file download key: `(Arc<str>, u32)` (entry id, file index)

`download_queue`, `download_cancel_flags`, and `download_activity_ids` change from
`HashMap<Arc<str>, _>` / `VecDeque<(Arc<str>, String)>` to keying on `(Arc<str>, u32)`, where the
`u32` is the file's position in `item.files` — the same index `LibraryItemFile::index` already
carries for the SDK's `prepare_download` call. Reusing that field rather than minting a new
per-file id keeps the queue key and the SDK call argument identical, so `dispatch_download` no
longer needs to re-derive which file it's fetching from a lookup.

Alternative considered: synthesize a per-file `Arc<str>` id (e.g. `"{entry_id}#{index}"`) and
keep single-key maps. Rejected — it adds a string-parsing round trip everywhere the entry id is
needed (cancel-flag cleanup, activity correlation, catalog lookup) for no benefit over a plain
tuple key, which `HashMap`/`VecDeque` support natively.

### Per-file download status lives on `LibraryItemFile`, entry status is derived

Add `pub downloaded: bool` to `LibraryItemFile`. `LibraryItem.status` stops being written
directly by `dispatch_download`/`remove_download`; instead both call a new
`recompute_entry_status(item)` helper that sets `status = Downloaded` iff every file in
`item.files` has `downloaded == true`, else `Cloud`. This is a source-of-truth change, not a
type change — `ItemStatus` keeps its existing two variants, so none of the 37 existing call sites
(sidebar counts, badges, context menus, Read/Reveal gating) need to change.

Alternative considered: add `ItemStatus::PartiallyDownloaded`. Rejected — it would ripple through
every exhaustive `match` on `ItemStatus` (list/grid icon, context menu, sidebar filter counts) for
a distinction that only two call sites actually need (the entry-level download button's
label/icon, and the new item-count/status badge). Those two call sites can compute
"some-but-not-all downloaded" locally from `item.files` without widening the shared enum.

### Entry-level download button enqueues per-file, skipping already-downloaded items

The button's click handler iterates `item.files`, enqueuing `(entry_id, idx)` for every file
where `!file.downloaded` and it isn't already queued/active (existing `already_pending` check,
now keyed per-file). Files already downloaded are left alone — clicking "Download" on a bundle
that has 1 of 2 items already downloaded queues only the missing one, matching the existing
single-item "don't re-download what's already there" behavior.

### Activity panel label includes the file name

Per-file activity labels become `"Downloading {entry_title} — {file_name}…"` instead of
`"Downloading {title}…"`, so two concurrent downloads from the same entry are distinguishable in
the activity panel. `download_activity_ids` moving to a `(Arc<str>, u32)` key means multiple
activity entries can coexist for the same entry without collision (today a second `start()` call
for the same entry id would silently overwrite the first's tracked activity id).

### Item-row download button reuses the entry-level button's click wiring, scoped to one file

`render_item_tier`'s per-row `Status` column (currently a `TODO` placeholder, see
`detail_panel_view.rs`) gets a small icon-button matching `detail-tab-download`'s existing
states (not-downloaded → download icon; downloaded → check icon, click removes; in-progress →
handled by the activity panel, row shows a neutral "queued/downloading" indicator rather than
duplicating cancel — cancelling stays a single, consistent action in the activity panel to avoid
two different cancel affordances for the same download).

## Risks / Trade-offs

- **[Risk]** Re-keying three maps and the queue touches most of `dispatch_download`,
  `enqueue_download`, `remove_download`, and `drain_download_queue` at once → **Mitigation**:
  no behavior change for single-item entries (index is always `0`), so existing single-item
  download/cancel tests continue to exercise the same code paths with an added, constant index
  component; add new tests specifically for multi-item enqueue/cancel/complete interleaving.
- **[Risk]** `available_slots`/`max_concurrent_downloads` accounting is entry-agnostic today
  (`active_downloads: usize`); queuing N items from one entry at once could let one entry consume
  the whole concurrency budget → **Mitigation**: out of scope for this change — existing queue
  behavior already lets any mix of entries fill all slots; per-entry fairness is a separate,
  future concern, called out as an open question below.
- **[Risk]** Cached `LibraryItem`/`LibraryItemFile` JSON on disk predates the new `downloaded`
  field → **Mitigation**: `#[serde(default)]` on `LibraryItemFile::downloaded`, defaulting to
  `false`; existing entries whose `status` was already `Downloaded` under the old model will
  re-derive as `Cloud` for their (default-false) files until the next full catalog refresh
  re-populates state — acceptable since the app already treats the cache as best-effort.

## Open Questions

- Should the entry-level download button's tooltip/label distinguish "download all N items" from
  "download the 2 remaining items" when some are already present? Left to implementation —
  functionally both enqueue only the missing files either way.
- Per-entry fairness in the concurrency slot allocator (so one 10-item bundle can't starve other
  entries' downloads) is explicitly out of scope; flagged for a follow-up change if it proves to
  be a real-world problem.
