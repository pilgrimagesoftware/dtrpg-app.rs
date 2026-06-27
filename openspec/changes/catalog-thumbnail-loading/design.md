## Context

The catalog already fetches and renders items incrementally (`append_catalog_page`). Each `LibraryItem` carries `cover_url: Option<Arc<str>>` but the cover rendering in `catalog_view.rs` always calls `render_generative_cover`, ignoring the URL entirely. `CoverCache` (a GPUI app-level global) already exists with `images` and `in_flight` tracking, but nothing ever populates it.

The GPUI framework provides `cx.background_executor().spawn(async move {...})` for offloading blocking HTTP work, and `cx.spawn(async move |this, async_cx| {...})` for tasks that need to drive UI updates.

## Goals / Non-Goals

**Goals:**
- Auto-enqueue items with a `cover_url` when they arrive via `append_catalog_page`
- Process the queue one thumbnail at a time to avoid thundering-herd HTTP bursts
- Surface progress in the activity panel as a single aggregated item
- Allow manual retry via a context menu on every catalog entry layout
- Guard against repeated rapid retries with a per-item cooldown

**Non-Goals:**
- Parallel thumbnail fetching (intentionally sequential for simplicity and server politeness)
- Persistent thumbnail cache (in-memory `CoverCache` only; no disk store)
- Thumbnail fetching during search/filter view modes (auto-enqueue fires on any `append_catalog_page`, but the queue is drained regardless of the current view)

## Decisions

### Sequential queue over parallel fetching

A `VecDeque<Arc<str>>` in `LibraryController` drains one item at a time. When the active fetch completes (or fails) the controller dequeues the next item and spawns a new background task.

**Why**: Server politeness, simpler error handling, and a single activity slot. Parallel fetching would require a semaphore and multiple activity items.

**Alternative**: `tokio::task::JoinSet` with bounded parallelism. Rejected for complexity and because real-world thumbnail latency is low enough that sequential is fine.

### Single aggregated activity item

One activity entry — "Loading covers… N remaining" — is updated in-place as the queue drains rather than starting/completing one item per thumbnail.

**Why**: Dozens of items per catalog page would flood the activity panel. Users care about overall progress, not individual URLs.

**How**: `LibraryController` holds `thumbnail_activity_id: Option<u64>`. On first enqueue it calls `activity.start(...)` and stores the ID. On each dequeue it calls a new `activity_label(id, label, cx)` method (or simply completes and restarts). On empty queue it calls `activity.complete(id, cx)`.

**Alternative**: `activity.update_label(id, label, cx)`. Preferred if `ActivityController` can be extended with a label-update method; otherwise complete/restart is acceptable.

### Cooldown stored on `LibraryItem`

`LibraryItem` gains `thumbnail_last_attempted: Option<std::time::SystemTime>`. The context menu render checks `SystemTime::now().duration_since(last)` against a 5-minute threshold to decide whether to enable the menu item.

**Why**: `std::time::SystemTime` is always available and requires no additional state; `Instant` is not `Send`/serializable. Cooldown lives on the item so it survives controller re-renders without extra lookup maps.

**Alternative**: A separate `HashMap<Arc<str>, SystemTime>` on `LibraryController`. More indirection for the same information.

### HTTP fetch via `reqwest` blocking client

Thumbnail URLs are HTTPS. The existing SDK already depends on `reqwest`; the app can use `reqwest::blocking::get` inside a `background_executor` task, or `reqwest::get` in an async task. Use the async form to stay consistent with the GPUI async task model.

**Why**: `background_executor().spawn` accepts an async future; calling the async `reqwest::get` avoids a blocking thread.

### Context menu via `gpui_component::menu`

All three catalog layouts (list, thumbs, grid) wrap their row/card in a `right_mouse_button_down` handler that shows a `DropdownMenu` with a `PopupMenuItem::new("Load Thumbnail")`. `gpui_component::menu` is already used in the toolbar.

## Risks / Trade-offs

- [Risk] `reqwest` is not a direct dependency of `dtrpg-ui` → Mitigation: add `reqwest` with the `rustls-tls` feature to `dtrpg-ui/Cargo.toml`; or extract the fetch into `dtrpg-core` behind a trait. Prefer adding directly for simplicity.
- [Risk] `SystemTime` is not monotonic; wall-clock skew could shorten or extend the cooldown window → Mitigation: use `checked_duration_since` and treat `None` (clock went backwards) as "not too soon" (allow retry).
- [Risk] Activity label updates require a new `ActivityController` method → Mitigation: implement `update_label(id, label, cx)` as part of this change.
- [Risk] Context menu position in GPUI varies by layout — list rows are thin, grid cards are large → Mitigation: use the same `DropdownMenu` pattern from the toolbar; position is relative to the triggering element.

## Open Questions

- Should failed fetches (non-200, network error) restart the queue immediately or delay? → Proceed immediately; the cooldown guard on the item prevents instant re-trigger from the context menu.
- Should the queue persist across catalog reloads? → No; the queue is in-memory and resets with the controller.
