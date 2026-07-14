Issue: https://github.com/pilgrimagesoftware/dtrpg-app.rs/issues/92

## Why

`resolve_cover_url` (`dtrpg-core/src/services/sdk/library/mapping.rs`) picks the smallest available cover image first: the 140px `thumbnail`, falling back to the 100px `thumbnail_100`, then the full-size `image`. But the UI renders that same URL well beyond 140px — the detail panel cover is ~480px wide and grid cards scale to their column width — so most covers are a small bitmap stretched several times past its native resolution, producing visibly blurry, pixelated covers throughout the catalog.

## What Changes

- Change `resolve_cover_url`'s preference order to favor higher-resolution sources over the small thumbnails: prefer the full-size `image`, then the `web_image` (WebP) if `image` is absent, falling back to `thumbnail` (140px) and `thumbnail_100` (100px) only when neither larger source exists.
- No change to the download, caching, or rendering pipeline — `cover_url` remains a single string per item; only which source path populates it changes.

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

- `libri-cover`: the cover thumbnail source URL resolution now prefers full-size/web-optimized images over the small pre-generated thumbnails when both are available.

## Impact

- `crates/dtrpg-core/src/services/sdk/library/mapping.rs`: `resolve_cover_url`'s field preference order changes; existing unit tests asserting the old preference order (`thumbnail` before `image`) need updating.
- `crates/dtrpg-ui/src/data/cover_cache.rs` and the cover download/render pipeline (`ui/library/cover.rs`) are unaffected — they already handle arbitrary image dimensions.
- Larger average per-item cover download and disk-cache footprint, since full-size or WebP images are typically larger than 100-140px thumbnails.
