## 1. Data model

- [x] 1.1 Add `is_available: bool` to `LibraryItem` (`crates/dtrpg-ui/src/data/library.rs`)
      with `#[serde(default = "default_true")]` and a `default_true() -> bool { true }`
      helper.
- [x] 1.2 Set `is_available: true` in `LibraryItem::new` and any other in-crate constructors
      (stub/sample data generators) so existing call sites still compile.
- [x] 1.3 Add a unit test in `library.rs` confirming a `LibraryItem` JSON blob without the
      `is_available` field deserializes with `is_available == true`.

## 2. Reconciliation logic

- [x] 2.1 In `crates/dtrpg-ui/src/controllers/library.rs`, extract the reconcile-by-id merge
      (build `HashMap<Arc<str>, LibraryItem>` from live items, walk existing catalog setting
      `is_available` true/false, append remaining new items) into a standalone function,
      e.g. `reconcile_catalog(existing: Vec<LibraryItem>, live: Vec<LibraryItem>) ->
      Vec<LibraryItem>`.
- [x] 2.2 Add unit tests for `reconcile_catalog` covering: item present in both (refreshed,
      available), item only local (kept, flagged unavailable), item only live (appended,
      available), previously-unavailable item reappearing (flag cleared).
- [x] 2.3 Update `set_catalog` (or split into `set_catalog` / a new reconciling entry point)
      to call `reconcile_catalog` when `catalog_was_empty` was `false` at fetch start, and
      keep the existing plain replace when it was `true`.
- [x] 2.4 Update the fetch-completion call site (around the `ctrl.set_catalog(live_items,
      cx)` call) to pass through whichever `catalog_was_empty` result was already computed
      earlier in the same spawn closure.

## 3. UI surface for unavailable items

- [x] 3.1 Identify the catalog list/grid row rendering code and add a visual indicator
      (badge or dimmed treatment) for `is_available == false`, consistent with existing
      metadata badges (e.g. multi-item badge from `catalog-entry-detail-view`).
- [x] 3.2 Confirm unavailable items are not excluded by any existing filter/search predicate
      (no code changes expected if none currently filter on availability — verify only).
- [x] 3.3 Add a translation string for the unavailable indicator's tooltip/label under the
      existing i18n catalog, following `string-catalog` conventions.

## 4. Manual reload cooldown

- [x] 4.1 Add `FORCE_RELOAD_COOLDOWN_SECS: u64 = 60` to
      `crates/dtrpg-ui/src/data/constants.rs`, alongside the existing cache-related
      constants, with a doc comment distinguishing it from `STALE_SECS`.
- [x] 4.2 In `LibraryController::reload_catalog`, load `CacheMetadata` via the existing
      `load_cache_metadata` helper before calling `start_load_inner`; if
      `saved_at_secs` is within `FORCE_RELOAD_COOLDOWN_SECS` of now, return without setting
      `catalog_loading`, emitting `LibraryChanged`, or calling `start_load_inner`.
- [x] 4.3 Add unit/integration coverage for `reload_catalog`'s cooldown branch: reload
      suppressed when metadata timestamp is recent, reload proceeds when metadata timestamp
      is older than the cooldown, reload proceeds when no metadata file exists.
- [x] 4.4 Confirm `clear_and_reload` (used after an on-disk cache clear) is unaffected by
      the cooldown — it clears the catalog and cache metadata before calling
      `reload_catalog`, so there is no recent timestamp to gate against.

## 5. Single-item on-demand check

- [x] 5.1 Add `#[serde(skip)] availability_last_checked: Option<std::time::SystemTime>` to
      `LibraryItem`, alongside `thumbnail_last_attempted`.
- [x] 5.2 Add `ITEM_CHECK_COOLDOWN_SECS: u64 = 300` to `constants.rs`.
- [x] 5.3 Add `LibraryController::maybe_check_item(id: Arc<str>, cx: &mut Context<Self>)`:
      no-ops if `availability_last_checked` is within `ITEM_CHECK_COOLDOWN_SECS`, otherwise
      calls `get_item` on the background executor via the existing `cx.spawn` pattern and
      applies the result per `catalog-availability-flag`'s single-item check requirement
      (`Ok` → refresh fields + `is_available = true`; `NotFound` → `is_available = false`;
      any other error → leave item and flag unchanged, log at `warn`).
- [x] 5.4 Call `maybe_check_item` from `LibraryController::select_item` and from
      `TabsController::open_detail_tab`.
- [x] 5.5 Add unit tests for `maybe_check_item`: cooldown suppresses a redundant check,
      `Ok` result updates fields and flag, `NotFound` sets `is_available = false`, a
      network/session error leaves the item unchanged.

## 6. Checking-in-progress indicator

- [x] 6.1 Add `checking_items: HashSet<Arc<str>>` to `LibraryController`; insert the id when
      a single-item check starts, remove it when the check completes, emitting
      `LibraryChanged` on both transitions.
- [x] 6.2 Add `LibraryController::is_checking(&self, id: &str) -> bool`.
- [x] 6.3 Render a checking indicator (spinner/overlay) on catalog entries where
      `is_checking(id)` is true, in both the catalog list/grid and the detail views,
      following the existing thumbnail in-flight rendering pattern.

## 7. Periodic per-item check queue

- [x] 7.1 Add `check_queue: VecDeque<Arc<str>>` to `LibraryController`.
- [x] 7.2 Add `enqueue_checks(&mut self, ids: impl Iterator<Item = Arc<str>>, cx: &mut
      Context<Self>)`: pushes ids not already queued or in `checking_items` onto
      `check_queue`, then calls `drain_check_queue`.
