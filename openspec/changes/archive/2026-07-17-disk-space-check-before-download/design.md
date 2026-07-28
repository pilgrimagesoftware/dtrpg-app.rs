## Context

Four call shapes currently start a download directly, unconditionally:
- `LibraryController::enqueue_download(id, title, cx)` — every not-yet-downloaded file in one
  entry. Called from `item_popover_view.rs`, `detail_panel_view.rs`, and four sites in
  `catalog_view.rs`.
- `LibraryController::download_all_for_collection(collection_id, cx)` and
  `download_all_for_publisher(publisher, cx)` — both loop over matching items and call
  `enqueue_download` per item.
- `LibraryController::enqueue_item_download(id, index, title, cx)` — a single file. Called
  internally by `enqueue_download` in a loop, and directly once from `detail_panel_view.rs` for
  a per-file button inside a multi-item entry's detail tab.

Each catalog file already carries `size_mb: f64` (`LibraryItemFile`). `StorageConfig::root_path()`
resolves the on-disk download root. There is no existing free-disk-space query anywhere in the
app. `window.open_alert_dialog(cx, |alert, _, _| alert.confirm().title(...).description(...)
.on_ok(...))` is an established pattern (`settings_advanced_view.rs`'s "Clear cache" action).

## Goals / Non-Goals

**Goals:**

- One disk-space check per discrete user-initiated queuing action, computed as an aggregate
  over every file that action would enqueue — not per individual file.
- Reuse the existing confirm-dialog pattern; no new dialog primitive.
- Fail open: if free space can't be determined (e.g. an unsupported filesystem or I/O error),
  queue the download as today rather than blocking it on an unrelated probe failure.

**Non-Goals:**

- Checking disk space against `enqueue_item_download` when it's invoked as part of
  `enqueue_download`'s internal per-file loop — that would check the same file multiple times
  for a multi-item entry's bulk action and, worse, checks only one file's size in isolation
  instead of the whole entry's aggregate size, defeating the purpose of the check.
  `enqueue_item_download` remains ungated when called that way.
- Live-updating the warning as free space or the queue changes after the dialog opens — the
  check is a one-time pre-flight snapshot, matching the "Clear cache" confirmation's behavior.
- Any settings/preference to disable the check.

## Decisions

### Gate at the four real user-initiated entry points, not at the shared per-file primitive

`enqueue_download` and `enqueue_item_download` stay exactly as they are today — unconditional,
reused by everything. New gating wrapper methods sit in front of the four places a user actually
triggers a queuing action:

- `request_download(id, title, cx)` — wraps `enqueue_download`. UI call sites that currently
  call `ctrl.enqueue_download(...)` directly switch to this.
- `request_item_download(id, index, title, cx)` — wraps `enqueue_item_download`, for the single
  direct call site in `detail_panel_view.rs`.
- `request_download_all_for_collection(collection_id, cx)` — wraps
  `download_all_for_collection`.
- `request_download_all_for_publisher(publisher, cx)` — wraps `download_all_for_publisher`.

Each computes the aggregate missing-file size for its own target set (reusing
`missing_file_indices`, `collection_download_targets`, `publisher_download_targets` — all
already exist), checks it against free space, and either calls straight through to the
unconditional method (space OK, or unknown) or stashes the request and emits a warning event
(space insufficient).

**Alternative considered**: embed the check inside `enqueue_download` /
`download_all_for_collection` / `download_all_for_publisher` directly. Rejected — those are
called from each other (`download_all_for_collection` calls `enqueue_download` per item) and
from the confirm handler once the user has already said yes; embedding the check inside them
means confirming would immediately re-trigger the same check and show the same dialog again
(the underlying state hasn't changed), and a bulk action's per-item calls into `enqueue_download`
would each check that one item's size instead of the batch's total.

### Pending request stored on the controller, one confirm dialog subscriber

Add `pending_download: Option<PendingDownloadRequest>` to `LibraryController`, where:

```rust
enum PendingDownloadRequest {
    Item { id: Arc<str>, title: String },
    ItemFile { id: Arc<str>, index: u32, title: String },
    Collection { collection_id: u64 },
    Publisher { publisher: Arc<str> },
}
```

A gating wrapper that decides to warn stores the appropriate variant in `pending_download` and
emits a new `LowDiskSpaceWarning { needed_mb: f64, free_mb: f64 }` event
(`data/events.rs`, `EventEmitter<LowDiskSpaceWarning> for LibraryController`). One subscription
in `root_view.rs` (via `cx.subscribe_in`, matching the existing `SignInSucceeded`/`CacheCleared`
subscriptions that need `window`) opens the confirm dialog:
title/description built from `needed_mb`/`free_mb`, `.on_ok` calls
`ctrl.confirm_pending_download(cx)`, dismissing without confirming leaves `pending_download` set
until the next queuing attempt overwrites or clears it (see Risks).

`confirm_pending_download` matches on the stashed variant and calls the corresponding
*unconditional* method (`enqueue_download`, `enqueue_item_download`,
`download_all_for_collection`, or `download_all_for_publisher`) directly, then clears
`pending_download` — never re-entering a `request_*` wrapper, so it can't loop back into another
check.

**Alternative considered**: pass a closure/continuation through the event instead of a stashed
enum. Rejected — GPUI events aren't naturally suited to carrying `'static` closures over `Arc`
item data cleanly, and the four request shapes are a small, closed set that an enum expresses
directly without extra machinery.

### Free-space query: add `fs4` for `available_space`

`fs4::available_space(path: &Path) -> io::Result<u64>` gives free bytes for the volume
containing `path`, cross-platform (macOS/Linux/Windows), with a small dependency footprint —
appropriate for the single field this app needs (`sysinfo`, already present transitively via
another dependency, is a far larger surface for the same one value and only relevant if the app
needed broader system telemetry, which it doesn't). Add a
`pub fn available_space_mb() -> Option<f64>` to `data/storage.rs`, calling
`fs4::available_space(&StorageConfig::load().root_path())`, `None` on any I/O error.

**Alternative considered**: `sysinfo::Disks`. Rejected per above — oversized for one field, and
would pull in platform-specific system-info collection this app has no other use for.

## Risks / Trade-offs

- **A user who dismisses the warning without confirming or cancelling explicitly** (e.g. clicks
  outside the dialog) leaves `pending_download` set with nothing consuming it. → Mitigation: the
  alert dialog's cancel path (or any dismissal) must explicitly call a
  `cancel_pending_download(cx)` that clears the field, not just closing the window — this needs
  an `.on_cancel` (or equivalent dismiss handler) alongside `.on_ok`, verified during
  implementation against `gpui_component`'s alert dialog API.
- **Free-space probe adds a small synchronous filesystem call on every queuing action.**
  `fs4::available_space` is a single fast syscall (`statvfs`/`GetDiskFreeSpaceEx`), not a
  directory walk — negligible compared to the network round-trips already involved in starting
  a download. No mitigation needed.
- **Race between the check and the actual download**: free space could still change between the
  check and the download completing (other apps writing to the same volume, or other queued
  downloads in this app consuming space concurrently). This is a pre-flight *warning*, not a
  guarantee — actual out-of-space failures during download are a separate, pre-existing failure
  mode this change doesn't need to solve.
