## 1. Cover URL resolution

- [ ] 1.1 In `crates/dtrpg-core/src/services/sdk/library/mapping.rs`, change `resolve_cover_url`'s field preference order from `thumbnail` → `thumbnail_100` → `image` to `image` → `web_image` → `thumbnail` → `thumbnail_100`
- [ ] 1.2 Update the doc comment above `resolve_cover_url` to describe the new preference order and rationale (full-size/WebP preferred for render-quality; small thumbnails as last-resort fallback)

## 2. Tests

- [ ] 2.1 Update `map_order_product_builds_cover_url_from_sideloaded_product_relationship`'s expected `cover_url` to the full-size image URL (`.../4952/515276.jpg`)
- [ ] 2.2 Update `map_order_product_builds_cover_url_from_embedded_thumbnail_fallback`'s expected `cover_url` to the full-size image URL (`.../4952/515276.jpg`)
- [ ] 2.3 Add a test covering the `web_image`-preferred-over-thumbnail case (full-size `image` absent, `web_image` present)
- [ ] 2.4 Add a test covering the thumbnail-only fallback case (neither `image` nor `web_image` present, only `thumbnail`/`thumbnail_100`)

## 3. Verification

- [ ] 3.1 `cargo build --workspace --all-features`
- [ ] 3.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 3.3 `cargo test --workspace --all-features` (mapping.rs unit tests)
- [ ] 3.4 Launch app, clear the cover cache, and confirm catalog covers render sharper at grid-card and detail-panel sizes than before
