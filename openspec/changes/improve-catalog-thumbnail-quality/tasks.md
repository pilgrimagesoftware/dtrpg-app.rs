## 1. Cover URL resolution

- [ ] 1.1 In `crates/dtrpg-core/src/services/sdk/library/mapping.rs`, update `resolve_cover_url`'s field preference order to `thumbnail` -> `thumbnail_100` -> `image` -> `web_image` (unchanged except for the `web_image` fallback appended at the end)
- [ ] 1.2 Add `resolve_detail_cover_url` with preference order `image` -> `web_image` -> `thumbnail` -> `thumbnail_100`
- [ ] 1.3 Call `resolve_detail_cover_url` in `map_order_product` and populate `LibraryItem::detail_cover_url`
- [ ] 1.4 Update the doc comments above both resolver functions to describe their preference order, render context, and rationale

## 2. Data model and cache

- [ ] 2.1 In `crates/dtrpg-ui/src/data/library.rs`, add `detail_cover_url: Option<Arc<str>>` to `LibraryItem` with `#[serde(default)]`, and set it to `None` in `LibraryItem::new`
- [ ] 2.2 In `crates/dtrpg-ui/src/data/catalog_cache.rs`, bump `CACHE_SCHEMA_VERSION` and document the bump (old caches lack `detail_cover_url`)

## 3. Lazy detail-cover fetch

- [ ] 3.1 In `crates/dtrpg-ui/src/controllers/library.rs`, add a method (e.g. `ensure_detail_cover`) that enqueues a fetch of `detail_cover_url` for a given item id if not already cached (under a distinct cache key, e.g. `"{item_id}::detail"`) or in flight
- [ ] 3.2 Call this method from each call site that invokes `TabsController::open_detail_tab` (`crates/dtrpg-ui/src/ui/views/catalog_view.rs`)
- [ ] 3.3 In `crates/dtrpg-ui/src/ui/views/root_view.rs`, look up the detail-context cover from the cache using the distinct key when building `render_detail_tab_content`'s `cover_image` argument, falling back to the small-context `cover_url`'s cache entry if the detail one isn't cached yet
- [ ] 3.4 In `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`, point the refresh-thumbnail action at `detail_cover_url` (falling back to `cover_url`) instead of `cover_url`

## 4. Tests

- [ ] 4.1 Update `map_order_product_builds_cover_url_from_sideloaded_product_relationship` to assert both the unchanged small-context `cover_url` and the new full-size `detail_cover_url`
- [ ] 4.2 Update `map_order_product_builds_cover_url_from_embedded_thumbnail_fallback` similarly
- [ ] 4.3 Add a test covering `detail_cover_url`'s `web_image`-preferred-over-thumbnail case (full-size `image` absent, `web_image` present)
- [ ] 4.4 Add a test covering `detail_cover_url`'s thumbnail-only fallback case (neither `image` nor `web_image` present, only `thumbnail`/`thumbnail_100`)
- [ ] 4.5 Add a test covering the lazy-fetch behavior: opening a detail tab enqueues the detail-cover fetch once, and a second open does not re-enqueue it while cached

## 5. Verification

- [ ] 5.1 `cargo build --workspace --all-features`
- [ ] 5.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 5.3 `cargo test --workspace --all-features`
- [ ] 5.4 Launch app, clear the cover cache, and confirm grid/thumb covers still render crisply at their small size while the detail panel renders sharper at ~480px than before
