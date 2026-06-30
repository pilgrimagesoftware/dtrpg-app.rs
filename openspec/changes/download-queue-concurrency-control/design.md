## Context

The app currently loads thumbnails serially via a `VecDeque<(Arc<str>, Arc<str>)>` and a `thumbnail_loading: bool` flag in `LibraryController`. Only one HTTP request runs at a time; new items queue behind it. File downloads are a stub that toggles `ItemStatus` without fetching anything. The activity panel exists and supports named in-progress items, but download progress is never reported there.

`StorageConfig` holds the catalog root path. `SettingsController` wraps it. Both are persisted to a TOML config file on disk.

## Goals / Non-Goals

**Goals:**
- Replace serial thumbnail drain with a bounded concurrent dispatcher (up to N parallel fetches).
- Implement real file downloads dispatched N-at-a-time, each surfaced in the activity panel.
- Add `max_concurrent_downloads: usize` to `StorageConfig` (default: 3) and expose it in the Storage settings panel.
- Keep all background work on GPUI background tasks; no new threads or runtimes.

**Non-Goals:**
- Pause/resume of individual downloads (cancel only).
- Download speed throttling or bandwidth limits.
- Resume of partially downloaded files after crash.
- Per-item download progress percentage (activity panel shows started/complete/error, not byte progress).

## Decisions

### Shared concurrency limit for thumbnails and downloads

Both thumbnail fetches and file downloads draw from the same `max_concurrent_downloads` setting rather than separate limits. This keeps the settings UI simple and avoids saturating the network when both are active simultaneously.

_Alternative considered_: Separate limits (e.g., `max_thumbnail_threads`, `max_download_threads`). Rejected because most users won't want to tune both independently, and exposing two settings adds UI complexity without meaningful benefit.

### Semaphore-style slot tracking in LibraryController

`LibraryController` tracks how many slots are currently occupied (`active_thumbnail_fetches: usize`, `active_downloads: usize`). When a slot frees (task completes or errors), the controller drains the corresponding queue to fill empty slots. This is simpler than a real semaphore and avoids async-sync boundary problems in GPUI's task model.

_Alternative considered_: `Arc<tokio::sync::Semaphore>` shared across tasks. Rejected because GPUI tasks are not bare Tokio tasks; sharing a semaphore across GPUI `cx.spawn` boundaries requires careful `Send` plumbing and obscures the controller's state invariants.

### Activity panel entry per download, not per batch

Each file download gets its own named activity item (`start()` on enqueue, `complete()` or `error()` on finish). Thumbnail batch loading uses a single shared activity item that is updated with remaining count, as today.

_Alternative considered_: One activity item for all downloads. Rejected because users need to see which specific title is downloading, especially when cancelling.

### StorageConfig holds the concurrency setting

`max_concurrent_downloads` lives in `StorageConfig` and is written to the same config TOML. `SettingsController` reads/writes it via the existing `apply_*` pattern.

## Risks / Trade-offs

- [Risk] Rapid sign-in followed by rapid catalog sync could enqueue hundreds of thumbnail fetches before the limit kicks in. → Mitigation: `drain_thumbnail_queue` already checks slot availability before spawning; capacity is bounded by the queue drain pattern.
- [Risk] Cancelling a download mid-flight leaves a partial file on disk. → Mitigation: write to a `.part` temp file and rename on completion; delete `.part` on cancel or error. (Out of scope for this change if `LibraryService::download_item` is not yet implemented; the queue wiring is the deliverable.)
- [Trade-off] Per-item download progress (bytes downloaded / total) is not tracked. Activity panel shows started/done/error only. Acceptable for the first pass; byte-level progress requires a streaming `LibraryService` API change.

## Open Questions

- Does `LibraryService::download_item` need to be added in this change, or is the queue infrastructure the deliverable and the real service implementation follows? **Decision: wire the queue and activity panel entries; `toggle_download` becomes `enqueue_download`; the actual HTTP fetch is a follow-on task once the service trait is extended.**
