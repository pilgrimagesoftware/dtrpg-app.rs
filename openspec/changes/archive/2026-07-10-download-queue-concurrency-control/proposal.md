## Why

Catalog downloads and thumbnail loading both run serially today: one thumbnail loads at a time behind a boolean lock, and the download button is a stub that toggles item status without fetching anything. Users have no way to see progress, cancel in-flight work, or tune how aggressively the app fetches, which makes the library feel unresponsive on large catalogs.

## What Changes

- Replace the serial thumbnail queue (`thumbnail_loading: bool` + `VecDeque`) with a bounded concurrent queue that processes N thumbnails in parallel, where N is user-configurable.
- Implement real file downloads via a queue that dispatches up to N concurrent downloads, surfacing each item as a named entry in the activity panel.
- Add a "Max concurrent downloads" setting in the Storage settings panel (applies to both downloads and thumbnails).
- Track per-item download progress (started, in-progress %, complete, error) in `ActivityController` so the activity panel displays meaningful state.

## Capabilities

### New Capabilities

- `download-queue`: File download queue dispatching up to N concurrent downloads; each download is represented as a named activity panel item showing filename and progress state.
- `thumbnail-queue-concurrency`: Parallel thumbnail loading with a shared concurrency limit drawn from the same user setting; replaces the current single-item serial drain loop.

### Modified Capabilities

- `rust-main-window-library-layout`: The existing non-blocking requirement (thumbnails and sync must not block main-window interaction) extends to explicitly require controlled concurrency: the number of concurrent background fetch operations MUST be bounded by a user-configurable limit, defaulting to 3.

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs` - replace `thumbnail_queue`/`thumbnail_loading` with a concurrent dispatcher; replace `toggle_download` stub with real queue enqueue/cancel logic.
- `crates/dtrpg-ui/src/controllers/activity.rs` - add per-item progress updates (`update_progress`, named download items).
- `crates/dtrpg-ui/src/controllers/settings.rs` - add `max_concurrent_downloads: usize` field; persist to storage config.
- `crates/dtrpg-ui/src/data/storage.rs` - add `max_concurrent_downloads` to `StorageConfig`.
- `crates/dtrpg-ui/src/ui/views/settings_storage_view.rs` - add concurrency stepper/numeric input.
- `crates/dtrpg-ui/src/services/mod.rs` - `LibraryService` needs a `download_item` method (or equivalent) returning a progress stream.
