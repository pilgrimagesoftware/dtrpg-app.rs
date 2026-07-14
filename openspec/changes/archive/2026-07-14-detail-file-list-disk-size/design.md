## Context

Every downloaded entry's files live under `{storage_root}/items/{entry_id}/`. `ItemOpener::open_item` (`util/item_opener.rs`) already establishes the per-file path convention: a file's on-disk path is `entry_dir.join(file.name.as_ref())` — `file.name` is both the display name and the expected filename on disk. The detail view currently shows `file.size_mb` (catalog-reported, from `LibraryItemFile`) in two places: the per-file `render_item_metadata` (multi-item entries) and the top-level entry metadata block (single-file entries, using `item.size_mb`), both in `detail_panel_view.rs`.

No real download service exists yet (see `download-queue-concurrency-control`'s design), so today no entry's `items/{id}/` directory is ever actually populated — `.exists()` checks on it already return `false` everywhere they're used (e.g. the "Reveal in Finder" warning). This change reads whatever is on disk at render time; it does not depend on the download service landing first, and will start showing real sizes automatically once real files exist there.

## Goals / Non-Goals

**Goals:**
- The catalog-reported size is always shown; when a `Downloaded` item's file has a resolvable on-disk path, its actual byte size is shown alongside it as a `"(X.X MB on disk)"` suffix, formatted consistently with the existing `"{:.1} MB"` / `"{:.0} MB"` style.
- No on-disk figure is shown whenever the file isn't present (not downloaded yet, removed, or unresolvable) — no error state, no blank field, just the catalog size alone.
- Multi-file entries show a *combined* catalog size (sum of every file's `size_mb`) at the top level, with an on-disk suffix that is itself the sum of on-disk sizes for whichever files are actually present — and a label distinguishing this total from a single file's size.
- The Items table (multi-item entry's file list) gains a "Size" column using the same catalog-plus-on-disk-suffix format, per row.
- Single helper function used by every render site (top-level single-file field, top-level combined field, Items table rows, per-file selected-item metadata).

**Non-Goals:**
- No caching or background pre-computation of on-disk sizes — `std::fs::metadata` on a single file is a cheap, synchronous syscall, consistent with how `check_storage_path_exists`-adjacent code already treats path/existence checks as render-cheap (though that one is async; see Decisions).
- No change to `LibraryItemFile`/`LibraryItem` data models — the on-disk size is derived at render time, not stored.
- No handling for partially-downloaded/temp (`.part`) files — out of scope until the real download service (and its `.part` convention, noted as a future mitigation in `download-queue-concurrency-control`'s design) exists.
- No partial-combined-total indicator (e.g. "3 of 5 files on disk") — the on-disk suffix is simply the sum over whichever files resolve; a multi-file entry that's only partially downloaded shows a combined on-disk figure smaller than the combined catalog figure, which is self-explanatory without extra UI.

## Decisions

### Resolve per-file path via `entry_dir.join(file.name.as_ref())`, matching `ItemOpener`

Reuses the exact convention `util/item_opener.rs::open_item` already established, rather than inventing a second file-path resolution rule. For the single-file top-level field, this is `entry_dir.join(item.files[0].name.as_ref())` when `item.files` has exactly one entry; when `item.files` is empty (legacy/stub items with no file breakdown), there's no filename to join, so it falls back to catalog size the same as a missing file would.

_Alternative considered_: Sum the sizes of every file directly inside `entry_dir` (a `read_dir` scan) instead of resolving specific filenames. Rejected — doesn't distinguish between a bundle's individual files (the file list needs one size per row, not one aggregate), and a directory-scan-based aggregate would silently include stray/leftover files.

### Compute size synchronously at render time, not via a background task

Existing per-item filesystem checks in this codebase are split: `path.exists()` for a single "is it downloaded" check is done both synchronously (e.g. reveal-in-Finder's inline `!item_path.exists()`) and asynchronously (`SettingsController::check_storage_path_exists`, for the one global storage-root check on settings open). A per-file `metadata()` call in a render function is the same class of cheap, synchronous operation as the existing inline `.exists()` checks already sprinkled through `detail_panel_view.rs` — no `Context`/`cx.spawn` plumbing needed, and adding an async round-trip per file row would flicker the field between "catalog size" and "real size" on every render, which is worse UX than a direct call.

_Alternative considered_: Cache resolved sizes in `LibraryController` state, refreshed on download completion. Rejected as premature — no real download completion event exists yet to invalidate the cache against (see `download-queue-concurrency-control`), and a stat-on-render is cheap enough that a cache adds complexity without a measurable benefit at today's file-list scale (one entry's files, not the whole catalog, per render).

### New helper: `on_disk_file_size(entry_dir: &Path, file_name: &str) -> Option<u64>`

A small, pure(-ish) helper — resolves the path, calls `std::fs::metadata`, returns `Some(bytes)` on success or `None` on any failure (missing file, permission error, etc.).

### Composite display: catalog size always shown, on-disk size appended as a suffix

Each render site now builds its string as `format!("{catalog} {suffix}")` where `catalog` is the existing `"{:.1} MB"`/`"{:.0} MB"`-style formatting of the catalog size (unchanged), and `suffix` is `t!("detail.on_disk_suffix", size = format_bytes(bytes))` (e.g. `"(11.8 MB on disk)"`) when at least one byte count resolved, or an empty string otherwise. This replaces the original replace-catalog-with-on-disk approach: the catalog figure is authoritative/comparable across items regardless of download state, and the on-disk figure is now presented as corroborating detail rather than a silent substitution — a user comparing sizes across a mixed cloud/downloaded catalog sees the same catalog-size baseline everywhere, with on-disk detail layered on top only where it's known.

_Alternative considered_: keep the original "replace when downloaded, fall back otherwise" behavior. Superseded per updated requirements — showing only one number hides the (occasionally informative) difference between what the catalog reports and what's actually on disk, and loses the reference catalog figure entirely once a file is downloaded.

### Multi-file top-level field: sum `item.files[].size_mb` and (independently) sum resolved on-disk sizes

For any entry with `item.files` non-empty, the top-level catalog figure is `item.files.iter().map(|f| f.size_mb).sum()` rather than `item.size_mb` — this unifies single- and multi-file entries under one computation (a single-file entry's sum trivially equals that one file's size) and matches what the Items table's per-row figures actually add up to, rather than potentially drifting from a separately-reported entry-level `size_mb`. The on-disk suffix sums `on_disk_file_size(...)` only over files that resolve (`Some`); if none resolve, no suffix is shown, matching the single-file fallback behavior. When `item.files` is empty (legacy/stub entries with no file breakdown), the field falls back to `item.size_mb` alone, as today.

The label switches to `t!("detail.field_total_file_size")` ("Total file size") when `item.files.len() > 1`, and stays `t!("detail.field_file_size")` ("File size") otherwise — the two labels are the same in every locale's existing string catalog by structure, just a new key added alongside.

_Alternative considered_: keep using `item.size_mb` for the top-level single-file case and only sum for multi-file. Rejected — a per-`files.len()` branch in the size computation (as opposed to only in the label) would mean the number and the on-disk suffix could be computed via two different code paths for what's conceptually the same "sum over files" operation, doubling the surface for the two to drift out of sync.

### Items table gains a "Size" column, same composite format per row

`render_item_tier`'s `header_row` gains a fourth `TableCell` labeled `t!("detail.item_list_column_size")`, and each row's `row_content` flex row gains a fourth `flex_1` child showing `on_disk_file_size(...)`-composed text for that row's `file`, built via the same shared formatter used everywhere else in this change. This reuses the existing `entry_dir` resolution (`publisher_dir(storage_root_path, &item.publisher)`) already threaded into `render_item_tier`.

## Risks / Trade-offs

- [Risk] A `metadata()` syscall per file row on every detail-tab render could add up for an entry with many files, and the top-level combined figure now does one such call per file too (previously it stopped at the first single-file case). → Mitigation: file lists in this app are per-entry (typically single digits, not hundreds), and `metadata()` is a single stat call; if this ever proves measurably slow, memoizing per `(entry_id, file_name)` pair in `LibraryController` is a contained follow-on change.
- [Trade-off] Byte-precise on-disk size vs. the catalog's MB-rounded size will occasionally look "different" for the same file even when nothing is wrong (rounding). → Accepted: this is the point of the change — the on-disk number is the more accurate one, now shown as a supplement rather than a silent replacement.
- [Trade-off] A partially-downloaded multi-file entry shows a combined on-disk figure smaller than the combined catalog figure, with no explicit "X of Y files" indicator. → Accepted per Non-Goals: the two numbers being different is self-explanatory in context.
