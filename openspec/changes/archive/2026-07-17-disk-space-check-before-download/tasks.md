## 1. Free Disk Space Query

- [x] 1.1 Check crates.io for the current stable version of `fs4`, add it as a `dtrpg-ui` dependency with `default-features = false` and only the features needed for `available_space`
- [x] 1.2 Add `pub fn available_space_mb() -> Option<f64>` to `dtrpg-ui/src/data/storage.rs`, calling `fs4::available_space(&StorageConfig::load().root_path())` and converting bytes to megabytes; return `None` on any I/O error

## 2. Aggregate Size Calculation

- [x] 2.1 Add a helper (e.g. `fn missing_files_size_mb(files: &[LibraryItemFile]) -> f64`) summing `size_mb` for files not yet downloaded, reusing the existing `missing_file_indices` selection logic
- [x] 2.2 Add a helper summing missing-file size across a set of catalog items (for collection/publisher bulk targets), reusing `collection_download_targets`/`publisher_download_targets` to resolve the target item set

## 3. Pending Request State and Event

- [x] 3.1 Add `enum PendingDownloadRequest { Item { id: Arc<str>, title: String }, ItemFile { id: Arc<str>, index: u32, title: String }, Collection { collection_id: u64 }, Publisher { publisher: Arc<str> } }` and a `pending_download: Option<PendingDownloadRequest>` field on `LibraryController`
- [x] 3.2 Add `pub struct LowDiskSpaceWarning { pub needed_mb: f64, pub free_mb: f64 }` and `impl EventEmitter<LowDiskSpaceWarning> for LibraryController` to `dtrpg-ui/src/data/events.rs`

## 4. Gating Wrapper Methods

- [x] 4.1 Add `pub fn request_download(&mut self, id: &str, title: impl Into<String>, cx: &mut Context<Self>)`: compute the entry's missing-file size, compare to `available_space_mb()`; if space is sufficient or unknown, call `enqueue_download` directly; otherwise stash `PendingDownloadRequest::Item` and emit `LowDiskSpaceWarning`
- [x] 4.2 Add `pub fn request_item_download(&mut self, id: &str, index: u32, title: impl Into<String>, cx: &mut Context<Self>)`: same pattern for a single file, wrapping `enqueue_item_download`
- [x] 4.3 Add `pub fn request_download_all_for_collection(&mut self, collection_id: u64, cx: &mut Context<Self>)`: same pattern over the collection's aggregate missing-file size, wrapping `download_all_for_collection`
- [x] 4.4 Add `pub fn request_download_all_for_publisher(&mut self, publisher: &str, cx: &mut Context<Self>)`: same pattern over the publisher's aggregate missing-file size, wrapping `download_all_for_publisher`
- [x] 4.5 Add `pub fn confirm_pending_download(&mut self, cx: &mut Context<Self>)`: match on `self.pending_download.take()` and call the corresponding unconditional method directly
- [x] 4.6 Add `pub fn cancel_pending_download(&mut self, cx: &mut Context<Self>)`: clear `self.pending_download` with no further action

## 5. UI Wiring

- [x] 5.1 In `dtrpg-ui/src/ui/views/root_view.rs`, add a `cx.subscribe_in(&controller, window, ...)` handler for `LowDiskSpaceWarning` that opens a `window.open_alert_dialog(...).confirm()` dialog with title/description built from `needed_mb`/`free_mb`, `.on_ok` calling `ctrl.confirm_pending_download(cx)`, `.on_cancel` calling `ctrl.cancel_pending_download(cx)`
- [x] 5.2 Update the `enqueue_download` call sites in `item_popover_view.rs`, `detail_panel_view.rs` (line ~280), and the four sites in `catalog_view.rs` to call `request_download` instead
- [x] 5.3 Update the direct `enqueue_item_download` call site in `detail_panel_view.rs` (line ~617) to call `request_item_download` instead
- [x] 5.4 Update the collection/publisher "Download All" action call sites (wherever `download_all_for_collection`/`download_all_for_publisher` are currently invoked from the UI — collection and publisher context menus) to call `request_download_all_for_collection`/`request_download_all_for_publisher` instead

## 6. Verification

- [x] 6.1 Run `cargo check -p dtrpg-ui` and confirm zero errors
- [x] 6.2 Run `cargo test -p dtrpg-ui` and confirm all existing tests pass; add unit tests for the size-aggregation helpers (missing-file sum, multi-item sum) and for `PendingDownloadRequest` confirm/cancel clearing state correctly
- [x] 6.3 Run the app; trigger an entry-level download where the entry's total size exceeds available free space (e.g. temporarily point the storage root at a near-full or small test volume); confirm the warning dialog appears before anything downloads
- [x] 6.4 Confirm the dialog and verify the download proceeds and completes normally
- [x] 6.5 Cancel the dialog and verify nothing was queued
- [x] 6.6 Repeat for a collection's and a publisher's "Download All" action, confirming the check is evaluated once as an aggregate, not per item
- [x] 6.7 Confirm a download well within available free space queues immediately with no dialog
