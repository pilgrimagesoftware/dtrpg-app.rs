## 1. Data model

- [x] 1.1 Add `pub downloaded: bool` to `LibraryItemFile` (`crates/dtrpg-ui/src/data/library.rs`)
      with `#[serde(default)]` for cache backward-compatibility.
- [x] 1.2 Add a `recompute_entry_status(item: &mut LibraryItem)` helper (or equivalent) that sets
      `item.status = Downloaded` iff every `item.files[i].downloaded` is true, else `Cloud`.

## 2. Controller: re-key the download queue per file

- [x] 2.1 Change `download_queue: VecDeque<(Arc<str>, String)>` to
      `VecDeque<(Arc<str>, u32, String)>` (entry id, file index, label) in
      `crates/dtrpg-ui/src/controllers/library.rs`.
- [x] 2.2 Change `download_cancel_flags: HashMap<Arc<str>, Arc<AtomicBool>>` to key on
      `(Arc<str>, u32)`.
- [x] 2.3 Change `download_activity_ids: HashMap<Arc<str>, u64>` to key on `(Arc<str>, u32)`.
- [x] 2.4 Added a new `enqueue_item_download(id, index, title, cx)` taking a file index, deduping
      against the queue/active maps using the `(entry_id, index)` key, skipping if that specific
      file is already downloaded. `enqueue_download(id, title, cx)` keeps its original signature
      (all 6 existing call sites unchanged) and now loops every not-yet-downloaded file, calling
      `enqueue_item_download` per file — this doubles as task 3.1's entry-level "enqueue all
      missing items" behavior.
- [x] 2.5 `cancel_download` now takes `(id, index)`, only touching that file's queue entry,
      cancel flag, and activity id. `remove_download` (entry-level "remove all downloads") clears
      every file's `downloaded` flag then calls `recompute_status` rather than setting
      `item.status` directly.
- [x] 2.6 `dispatch_download` now takes the file index, resolves `item.files[index]` (not
      `.first()`) for the SDK `download_item` call, and labels the activity entry with both entry
      title and file name (`"Downloading {title} — {file_name}..."`).
- [x] 2.7 On successful completion, sets `item.files[index].downloaded = true` and calls
      `item.recompute_status()`; on failure/cancellation leaves it false and still recomputes.
- [x] 2.8 `drain_download_queue` updated for the new tuple shape; concurrency slot accounting
      (`active_downloads`, `available_slots`) stays entry-agnostic (design's stated non-goal).

## 3. Entry-level download action enqueues all missing items

- [x] 3.1 No click-handler change needed: `detail-tab-download`'s existing call to
      `enqueue_download(&id, download_title.clone(), cx)` (`detail_panel_view.rs`) already
      enqueues every not-yet-downloaded file now that `enqueue_download` loops `item.files`
      internally (see 2.4) — the view didn't need to know about indices.
- [x] 3.2 No further change needed: the button's `is_downloaded = item.status ==
      ItemStatus::Downloaded` check already reflects the design's aggregate rule, since
      `item.status` is now derived by `recompute_status` (all files downloaded) rather than set
      directly.
- [x] 3.3 Confirmed: `catalog_view.rs`'s list row badge, grid tile badge, and context menu
      Download/Downloaded items all read `item.status` directly and need no change — they
      automatically reflect the new aggregate-derived status.

## 4. Per-item download button in the detail tab item list

- [x] 4.1 `render_item_tier`'s per-row `Status` column (previously a `TODO` placeholder) now
      shows a download icon when `!file.downloaded` and not queued/active, a checkmark when
      `file.downloaded`, or a neutral "Downloading…" text indicator when queued/active (no
      duplicate cancel control on the row — cancel stays in the activity panel).
- [x] 4.2 Wired the row's download icon's `on_click` to `enqueue_item_download(entry_id, row_ix,
      title, cx)` with `cx.stop_propagation()` so it doesn't also fire the row's existing
      `select_item_file` click handler (same class of bug fixed in `activity_panel_view.rs`'s
      cancel button — see prior session).
- [x] 4.3 Added `LibraryController::is_file_queued_or_active(id, index)`, checking
      `download_queue` and `download_activity_ids` for `(entry_id, row_ix)` membership; used by
      the row to pick its downloaded/downloading/download-action state.

## 5. Tests

Extracted the pure decision logic behind the GPUI-context-dependent controller methods into free
functions (`missing_file_indices`, `dequeue_file`), matching this file's existing pattern of
testing free functions rather than `Context<Self>`-bound methods directly (no `TestAppContext`
precedent exists in this crate).

- [x] 5.1 `missing_file_indices_returns_every_index_when_none_downloaded` — enqueueing a
      multi-item entry's download queues one index per file (`enqueue_download` maps each
      returned index to one `download_queue` entry).
- [x] 5.2 `missing_file_indices_skips_already_downloaded_files` — enqueueing when some files are
      already downloaded queues only the remaining ones.
- [x] 5.3 `dequeue_file_removes_only_the_matching_entry` /
      `dequeue_file_leaves_other_entries_untouched` — cancelling one file's queued download
      leaves a sibling file's queue entry untouched.
- [x] 5.4 `recompute_status_is_cloud_when_no_files_downloaded` /
      `recompute_status_is_cloud_when_only_some_files_downloaded` /
      `recompute_status_is_downloaded_once_every_file_is_downloaded` /
      `recompute_status_is_cloud_for_an_entry_with_no_files` — `LibraryItem::recompute_status`
      reports `Downloaded` only when all files are downloaded, `Cloud` otherwise.
- [x] 5.5 `recompute_status_completing_the_last_file_flips_cloud_to_downloaded` — completing the
      last remaining file transitions the entry from Cloud to Downloaded.

## 6. Verification

- [x] 6.1 `cargo build --workspace --all-features` — clean.
- [x] 6.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings` — clean.
- [x] 6.3 `cargo test --workspace --all-features` — 221 unit tests + 11 doc-tests, all pass.
- [x] 6.4 Launch app: download a multi-item entry from the entry-level button, confirm every item
      downloads and each gets its own activity panel entry.
- [x] 6.5 Launch app: download a single item from the detail tab's item list independent of its
      siblings, confirm only that item downloads.
