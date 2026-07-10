## Context

Every downloaded entry's files live under `{storage_root}/items/{entry_id}/`. `ItemOpener::open_item` (`util/item_opener.rs`) already establishes the per-file path convention: a file's on-disk path is `entry_dir.join(file.name.as_ref())` — `file.name` is both the display name and the expected filename on disk. The detail view currently shows `file.size_mb` (catalog-reported, from `LibraryItemFile`) in two places: the per-file `render_item_metadata` (multi-item entries) and the top-level entry metadata block (single-file entries, using `item.size_mb`), both in `detail_panel_view.rs`.

No real download service exists yet (see `download-queue-concurrency-control`'s design), so today no entry's `items/{id}/` directory is ever actually populated — `.exists()` checks on it already return `false` everywhere they're used (e.g. the "Reveal in Finder" warning). This change reads whatever is on disk at render time; it does not depend on the download service landing first, and will start showing real sizes automatically once real files exist there.

## Goals / Non-Goals

**Goals:**
- Downloaded files with a resolvable on-disk path show their actual byte size, formatted consistently with the existing `"{:.1} MB"` / `"{:.0} MB"` style.
- Falls back to the existing catalog-reported size whenever the file isn't present (not downloaded yet, removed, or unresolvable) — no error state, no blank field.
- Single helper function used by both render sites (top-level single-file field and per-file multi-item rows).

**Non-Goals:**
- No caching or background pre-computation of on-disk sizes — `std::fs::metadata` on a single file is a cheap, synchronous syscall, consistent with how `check_storage_path_exists`-adjacent code already treats path/existence checks as render-cheap (though that one is async; see Decisions).
- No change to `LibraryItemFile`/`LibraryItem` data models — the on-disk size is derived at render time, not stored.
- No handling for partially-downloaded/temp (`.part`) files — out of scope until the real download service (and its `.part` convention, noted as a future mitigation in `download-queue-concurrency-control`'s design) exists.

## Decisions

### Resolve per-file path via `entry_dir.join(file.name.as_ref())`, matching `ItemOpener`

Reuses the exact convention `util/item_opener.rs::open_item` already established, rather than inventing a second file-path resolution rule. For the single-file top-level field, this is `entry_dir.join(item.files[0].name.as_ref())` when `item.files` has exactly one entry; when `item.files` is empty (legacy/stub items with no file breakdown), there's no filename to join, so it falls back to catalog size the same as a missing file would.

_Alternative considered_: Sum the sizes of every file directly inside `entry_dir` (a `read_dir` scan) instead of resolving specific filenames. Rejected — doesn't distinguish between a bundle's individual files (the file list needs one size per row, not one aggregate), and a directory-scan-based aggregate would silently include stray/leftover files.

### Compute size synchronously at render time, not via a background task

Existing per-item filesystem checks in this codebase are split: `path.exists()` for a single "is it downloaded" check is done both synchronously (e.g. reveal-in-Finder's inline `!item_path.exists()`) and asynchronously (`SettingsController::check_storage_path_exists`, for the one global storage-root check on settings open). A per-file `metadata()` call in a render function is the same class of cheap, synchronous operation as the existing inline `.exists()` checks already sprinkled through `detail_panel_view.rs` — no `Context`/`cx.spawn` plumbing needed, and adding an async round-trip per file row would flicker the field between "catalog size" and "real size" on every render, which is worse UX than a direct call.

_Alternative considered_: Cache resolved sizes in `LibraryController` state, refreshed on download completion. Rejected as premature — no real download completion event exists yet to invalidate the cache against (see `download-queue-concurrency-control`), and a stat-on-render is cheap enough that a cache adds complexity without a measurable benefit at today's file-list scale (one entry's files, not the whole catalog, per render).

### New helper: `on_disk_file_size(entry_dir: &Path, file_name: &str) -> Option<u64>`

A small, pure(-ish) helper — resolves the path, calls `std::fs::metadata`, returns `Some(bytes)` on success or `None` on any failure (missing file, permission error, etc.), so both call sites collapse to `on_disk_file_size(...).map(format_bytes).unwrap_or_else(|| format!("{:.1} MB", file.size_mb))`. Lives in `crate::util` (a new small module, e.g. `util/file_size.rs`) rather than inline in the view, so it's unit-testable without a GPUI context — consistent with how `util/item_opener.rs` and other `util/*` helpers in this crate are already structured and tested.

## Risks / Trade-offs

- [Risk] A `metadata()` syscall per file row on every detail-tab render could add up for an entry with many files. → Mitigation: file lists in this app are per-entry (typically single digits, not hundreds), and `metadata()` is a single stat call; if this ever proves measurably slow, memoizing per `(entry_id, file_name)` pair in `LibraryController` is a contained follow-on change.
- [Trade-off] Byte-precise on-disk size vs. the catalog's MB-rounded size will occasionally look "different" for the same file even when nothing is wrong (rounding). → Accepted: this is the point of the change — the on-disk number is the more accurate one.
