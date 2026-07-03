## 1. Data model

- [x] 1.1 Add `thumbnail_last_attempted: Option<std::time::SystemTime>` field to `LibraryItem` in `dtrpg-ui/src/data/library.rs`
- [x] 1.2 Update all `LibraryItem` construction sites (map functions in `dtrpg-core/src/services/sdk.rs`) to set the new field to `None`

## 2. ActivityController extension

- [x] 2.1 Add `update_label(id: u64, label: impl Into<String>, cx: &mut Context<Self>)` to `ActivityController` in `dtrpg-ui/src/controllers/activity.rs`

## 3. LibraryController queue and fetch loop

- [x] 3.1 Add `thumbnail_queue: VecDeque<Arc<str>>`, `thumbnail_loading: bool`, and `thumbnail_activity_id: Option<u64>` fields to `LibraryController`
- [x] 3.2 Add `enqueue_thumbnails(items: &[LibraryItem], cx: &mut Context<Self>)` helper: for each item with `cover_url` not in `CoverCache` and not in-flight, push to `thumbnail_queue` and mark in-flight
- [x] 3.3 Add `drain_thumbnail_queue(&mut self, cx: &mut Context<Self>)` helper: if not already loading and queue non-empty, dequeue one URL, spawn the async fetch task, and set `thumbnail_loading = true`
- [x] 3.4 Implement the fetch task body: fetch `url`'s bytes on a background executor thread, on success insert bytes into `CoverCache`, clear in-flight, update `thumbnail_last_attempted` on the matching `LibraryItem`; on failure clear in-flight only; in both cases call `drain_thumbnail_queue` for the next item and update/complete the activity entry
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

## 8. Bug fix: thumbnails never actually loaded

- [x] 8.1 Root-caused why thumbnails never loaded in practice despite 1-7 being implemented:
      `drain_thumbnail_queue` (`dtrpg-ui/src/controllers/library.rs`) called
      `reqwest::get(&url_str).await` directly inside a `cx.spawn` future. `dtrpg-ui` does
      not depend on `tokio` and gpui's executors (`BackgroundExecutor`/`ForegroundExecutor`)
      are backed by a platform dispatcher (e.g. GCD on macOS), not a Tokio runtime — so the
      async reqwest client, which needs an active Tokio reactor to resolve DNS/open sockets,
      had nothing to run on. Every fetch failed immediately. `dtrpg-core`'s SDK gateway
      (`services/sdk.rs`, `services/login.rs`, `services/collections_sdk.rs`) avoids this by
      owning a dedicated `tokio::runtime::Runtime` and calling `.block_on(...)` from
      background-executor threads; the thumbnail loader had no equivalent.
- [x] 8.2 Fixed by switching the fetch to `reqwest::blocking::get(...)`, run inside
      `async_cx.background_executor().spawn(async move { ... })` — `reqwest`'s blocking
      client manages its own internal runtime per call and needs no ambient Tokio context,
      matching the pattern the SDK gateway already uses for calling network code from these
      same background-executor threads. The `blocking` reqwest feature was already enabled
      in the workspace `Cargo.toml` (added previously, seemingly in anticipation of this
      issue) but never wired into the actual call site until now.
- [ ] 8.3 Manually launch the app, sign in, and confirm thumbnails now actually appear on
      catalog items (not just that the queue drains without visible progress)

## 9. Bug fix: stale pre-existing disk cache silently disables thumbnail loading

- [x] 9.1 Root-caused a second, independent failure reported after 8.1/8.2 landed: on a
      machine with a catalog cache already on disk from before `cover_url` was populated
      correctly, every cached `LibraryItem` has `cover_url: null` (confirmed: 1890/1890 items
      in the reporter's local cache). `enqueue_thumbnails` correctly finds zero items with a
      `cover_url` to queue, so no fetch ever starts — this is not a bug in the queue/fetch code,
      it never receives data to act on. Compounding it: the auto-load policy in
      `start_load_inner` (`dtrpg-ui/src/controllers/library.rs`) skips the live API refetch
      whenever the cache is fresh (<7 days, see `catalog-auto-load-policy`) and the remote item
      count matches, so a fresh-but-schema-stale cache is served indefinitely with no live
      refetch to ever pick up `cover_url` — and thus no visible error, activity entry, or any
      other sign that thumbnails should be loading at all.
- [x] 9.2 Fixed by adding `schema_version: u32` to `CacheMetadata`
      (`dtrpg-ui/src/data/catalog_cache.rs`), bumped to `2`, and folding a version check into
      `is_stale()`. Caches written before this field existed deserialize `schema_version` as `0`
      via `#[serde(default)]`, which never matches the current version and is always treated as
      stale — forcing a live refetch on next launch regardless of age or item-count match. Added
      regression tests: a fresh cache with a stale schema version must report `is_stale() ==
      true`, and JSON without a `schema_version` key must default to `0` and be stale.
- [x] 9.3 Deleted the reporter's stale local cache
      (`~/Library/Caches/com.pilgrimagesoftware.dtrpg/metadata/catalog_cache*.json`) so their
      next launch does a live refetch under the new schema version — this was a one-time local
      fix; 9.2 prevents recurrence for this and any other existing install.
- [ ] 9.4 Manually launch the app and confirm: (a) with the reporter's now-deleted cache, a live
      refetch happens and thumbnails begin loading; (b) with a freshly-saved cache from this
      build, a second launch within 7 days correctly skips the live refetch (schema version now
      matches, so the auto-load optimization still works for genuinely current caches)
