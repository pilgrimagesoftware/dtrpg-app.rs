## 1. Cache `visible_items()` on `LibraryController`

- [ ] 1.1 Add `items_cache: Option<Vec<LibraryItem>>` field to `LibraryController` (initialize to `None`)
- [ ] 1.2 Add private `fn invalidate_cache(&mut self)` that sets `self.items_cache = None`
- [ ] 1.3 Add private `fn cached_visible_items(&mut self) -> &[LibraryItem]` that populates the cache if `None` (runs filter + sort) and returns a slice
- [ ] 1.4 Rewrite `visible_items()` to call `cached_visible_items()` and clone the result
- [ ] 1.5 Rewrite `visible_items_count()` to call `cached_visible_items().len()` without a full re-scan
- [ ] 1.6 Rewrite `visible_items_slice()` to call `cached_visible_items()` and slice without a full re-scan
- [ ] 1.7 Call `invalidate_cache()` at every mutation site: `set_filter`, `set_search_query`, `set_sort`, `set_sort_direction`, `set_page`, `set_page_size`, `load_catalog`, and any other method that changes `self.catalog`, `self.filter`, `self.search_query`, `self.sort`, or `self.sort_direction`

## 2. Cache Grouped Data in `CatalogView`

- [ ] 2.1 Add `grouped_cache: Option<Vec<PublisherGroup>>` field to `CatalogView` (initialize to `None`)
- [ ] 2.2 In `CatalogView::new()`, update the `LibraryChanged` subscription to also set `this.grouped_cache = None`
- [ ] 2.3 In `CatalogView::render()`, replace the inline `group_by_publisher(items)` calls in grouped Thumbs and Grid paths with a single read: if `self.grouped_cache.is_none()` then compute and store; then clone for the render

## 3. Detail Panel Cover

- [ ] 3.1 Update `render_detail_panel` signature to accept `cover_image: Option<Arc<gpui::Image>>`
- [ ] 3.2 In the cover child block, render `img(image).w(px(cover_w)).h(px(cover_h)).object_fit(ObjectFit::Cover)` when `Some`, fall back to `render_generative_cover` when `None`
- [ ] 3.3 In `LibraryRootView::render()`, look up `cx.global::<CoverCache>().get(&item.id)` for the selected item before calling `render_detail_panel`, pass the result as the new parameter

## 4. Build and Lint

- [ ] 4.1 Run `cargo check --workspace` -- no errors
- [ ] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings` -- no new warnings
- [ ] 4.3 Run `cargo fmt --all` -- no formatting changes
- [ ] 4.4 Run `cargo test --workspace` -- all tests pass

## 5. Manual Verification

- [ ] 5.1 Mouse movement over the catalog (Thumbs, ungrouped) no longer causes visible lag or beachball
- [ ] 5.2 Mouse movement over the catalog (Thumbs, grouped) no longer causes visible lag or beachball
- [ ] 5.3 Mouse movement over the detail panel no longer causes visible lag or beachball
- [ ] 5.4 When a cover is cached, the detail panel shows the real thumbnail (not the generative cover)
- [ ] 5.5 Switching filters/search updates the catalog correctly (cache invalidation works)
