## Context

Downloaded files live at `{root}/items/{sanitized publisher}/{filename}` (see `implement-real-file-downloads`), resolved via `StorageConfig::path_for_publisher` / `publisher_dir` in `crates/dtrpg-ui/src/data/storage.rs`. `LibraryController` already holds `self.collections: Vec<CollectionEntry>` (loaded via `CollectionsService`, see `crates/dtrpg-ui/src/data/collection.rs`), and `crate::util::matching::{collection_member_id, member_ids_contain}` already resolves whether a given `LibraryItem` belongs to a `CollectionEntry` — this is the same lookup the sidebar's collection filtering uses.

`dispatch_download` (in `crates/dtrpg-ui/src/controllers/library.rs`) is the single place a download transitions to complete: it resolves `order_product_id`/`index`/`dest` before spawning, then on success updates the item's status and resolves the activity entry. This is the natural insertion point for symlink creation — it already has `self.catalog` and (via `self`) `self.collections` available before the background spawn.

Storage settings (`create_collections`, alongside `max_concurrent_downloads`) persist through `StorageConfig`'s existing `StorageConfigFile` TOML round-trip in `save()`/`load()`.

## Goals / Non-Goals

**Goals:**
- A user can opt into a `collections/` mirror of their downloads, organized by collection name, without duplicating file bytes.
- Symlink creation is best-effort and never turns a successful download into a failed one.
- The mechanism works on macOS, Linux, and Windows using each platform's native symlink primitive.

**Non-Goals:**
- No retroactive symlink creation for items downloaded before the setting was enabled, or when collection membership changes after download — this proposal only acts at download-completion time. A "resync collections" action is a reasonable follow-on if this proves limiting in practice.
- No symlink cleanup when a download is removed or a collection is deleted — `remove_download` does not delete the underlying file today (see `confirm-download-removal`), so there is no natural trigger to clean up symlinks yet; this is deferred until real file removal exists.
- No handling of collection renames retargeting existing symlink directories — a rename leaves the old `collections/{old name}/` directory with stale symlinks (accepted, see Risks).
- No progress or per-symlink UI feedback; symlink creation is silent on success, logged on failure.

## Decisions

### Create symlinks inline in `dispatch_download`'s success path, on the calling (foreground) thread

Symlink creation is a fast, local filesystem metadata operation (unlike the download itself), so it does not need to run on the background executor. After `download_item` returns `Ok(())` and before the item's status flips to `Downloaded`, `dispatch_download` looks up which of `self.collections` the item belongs to (via `collection_member_id`/`member_ids_contain`, the same helpers `catalog_view.rs` already uses for collection filtering) and, if `StorageConfig::load().create_collections()` is `true`, creates one symlink per matching collection.

_Alternative considered_: Push symlink creation into `LibraryService::download_item` itself, alongside the real file write. Rejected — `download_item` is a service-layer trait method that only knows about a single destination path; collection membership is UI-controller-layer state (`self.collections`), so resolving it there would require threading collection data through the service trait, breaking the trait's existing single-purpose shape (`design.md` of `implement-real-file-downloads` made the same call for `dest`).

### One symlink per collection membership: `{root}/collections/{sanitized collection name}/{filename}`

Mirrors `publisher_dir`'s existing sanitization pattern (`crate::data::storage::sanitize_path_component`, made non-private or duplicated locally as needed) so a collection name containing a path separator can't escape `root`. An item that belongs to N collections gets N symlinks, one under each collection's directory — matches how the sidebar already lets an item appear under multiple collections simultaneously.

_Alternative considered_: A single `collections/` entry per item using its primary/first collection only. Rejected — collections are not exclusive in this app's model (an item can belong to several), so picking "the first one" would be arbitrary and surprising.

### Platform-conditional symlink creation, best-effort with logging only

```rust
#[cfg(unix)]
fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}
```

A failure (permissions, unsupported filesystem, Windows without symlink privilege) is logged via `tracing::warn!` and does not affect the download's own success/failure outcome or the activity panel entry — consistent with `reveal_in_file_manager`'s existing platform-conditional, best-effort pattern elsewhere in this crate. If the target symlink path already exists (e.g. re-downloading an item), creation is skipped rather than erroring — checked via `link.symlink_metadata().is_ok()` before calling `create_symlink`, avoiding an `AlreadyExists` I/O error path entirely.

_Alternative considered_: Surface symlink failures as a distinct activity-panel warning state. Rejected for this change — adds a new activity-panel state for a rare, non-critical failure mode; a log line is sufficient until real user feedback says otherwise.

### `create_collections` setting stored on `StorageConfig`, defaulting to `false`

Mirrors the existing `max_concurrent_downloads` field: a `bool` on `StorageConfigFile` with `#[serde(default)]` (defaulting `false` preserves current on-disk layout for users upgrading from a version without this field), exposed via `StorageConfig::create_collections()` / `set_create_collections(bool)`, and a new toggle control in the storage settings view alongside the existing location/concurrency controls.

_Alternative considered_: A separate preferences file/section outside `StorageConfig`. Rejected — this setting is specifically about how downloads are laid out on disk, which is exactly `StorageConfig`'s existing responsibility; adding a new config surface for one boolean would be unwarranted.

## Risks / Trade-offs

- [Risk] Symlinks can go stale: a manually deleted or moved file under `items/` leaves a dangling symlink under `collections/`, and a collection rename leaves an orphaned `collections/{old name}/` directory. → Accepted per Non-Goals; a future "resync collections" / cleanup pass is a reasonable follow-on, not blocking this change.
- [Risk] Windows requires either Developer Mode or an elevated process to create symlinks without `SeCreateSymbolicLinkPrivilege`; on a locked-down Windows install, every symlink creation may fail. → Mitigation: best-effort + logging (this design's core decision) means the download itself always still succeeds; the setting simply becomes a no-op in that environment. Consider documenting this limitation in the settings toggle's tooltip during implementation.
- [Trade-off] Symlink creation only happens at download-completion time, not on collection membership changes. → Accepted; keeps this change scoped to the download path rather than also hooking collection add/remove flows.
- [Risk] Two collections with names that sanitize to the same on-disk directory (e.g. "Foo/Bar" and "Foo_Bar" both sanitize to `Foo_Bar`) would have their symlinks land in the same `collections/` subdirectory. → Accepted as a pre-existing class of risk already present in `publisher_dir`'s sanitization approach (`implement-real-file-downloads`); not unique to this change.

## Open Questions

- Should the settings toggle immediately create symlinks for all currently-downloaded items when first enabled (a one-time backfill), rather than only affecting future downloads? Deferred to implementation discretion; the proposal's default (no backfill) is simpler and matches the "no retroactive action" behavior for toggling off, but a backfill may be worth a cheap follow-up if requested.
