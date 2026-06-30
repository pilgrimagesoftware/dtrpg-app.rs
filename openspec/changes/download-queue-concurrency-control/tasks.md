## 1. Storage Config and Settings

- [ ] 1.1 Add `max_concurrent_downloads: usize` field (default 3) to `StorageConfig` in `data/storage.rs`, with TOML serialization
- [ ] 1.2 Add `max_concurrent_downloads` to `SettingsSnapshot` and expose it from `SettingsController::snapshot()`
- [ ] 1.3 Add `set_max_concurrent_downloads(n, cx)` to `SettingsController`; persist to `StorageConfig`
- [ ] 1.4 Add a numeric stepper or text input for the concurrency limit in `settings_storage_view.rs`

## 2. Concurrent Thumbnail Queue

- [ ] 2.1 Replace `thumbnail_loading: bool` with `active_thumbnail_fetches: usize` in `LibraryController`
- [ ] 2.2 Update `drain_thumbnail_queue` to dispatch up to `max_concurrent_downloads - active_downloads` slots concurrently
- [ ] 2.3 Decrement `active_thumbnail_fetches` and re-drain on each thumbnail task completion or error
- [ ] 2.4 Verify priority promotion (`push_front`) still works correctly with the concurrent drainer

## 3. Download Queue Infrastructure

- [ ] 3.1 Add `download_queue: VecDeque<(Arc<str>, String)>` (item id, title) and `active_downloads: usize` to `LibraryController`
- [ ] 3.2 Replace `toggle_download` with `enqueue_download(id, title, cx)` that pushes to the queue and drains
- [ ] 3.3 Add `drain_download_queue` that dispatches up to `max_concurrent_downloads - active_thumbnail_fetches` downloads
- [ ] 3.4 On download task completion, decrement `active_downloads`, update item status to Downloaded, complete activity entry, re-drain
- [ ] 3.5 On download task error, decrement `active_downloads`, revert item status to Cloud, error activity entry, re-drain

## 4. Activity Panel Integration

- [ ] 4.1 Call `activity.start(title, cx)` when a download slot is taken, storing the returned activity ID alongside the task
- [ ] 4.2 Call `activity.complete(id, cx)` on successful download completion
- [ ] 4.3 Call `activity.error(id, message, cx)` on download failure

## 5. Cancel Support

- [ ] 5.1 Add `cancel_download(id, cx)` to `LibraryController`: removes from queue if pending, or signals abort if active
- [ ] 5.2 Wire cancel action through the UI (detail panel or activity panel entry)

## 6. Shared Slot Accounting

- [ ] 6.1 Ensure `drain_thumbnail_queue` and `drain_download_queue` both read `active_thumbnail_fetches + active_downloads` against `max_concurrent_downloads` to enforce the aggregate limit
- [ ] 6.2 Add a unit test verifying that total active fetches never exceeds the configured limit when both queues are non-empty

## 7. Verification

- [ ] 7.1 Build passes with no new clippy warnings
- [ ] 7.2 Launch app and confirm thumbnails load concurrently in grid view
- [ ] 7.3 Confirm concurrency setting persists across app restarts
- [ ] 7.4 Confirm activity panel shows a named entry for each enqueued download
