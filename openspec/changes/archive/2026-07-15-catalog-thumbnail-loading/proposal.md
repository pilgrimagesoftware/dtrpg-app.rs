## Why

Catalog entries display a generative placeholder cover at all times because real thumbnails are never fetched, making the library look synthetic regardless of what the user actually owns. Fetching real cover art improves recognition and visual quality for the catalog experience.

## What Changes

- When a catalog page is appended, items that have a `cover_url` and whose cover is not already cached or in-flight are automatically enqueued for thumbnail loading.
- A sequential background queue processes one thumbnail at a time via HTTP, storing the result in `CoverCache`.
- The activity panel reflects thumbnail loading progress with a single aggregated item ("Loading covers… N remaining") rather than one item per cover.
- Each catalog entry's context menu exposes a "Load Thumbnail" action to manually trigger or retry a cover fetch.
- `LibraryItem` records the last time a thumbnail load was attempted; if attempted too recently (within 5 minutes), the context menu item is disabled.

## Capabilities

### New Capabilities

- `thumbnail-loading-queue`: Automatic and manual sequential thumbnail fetching with activity panel tracking.
- `thumbnail-cooldown-guard`: Per-item last-attempted timestamp that disables the "Load Thumbnail" menu item when a fetch was attempted too recently.

### Modified Capabilities

<!-- none -->

## Impact

- `dtrpg-ui/src/data/library.rs` — add `thumbnail_last_attempted: Option<std::time::SystemTime>` to `LibraryItem`
- `dtrpg-ui/src/controllers/library.rs` — add thumbnail queue (`VecDeque<Arc<str>>`), `thumbnail_loading: bool`, `thumbnail_activity_id: Option<u64>`; drive the sequential fetch loop; expose "enqueue" action for context menu
- `dtrpg-ui/src/ui/library/cover.rs` — `CoverCache` already exists; no structural changes needed
- `dtrpg-ui/src/ui/views/catalog_view.rs` — add context menu to list, thumbs, and grid layouts using `gpui_component::menu::{DropdownMenu, PopupMenuItem}`
- No new external crate dependencies; thumbnail bytes are fetched via the existing HTTP client already used by the SDK
