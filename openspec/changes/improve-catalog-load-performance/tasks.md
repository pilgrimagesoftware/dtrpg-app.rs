## 1. Visible-items cache

- [ ] 1.1 Add `visible_cache: Option<Vec<LibraryItem>>` field to `LibraryController`; initialize to `None`
- [ ] 1.2 Add `fn invalidate_visible_cache(&mut self)` helper that sets `self.visible_cache = None`
- [ ] 1.3 Call `self.invalidate_visible_cache()` at the top of `set_filter`, `set_sort` (if it exists), `set_search_query`, `clear_search_query`, and `reload`
- [ ] 1.4 Rewrite `visible_items(&self)` to: if `self.visible_cache` is `Some`, return a clone; otherwise compute, store in `self.visible_cache`, and return a clone
- [ ] 1.5 Change `visible_items` to `&mut self` so it can populate the cache (update all call sites)

## 2. Batch flush in append path

- [ ] 2.1 Add `fn append_catalog_batch(&mut self, items: Vec<LibraryItem>, cx: &mut Context<Self>)` that extends the catalog, calls `invalidate_visible_cache()`, recomputes `section_counts` and `publisher_entries`, and emits `LibraryChanged` — this replaces `append_catalog_page`
- [ ] 2.2 Rename or remove the old `append_catalog_page` method (or make it delegate to `append_catalog_batch`)

## 3. Batched background load loop

- [ ] 3.1 Add `futures` as a direct dependency of `dtrpg-ui` in `Cargo.toml` if not already present (needed for `futures::select!` or `futures::future::select`)
- [ ] 3.2 Rewrite the page-receive loop in `LibraryController::new`: maintain a `buffer: Vec<LibraryItem>` and a flush timer using `async_cx.background_executor().timer(Duration::from_millis(500))`
- [ ] 3.3 On each loop iteration, `select!` between the next recv future and the timer future; if a page arrives, push items to `buffer`; if the timer fires (or the channel closes), flush the buffer via `this.update(async_cx, |ctrl, cx| ctrl.append_catalog_batch(std::mem::take(&mut buffer), cx))` and reset the timer
- [ ] 3.4 After the channel closes, flush any remaining buffered items immediately (before awaiting `fetch.await`)

## 4. Verify

- [ ] 4.1 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 4.2 Manually launch the app and confirm the catalog populates in visible batches (items appear every ~500 ms) without UI sluggishness during load
- [ ] 4.3 Confirm that filtering, sorting, and searching remain responsive during and after load
- [ ] 4.4 Confirm that interacting with the UI (clicking sidebar filters, typing in search) during load does not feel blocked
