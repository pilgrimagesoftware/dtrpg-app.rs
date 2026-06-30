## 1. SDK helper: parse last-page from links

- [x] 1.1 Add `fn last_page_from_links(links: &PaginationLinks) -> Option<u32>` in `dtrpg-core/src/services/sdk.rs`: parse the `page` query parameter from `links.last` using `url::Url` or a simple string split (no new dep required - split on `page=` then parse digits)
- [x] 1.2 Add a unit test in the same file covering: last URL present with valid page number, last URL absent (`None`), last URL present but no `page` param (`None`)

## 2. Thread total count through list_items_paged

- [x] 2.1 Add an `on_total: Option<&mut dyn FnMut(usize)>` parameter to `SdkLibraryGateway::list_order_products_paged` - but since there is no such trait method currently (the loop is inline in `RustSdkLibraryService::list_items_paged`), instead modify `RustSdkLibraryService::list_items_paged` to accept a second parameter `on_total: Option<&mut dyn FnMut(usize)>` and update the signature in the `LibraryService` trait accordingly
- [x] 2.2 Inside the `list_items_paged` loop, after receiving the first page response, call `last_page_from_links(&response.links)` to compute `estimated_total = last_page * page_size` and call `on_total(estimated_total)` exactly once (guard with a `total_reported: bool` flag)
- [x] 2.3 Update the default `list_items_paged` implementation in the `LibraryService` trait (which calls `list_items`) to pass `on_total: None` - or update its signature to also accept `on_total` so callers can use it
- [x] 2.4 Update `list_items` (which calls `list_items_paged`) to pass `None` for `on_total`
- [x] 2.5 Update all stub/test `LibraryService` implementations (`UnavailableSdkGateway`, stubs in `dtrpg-ui/src/services/stub.rs`, test stubs in `view_models/library.rs`) to pass the updated signature - `on_total` receives `None` or is simply ignored in stubs

## 3. LibraryController: drive activity panel progress

- [x] 3.1 In `LibraryController::start_load` (`dtrpg-ui/src/controllers/library.rs`), change the `list_items_paged` call to pass an `on_total` closure that captures `weak_activity` and the activity `id`, and calls `activity.update(cx, |a, cx| a.update_progress(id, 0.0, cx))` to make the bar visible the moment the total is known
- [x] 3.2 Track `items_loaded: usize` and `estimated_total: Option<usize>` as local variables inside the `start_load` background spawn; after each `on_page` delivers items, compute `progress = items_loaded as f32 / estimated_total as f32` and call `weak_activity.update(cx, |a, cx| a.update_progress(id, progress, cx))` - skip the call if `estimated_total` is `None`
- [x] 3.3 Ensure the progress update is called from within the background executor's `spawn` correctly (the existing `cx.spawn` / `async_cx` pattern already handles this - follow the same pattern as the existing `weak_activity` calls in `start_load`)

## 4. Verify

- [x] 4.1 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 4.2 Manually launch the app, sign in, and confirm the activity panel shows a filling progress bar during the initial catalog load (not just a spinner)
- [ ] 4.3 Confirm the bar reaches 100% and the entry transitions to "complete" state after load finishes
- [ ] 4.4 Confirm a subsequent reload (replace_service / re-login) shows progress again from 0%
