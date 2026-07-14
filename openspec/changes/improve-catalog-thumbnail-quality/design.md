## Context

`resolve_cover_url` (`dtrpg-core/src/services/sdk/library/mapping.rs`) picks one `cover_url` per catalog item from `OrderProductInfo`'s four optional image fields — `image` (full size), `web_image` (WebP), `thumbnail` (140px), `thumbnail_100` (100px) — and that single URL is used everywhere the item's cover renders: catalog grid cards, thumb rows, item popover, and the detail panel (~480px wide). The current order (`thumbnail` -> `thumbnail_100` -> `image`) always picks a source no larger than 140px, which the UI then upscales past its native resolution in the detail panel. Rendering a full-size image everywhere instead would fix the detail panel but would upscale nothing while needlessly downloading/caching a large image for every small grid card and thumb row, where the 140px thumbnail already renders correctly.

The cover pipeline downstream of the resolved URL(s) (`cover_cache.rs`, `ui/library/cover.rs`) is size-agnostic — it downloads, disk-caches, and renders whatever bytes are behind a URL — so the change is about which URL(s) get chosen and fetched per render context, not the pipeline mechanics.

## Goals / Non-Goals

**Goals:**
- Small render contexts (grid card, thumb row) keep using a thumbnail-sized source; the detail panel uses a full-size/WebP source. Neither context upscales past its source's native resolution nor downloads more bytes than its render size needs.
- Keep `resolve_cover_url` and its new counterpart pure, deterministic functions of `OrderProductInfo` — no per-render-context branching inside the resolver, no network calls in mapping.

**Non-Goals:**
- No resizing/transcoding pipeline — still just picking among the four URLs the API already provides.
- No user-facing preference for image quality vs. bandwidth — out of scope, can be a follow-on if requested.

## Decisions

### Two resolved URLs per item: `cover_url` (small) and `detail_cover_url` (large)

`LibraryItem` gains a second optional field, `detail_cover_url: Option<Arc<str>>`, alongside the existing `cover_url`. `mapping.rs` resolves both from the same `OrderProductInfo` with opposite preference orders:

- `cover_url` (grid card, thumb row): `thumbnail` -> `thumbnail_100` -> `image` -> `web_image`. Unchanged from today except for the `web_image` fallback appended at the end — this context wants the smallest correctly-sized source and only reaches for something larger if no thumbnail was ever generated.
- `detail_cover_url` (detail panel): `image` -> `web_image` -> `thumbnail` -> `thumbnail_100`. `image` is the full-size original, preferred first; `web_image` is a WebP re-encode of the same asset (smaller file, comparable resolution) and is next-best when `image` is absent. The two small thumbnails remain a last-resort fallback for older catalog entries where the API only ever populated those fields.

_Alternative considered_: A single `cover_url` favoring the large sources everywhere (the original scope of this change). Rejected — it fixes the detail panel but forces every grid card and thumb row to download and disk-cache a full-size/WebP image it renders at a fraction of its native size, for no visual benefit in that context.

_Alternative considered_: Per-context resizing in `cover_cache.rs` (decode once, downsample for small contexts) instead of two URLs. Rejected as premature — the API already serves pre-generated smaller variants; re-deriving them client-side would duplicate work the server already does, for no quality gain.

### Detail cover is fetched lazily, on tab open

`cover_url` keeps its existing eager-fetch behavior (`LibraryController::enqueue_thumbnails`, called for every item as the catalog loads) — the grid needs it immediately. `detail_cover_url` is fetched lazily: `LibraryController` fetches it only when a detail tab is opened for that item (the same call sites that already call `TabsController::open_detail_tab`), and only if not already cached or in flight. This avoids downloading a full-size/WebP image for every catalog item up front when most items are never opened in the detail panel.

The in-memory `CoverCache` and on-disk cover cache are both keyed by a string id; the detail image is stored under a distinct derived key (`"{item_id}::detail"`) so it doesn't collide with the small-context entry for the same item — both can be cached independently and evicted/refreshed independently.

_Alternative considered_: Eagerly fetch both URLs for every item on catalog load, same as today's single-`cover_url` behavior. Rejected — the whole point of the small/large split is to avoid paying for a large download on items that are never opened in the detail panel.

### Detail panel falls back to `cover_url` when no large-context source exists

If `detail_cover_url` is `None` (rare — only when `OrderProductInfo` had no `image`/`web_image`/`thumbnail`/`thumbnail_100` at all, which also makes `cover_url` `None`) or its fetch hasn't completed yet, the detail panel renders whatever is available in `cover_url`'s cache entry rather than showing the generative placeholder unnecessarily.

## Risks / Trade-offs

- [Risk] Two cached images per item (small + large) instead of one increases total disk-cache footprint for items a user does open in the detail panel. -> Mitigation: the large image is fetched lazily (only for opened items), and the disk cache already persists across sessions, so the cost is paid once per opened item, not per render.
- [Risk] The two existing `mapping.rs` unit tests assert the old single `thumbnail`-first `cover_url`. -> Mitigation: update both tests' `cover_url` assertions and add `detail_cover_url` assertions alongside them (task list covers this).
- [Risk] Existing on-disk catalog caches lack `detail_cover_url` for every item. -> Mitigation: bump `CACHE_SCHEMA_VERSION` in `catalog_cache.rs` so old caches are treated as stale and refetched, consistent with how the version-2 bump handled the original `cover_url` field rollout.
