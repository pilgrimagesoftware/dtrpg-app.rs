## Context

`resolve_cover_url` (`dtrpg-core/src/services/sdk/library/mapping.rs`) picks one `cover_url` per catalog item from `OrderProductInfo`'s four optional image fields — `image` (full size), `web_image` (WebP), `thumbnail` (140px), `thumbnail_100` (100px) — and that single URL is used everywhere the item's cover renders: catalog grid cards, thumb rows, item popover, and the detail panel (~480px wide). The current order (`thumbnail` → `thumbnail_100` → `image`) always picks a source no larger than 140px, which the UI then upscales past its native resolution in every context except small list rows.

The cover pipeline downstream of `cover_url` (`cover_cache.rs`, `ui/library/cover.rs`) is size-agnostic — it downloads, disk-caches, and renders whatever bytes are behind the URL, so this change is isolated to which URL gets chosen.

## Goals / Non-Goals

**Goals:**
- Resolve a higher-resolution `cover_url` when the API response offers one, so covers rendered at detail-panel/grid-card sizes are sharp.
- Keep `resolve_cover_url` a pure, single-URL-per-item function — no new fields on `LibraryItem`, no per-render-context URL selection.

**Non-Goals:**
- No change to the download/cache/render pipeline itself.
- No per-context (grid vs. detail vs. thumb-row) resolution selection — one URL per item remains the model, matching every other capability built on `cover_url` (`libri-cover`, `cover-thumbnail-disk-cache`, `thumbnail-queue-concurrency`).
- No user-facing preference for image quality vs. bandwidth — out of scope, can be a follow-on if requested.

## Decisions

### New preference order: `image` → `web_image` → `thumbnail` → `thumbnail_100`

`image` is the full-size original, so it's preferred first. `web_image` is a WebP re-encode of the same asset (smaller file size, comparable resolution) and is the next-best choice when `image` is absent. The two small pre-generated thumbnails remain as a last-resort fallback for older catalog entries where the API only ever populated those fields.

_Alternative considered_: Prefer `web_image` over `image` for its smaller download size. Rejected — `image` is the more universally-populated field in observed API responses (the two production fixtures both have all four fields populated, but `web_image` is a newer field and can't be assumed present for all catalog history), and correctness/quality takes priority over bandwidth for a first change; bandwidth can be revisited separately if downloads prove too large in practice.

### No per-context sizing

Selecting a different `cover_url` per render context (e.g., a small thumbnail for list rows, a large one for the detail panel) would need either multiple URL fields on `LibraryItem` or an image-resizing step in `cover_cache.rs`, both larger changes than the reported problem (blurry covers) requires. A single higher-resolution URL is a strict quality improvement everywhere it renders, including the small list rows (a downscaled full-size image still looks correct, just not maximally bandwidth-efficient).

_Alternative considered_: Add a second `thumb_cover_url` field for small contexts. Rejected as premature — no evidence yet that downloading full-size images for list rows causes a measurable problem; revisit if it does.

## Risks / Trade-offs

- [Risk] Full-size/WebP images are larger than 100-140px thumbnails, increasing per-item network transfer and disk-cache size across the whole catalog. → Mitigation: the disk cache (`cover-thumbnail-disk-cache`) already persists across sessions, so the cost is paid once per item, not per render; if this proves too costly in practice, a follow-on change can revisit per-context sizing.
- [Risk] The two existing `mapping.rs` unit tests assert the old `thumbnail`-first URL. → Mitigation: update both tests' expected URLs as part of this change (task list covers this).
