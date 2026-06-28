## 1. Data model

- [x] 1.1 Add `thumbnail_last_attempted: Option<std::time::SystemTime>` field to `LibraryItem` in `dtrpg-ui/src/data/library.rs`
- [x] 1.2 Update all `LibraryItem` construction sites (map functions in `dtrpg-core/src/services/sdk.rs`) to set the new field to `None`

## 2. ActivityController extension

- [x] 2.1 Add `update_label(id: u64, label: impl Into<String>, cx: &mut Context<Self>)` to `ActivityController` in `dtrpg-ui/src/controllers/activity.rs`

## 3. LibraryController queue and fetch loop

- [x] 3.1 Add `thumbnail_queue: VecDeque<Arc<str>>`, `thumbnail_loading: bool`, and `thumbnail_activity_id: Option<u64>` fields to `LibraryController`
- [x] 3.2 Add `enqueue_thumbnails(items: &[LibraryItem], cx: &mut Context<Self>)` helper: for each item with `cover_url` not in `CoverCache` and not in-flight, push to `thumbnail_queue` and mark in-flight
- [x] 3.3 Add `drain_thumbnail_queue(&mut self, cx: &mut Context<Self>)` helper: if not already loading and queue non-empty, dequeue one URL, spawn the async fetch task, and set `thumbnail_loading = true`
- [x] 3.4 Implement the async fetch task body: call `reqwest::get(url)`, on success insert bytes into `CoverCache`, clear in-flight, update `thumbnail_last_attempted` on the matching `LibraryItem`; on failure clear in-flight only; in both cases call `drain_thumbnail_queue` for the next item and update/complete the activity entry
- [x] 3.5 Call `enqueue_thumbnails` then `drain_thumbnail_queue` at the end of `append_catalog_page`

## 4. Add reqwest dependency

- [x] 4.1 Add `reqwest` with `rustls-tls` feature to `dtrpg-ui/Cargo.toml` (or `dtrpg-core/Cargo.toml` if the fetch is extracted there)

## 5. Context menu in catalog views

- [x] 5.1 Add a `load_thumbnail(cover_url: Arc<str>, cx: &mut Context<LibraryController>)` method to `LibraryController` that enqueues the given URL at the front of the queue and calls `drain_thumbnail_queue`
- [x] 5.2 Add a context menu to the list layout in `catalog_view.rs` using `DropdownMenu` / `PopupMenuItem::new("Load Thumbnail")` with a disabled state when the cooldown guard applies
- [x] 5.3 Add the same context menu to the thumbs layout
- [x] 5.4 Add the same context menu to the grid layout

## 6. Cooldown guard in context menu

- [x] 6.1 Implement a helper `fn thumbnail_cooldown_elapsed(item: &LibraryItem) -> bool` that returns `true` if `thumbnail_last_attempted` is `None` or older than 5 minutes (handle clock skew via `checked_duration_since`)
- [x] 6.2 Wire the helper into all three context menu render sites so the "Load Thumbnail" item is disabled when the guard returns `false`

## 7. Verify

- [x] 7.1 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 7.2 Manually launch the app and confirm thumbnails begin loading as catalog pages arrive, with the activity panel showing the aggregated label
- [ ] 7.3 Confirm the context menu appears on right-click in all three catalog layouts
- [ ] 7.4 Confirm the "Load Thumbnail" item is disabled within 5 minutes of a prior attempt and enabled after
