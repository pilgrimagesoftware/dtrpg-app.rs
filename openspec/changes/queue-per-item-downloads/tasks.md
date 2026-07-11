## 1. Data model

- [x] 1.1 Add `pub downloaded: bool` to `LibraryItemFile` (`crates/dtrpg-ui/src/data/library.rs`)
      with `#[serde(default)]` for cache backward-compatibility.
- [x] 1.2 Add a `recompute_entry_status(item: &mut LibraryItem)` helper (or equivalent) that sets
      `item.status = Downloaded` iff every `item.files[i].downloaded` is true, else `Cloud`.

## 2. Controller: re-key the download queue per file

- [ ] 2.1 Change `download_queue: VecDeque<(Arc<str>, String)>` to
      `VecDeque<(Arc<str>, u32, String)>` (entry id, file index, label) in
      `crates/dtrpg-ui/src/controllers/library.rs`.
- [ ] 2.2 Change `download_cancel_flags: HashMap<Arc<str>, Arc<AtomicBool>>` to key on
      `(Arc<str>, u32)`.
- [ ] 2.3 Change `download_activity_ids: HashMap<Arc<str>, u64>` to key on `(Arc<str>, u32)`.
- [ ] 2.4 Update `enqueue_download` to accept a file index, dedupe against the queue/active maps
      using the `(entry_id, index)` key, and skip enqueueing if that specific file is already
      downloaded.
- [ ] 2.5 Update `remove_download`/cancel path to take `(entry_id, index)`, only touching that
      file's queue entry, cancel flag, and activity id.
- [ ] 2.6 Update `dispatch_download` to take the file index, resolve `item.files[index]` (not
      `.first()`) for the SDK `download_item` call, and label the activity entry with both entry
      title and file name.
- [ ] 2.7 On successful completion, set `item.files[index].downloaded = true` and call
      `recompute_entry_status`; on failure/cancellation leave it false and recompute.
- [ ] 2.8 Update `drain_download_queue` for the new tuple shape; concurrency slot accounting
      (`active_downloads`, `available_slots`) stays entry-agnostic (design's stated non-goal).

## 3. Entry-level download action enqueues all missing items

- [ ] 3.1 Update the entry-level download button's click handler
      (`crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`, `detail-tab-download`) to iterate
      `item.files`, enqueueing every file where `!downloaded` via the updated
      `enqueue_download(entry_id, index, label, cx)`.
- [ ] 3.2 Update the button's label/icon/enabled state to reflect aggregate status per
      design.md's "Rust entry-level download status MUST reflect aggregate per-item state"
      (fully downloaded only when every file is downloaded).
- [ ] 3.3 Update `catalog_view.rs`'s entry-level status affordances (list row badge, grid tile
      badge, context menu Download/Downloaded items) that read `item.status` directly — confirm
      they still read the (now-derived) `item.status` and require no further change, or adjust
      if any read `item.files` directly.

## 4. Per-item download button in the detail tab item list

- [ ] 4.1 In `render_item_tier`'s per-row `Status` column (currently a `TODO` placeholder),
      replace it with a download action/status affordance: download icon when
      `!file.downloaded` and not queued/active, checkmark when `file.downloaded`, a neutral
      in-progress indicator when queued/active (no duplicate cancel control on the row — cancel
      stays in the activity panel).
- [ ] 4.2 Wire the row's download icon's `on_click` to `enqueue_download(entry_id, row_ix, ...)`
      with `cx.stop_propagation()` so it doesn't also fire the row's existing
      `select_item_file` click handler (same class of bug fixed in `activity_panel_view.rs`'s
      cancel button — see prior session).
- [ ] 4.3 Determine each row's queued/active state by checking the controller's
      `download_queue`/`download_cancel_flags` for `(entry_id, row_ix)` membership.

## 5. Tests

- [ ] 5.1 Unit test: enqueueing a multi-item entry's download queues one entry per file.
- [ ] 5.2 Unit test: enqueueing when one file is already downloaded queues only the remaining
      files.
- [ ] 5.3 Unit test: cancelling one file's download leaves a sibling file's queue/active state
      untouched.
- [ ] 5.4 Unit test: `recompute_entry_status` reports `Downloaded` only when all files are
      downloaded, `Cloud` otherwise (including zero-downloaded and partially-downloaded cases).
- [ ] 5.5 Unit test: completing the last remaining file's download transitions the entry from
      Cloud to Downloaded.

## 6. Verification

- [ ] 6.1 `cargo build --workspace --all-features`
- [ ] 6.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 6.3 `cargo test --workspace --all-features`
- [ ] 6.4 Launch app: download a multi-item entry from the entry-level button, confirm every item
      downloads and each gets its own activity panel entry.
- [ ] 6.5 Launch app: download a single item from the detail tab's item list independent of its
      siblings, confirm only that item downloads.
