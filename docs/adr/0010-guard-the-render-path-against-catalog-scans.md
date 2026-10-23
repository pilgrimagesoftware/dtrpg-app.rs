# 0010 - Guard the render path against catalog scans

## Status

Accepted.

## Context

GPUI re-renders the window on every keystroke, so a whole-catalog read placed
on the render path runs once per frame. `perf/render-path` (PR #432) removed
three such reads: the window total size walked every item each render pass,
the storage title counts recounted for every catalog page, and `is_loading`
read the index while a table asked for the loading state every frame. The
fixes replaced them with SQL sums, cached counts and `is_view_loading`.

Each fix was carried only by code and a commit message. Nothing failed when
the scans came back: feature branches cut before the fix were merged after
it and silently re-added the removed calls, and a developer reaching for the
obvious accessor (`all_items`, `catalog_ids`) puts a scan back with no signal
at the call site. CI ran the suite without rendering anything, so every
reintroduction passed. The sluggishness returned after each round of main
content changes, three fixes deep.

## Decision

Make the invariant checkable instead of remembered.

1. `dtrpg_core::scan_counter` counts, per thread, index reads that walk the
   whole catalog. Every such leaf query calls `catalog_scan!()` with
   `#[track_caller]`, and the wrapper chain the read passes through is
   `#[track_caller]` too, so the counter records the scan's real caller. A
   new index query that reads the catalog in full is annotated the same way.
2. `tests_render_guard` in `dtrpg_ui` reads the exact per-frame surface of
   the root view, catalog view and table delegates, then asserts the counter
   stayed at zero. A failure names the call site that scanned.
3. Whole-catalog accessors are named with an `_scan` suffix
   (`all_items_scan`, `catalog_ids_scan`); the cached equivalents
   (`visible_items_all`, `cached_catalog_ids`, `section_counts`) keep plain
   names. The cost is visible in the name at the call site.

## Consequences

The reintroduction pattern stops: any scan reaching the guarded surface fails
`cargo test` at the PR, including scans resurrected by a merge, and the
assertion points at the offender instead of a profile run.

The guard covers the reads it enumerates. A frame that begins reading through
a new accessor must be added to `read_one_frame`, and a scan dispatched to a
worker thread stays invisible to a test-thread counter — the render path
itself runs on the main thread, so that is the path guarded. The renames
touch every call site once; the counter costs a thread-local cell write
beside a full-table query, which is noise.