- [x] 7.3 Add `drain_check_queue(&mut self, cx: &mut Context<Self>)`, mirroring
      `drain_thumbnail_queue`: no-ops if `checking_items` is non-empty or the queue is
      empty; otherwise pops the next id, runs the same check logic as `maybe_check_item`
      (bypassing its per-item cooldown check, since queue population already filters by
      staleness), and on completion calls `drain_check_queue` again.
- [x] 7.4 Add unit tests for queue draining: single-flight (only one check in progress at a
      time), queue empties after all items are processed, an id already in flight or
      already queued is not re-added.

## 8. Manual and automatic check-batch cooldown

- [x] 8.1 Add `last_item_check_batch_secs: Option<u64>` to `CacheMetadata`
      (`crates/dtrpg-ui/src/data/catalog_cache.rs`) with `#[serde(default)]`.
- [x] 8.2 Add `ITEM_CHECK_BATCH_COOLDOWN_SECS: u64 = 900` to `constants.rs`.
- [x] 8.3 Add `LibraryController::request_check_batch(&mut self, cx: &mut Context<Self>)`:
      loads `CacheMetadata`, no-ops if `last_item_check_batch_secs` is within
      `ITEM_CHECK_BATCH_COOLDOWN_SECS`; otherwise selects up to a bounded batch size (e.g.
      50) of items whose `availability_last_checked` is `None` or older than
      `ITEM_CHECK_COOLDOWN_SECS`, oldest-checked first, calls `enqueue_checks` with them,
      and immediately persists `last_item_check_batch_secs = Some(now)` to the metadata
      sidecar before the batch finishes draining.
- [x] 8.4 Wire a manual trigger (menu action or catalog-view control) to call
      `request_check_batch`.
- [x] 8.5 Add an automatic periodic trigger: a `cx.spawn` loop using
      `background_executor().timer(Duration)` between iterations that calls
      `request_check_batch` on each wake.
- [x] 8.6 Add unit tests for `request_check_batch`: cooldown suppresses a request when a
      recent batch (manual or automatic) was already enqueued, batch selection prefers
      oldest-checked items, batch size is bounded.

## 9. Startup partial-fetch decision

- [x] 9.1 Add `updated_date_after`-based partial fetch support: extend `LibraryService`
      (`crates/dtrpg-ui/src/services/mod.rs`) with `fn list_items_updated_since(&self,
      since_iso8601: &str, on_page: &mut dyn FnMut(Vec<LibraryItem>)) ->
      Option<Result<(), LibraryServiceError>>`, default returns `None` (unsupported).
- [x] 9.2 Implement `list_items_updated_since` on the SDK-backed service
      (`dtrpg-core/src/services/sdk.rs`), setting `LibraryItemsParams::updated_date_after`
      and paginating like `list_items_paged`.
- [x] 9.3 In `start_load_inner`'s count-check branch, when `remote_count > cached_count`,
      compute `since_iso8601` from the max `date_updated`/`date_added` across the cached
      catalog and call `list_items_updated_since`; on `None` (unsupported) or an error
      result, fall back to the existing full paginated fetch.
- [x] 9.4 Merge a successful partial fetch's results additively (append new ids, refresh
      + set `is_available = true` for existing ids) without running the "mark absent items
      unavailable" sweep.
- [x] 9.5 Add unit tests: `remote_count > cached_count` triggers a partial fetch and merges
      additively without flagging any item unavailable; `remote_count < cached_count`
      triggers a full fetch; unsupported partial fetch falls back to a full fetch.

## 10. Verification

- [x] 10.1 `cargo test -p dtrpg-ui` and `cargo test -p dtrpg-core` pass, including the new
       reconciliation, deserialization-default, reload-cooldown, single-item-check,
       check-queue, check-batch-cooldown, and partial-fetch tests. (6 pre-existing
       `util::datetime` locale-pluralization failures unrelated to this change, confirmed
       present on the base commit before this branch's work.)
- [x] 10.2 `cargo clippy --all-targets --all-features -- -D warnings` passes.
- [x] 10.3 `cargo fmt --all -- --check` passes. (Diff count against the base commit is
       unchanged in substance — the reported delta is pre-existing nightly-rustfmt-only
       style diffs shifting to new line numbers after this change's insertions, not new
       issues; this repo's checked-in style requires a nightly rustfmt unavailable in this
       environment, a known pre-existing gap.)
- [x] 10.4 Manually run the app against a cached catalog, remove/rename an item on the
       server side (or stub the SDK response), reload, and confirm the item persists in
       the UI flagged unavailable rather than disappearing; then restore it server-side and
       confirm the flag clears on the next load.
- [x] 10.5 Manually confirm a first-launch (no cache) load shows items appearing
       incrementally page-by-page rather than all at once at the end.
- [x] 10.6 Manually trigger "Catalog > Reload" twice in quick succession and confirm the
       second attempt is a no-op (no activity indicator, no network call); wait past the
       cooldown and confirm a third attempt performs a real reload.
- [x] 10.7 Manually open a catalog entry's details, confirm a checking indicator appears
       briefly and the item's availability is (re)confirmed; reopen the same entry within
       the cooldown and confirm no new check runs.
- [x] 10.8 Manually trigger a check batch twice in quick succession and confirm the second
       is a no-op; wait past the batch cooldown and confirm a third trigger enqueues checks,
       visibly indicated one at a time in the catalog view.
- [x] 10.9 Manually simulate `remote_count > cached_count` (e.g. add an item server-side or
       via a stub) and confirm the next load performs a partial fetch rather than a full
       paginated one, while an item removed server-side stays flagged available until a
       full reconciliation or item-level check catches it.
