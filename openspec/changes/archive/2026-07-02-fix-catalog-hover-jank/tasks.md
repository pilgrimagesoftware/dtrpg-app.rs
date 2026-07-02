## 1. Cache `visible_items()` on `LibraryController`

- [x] 1.1 Add `items_cache: Option<Vec<LibraryItem>>` field to `LibraryController` (initialize to `None`)
- [x] 1.2 Add private `fn invalidate_cache(&mut self)` — implemented as an eager recompute (matches design.md's chosen "eager rebuild on mutation" decision) rather than a lazy `None` flag, since `visible_items()` etc. must stay `&self` for existing read-only call sites
- [x] 1.3 (superseded by 1.2's eager approach) — no separate lazy `cached_visible_items` helper; `visible_items()` reads the eagerly-populated cache directly
- [x] 1.4 `visible_items()` now clones from `self.items_cache` instead of re-filtering/sorting
- [x] 1.5 `visible_items_count()` unchanged in structure but now backed by the cache via `visible_items()` (no full re-scan)
- [x] 1.6 `visible_items_slice()` unchanged in structure but now backed by the cache via `visible_items()` (no full re-scan)
- [x] 1.7 Call `invalidate_cache()` at every mutation site: `set_filter`, `set_search_query`, `clear_search_query`, `set_sort`, `set_sort_direction`, `set_catalog`, `append_catalog_page`, `replace_service`, `reload`, `toggle_download`, and the thumbnail-fetch completion handlers (any site that mutates `self.catalog`, `self.filter`, `self.search_query`, `self.sort`, or `self.sort_direction`). `set_page`/`set_page_size` intentionally do not invalidate since they don't change the filtered/sorted set.

## 2. Cache Grouped Data in `CatalogView`

- [x] 2.1 Add `grouped_cache: Option<Vec<PublisherGroup>>` field to `CatalogView` (initialize to `None`)
- [x] 2.2 In `CatalogView::new()`, update the `LibraryChanged` subscription to also set `this.grouped_cache = None`
- [x] 2.3 In `CatalogView::render()`, replace the inline `group_by_publisher(items)` calls in grouped List/Thumbs/Grid paths with `self.grouped_items(cx)`, which computes and stores the cache only when `None`

## 3. Detail Panel Cover

- [x] 3.1 Update `render_detail_panel` signature to accept `cover_image: Option<Arc<gpui::Image>>`
- [x] 3.2 In the cover child block, render `img(image).w(px(cover_w)).h(px(cover_h)).object_fit(ObjectFit::Cover)` when `Some`, fall back to `render_generative_cover` when `None`
- [x] 3.3 In `LibraryRootView::render()`, look up `cx.global::<CoverCache>().get(&item.id)` for the selected item before calling `render_detail_panel`, pass the result as the new parameter

## 4. Build and Lint

- [x] 4.1 Run `cargo check --workspace` -- no errors
- [x] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings` -- no new warnings
- [x] 4.3 Run `cargo fmt --all` -- no formatting changes (touched files only; two pre-existing unrelated diffs in `dtrpg-sdk/rust/src/client.rs` left untouched)
- [x] 4.4 Run `cargo test --workspace` -- all 83 unit tests pass; one pre-existing doctest failure in `credentials/mod.rs` confirmed unrelated (reproduces on `develop` before this change)

## 5. Manual Verification

- [x] 5.1 Mouse movement over the catalog (Thumbs, ungrouped) no longer causes visible lag or beachball
- [x] 5.2 Mouse movement over the catalog (Thumbs, grouped) no longer causes visible lag or beachball
- [x] 5.3 Mouse movement over the detail panel no longer causes visible lag or beachball
- [x] 5.4 When a cover is cached, the detail panel shows the real thumbnail (not the generative cover)
- [x] 5.5 Switching filters/search updates the catalog correctly (cache invalidation works)
