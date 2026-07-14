Issue: https://github.com/pilgrimagesoftware/dtrpg-app.rs/issues/92

## Why

`resolve_cover_url` (`dtrpg-core/src/services/sdk/library/mapping.rs`) picks one `cover_url` per catalog item and that single URL is rendered everywhere: grid cards, thumb rows, and the detail panel. The current order (`thumbnail` 140px -> `thumbnail_100` 100px -> `image` full-size) always picks a source no larger than 140px, which the detail panel (~480px wide) then upscales several times past its native resolution, producing visibly blurry, pixelated covers. Using the full-size image everywhere instead would fix the detail panel but needlessly download and cache a full-size image for every small grid card and thumb row, where a 140px thumbnail already renders correctly.

## What Changes

- Resolve two cover URLs per item instead of one: a small-context `cover_url` for grid cards and thumb rows, and a large-context `detail_cover_url` for the detail panel.
- `cover_url` preference order stays small-first: `thumbnail` (140px) -> `thumbnail_100` (100px) -> `image` -> `web_image`, unchanged from today except for the `web_image` fallback added at the end.
- `detail_cover_url` preference order is large-first: `image` (full-size) -> `web_image` (WebP) -> `thumbnail` -> `thumbnail_100`.
- The detail panel renders `detail_cover_url`, falling back to `cover_url` when the item has no large-context source.
- No change to the download/cache/render pipeline mechanics themselves (`cover_cache.rs`, `ui/library/cover.rs`) beyond loading two URLs per item instead of one — both are already size-agnostic.

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

- `libri-cover`: cover URL resolution now produces a small-context URL (grid/thumb row) and a large-context URL (detail panel) per item, each preferring the source closest to its render size, instead of one URL shared everywhere.

## Impact

- `crates/dtrpg-core/src/services/sdk/library/mapping.rs`: `resolve_cover_url` gains a large-context counterpart; existing unit tests need a `detail_cover_url` assertion alongside the existing `cover_url` one.
- `crates/dtrpg-ui/src/data/library.rs`: `LibraryItem` gains a `detail_cover_url: Option<Arc<str>>` field (`#[serde(default)]` for cache compatibility).
- `crates/dtrpg-ui/src/data/catalog_cache.rs`: cache schema version bump, since old caches lack `detail_cover_url` and should be treated as stale rather than silently missing detail-panel cover data.
- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: renders `item.detail_cover_url` (falling back to `item.cover_url`) instead of `item.cover_url`.
- `crates/dtrpg-ui/src/controllers/library.rs`: thumbnail queue/refresh logic extends to cover both URLs where each render context needs its own fetch.
- `crates/dtrpg-ui/src/ui/views/catalog_view.rs` (grid card, thumb row) and `cover_cache.rs`/`ui/library/cover.rs` are otherwise unaffected — they already handle arbitrary image dimensions and per-URL disk caching.
- Larger disk-cache footprint per item (a small thumbnail plus a full-size/WebP image are both cached now, versus one shared image today), offset by only downloading the large image for items the user actually opens in the detail panel.
