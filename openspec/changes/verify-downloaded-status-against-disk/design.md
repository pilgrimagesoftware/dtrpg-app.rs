## Context

`LibraryItemFile::downloaded` is set once, optimistically, in
`dispatch_download`'s completion handler (`file.downloaded = true;`) and
otherwise never re-derived from disk. `fix-download-status-lost-on-restart`
and `fix-item-check-clears-download-status` (two sibling in-flight changes)
fix the two known ways the flag gets wrongly reset to `false` by a live
catalog reconcile or a per-item availability check, by merging the flag
through those overwrites. Neither change makes the flag *self-correcting*:
if a file is deleted outside the app, or was never actually written despite
the flag saying so, nothing notices until `ItemOpener::open` fails at
open-time with `OpenError::FileNotFound` — after the catalog has already
been showing `Downloaded` status and counting the item under "On This
Device."

The expected on-disk path for a file is already computed once, inline, in
`dispatch_download`:
`StorageConfig::load().path_for_publisher(&item.publisher).join(file.name.as_ref())`.
That's the same computation this change needs to reuse for verification —
there's no separate "where should this file be" concept to invent.

`start_load_inner` (the async catalog-load task) has three points where the
catalog settles into its final, displayed state for that load:
1. **Auto-load skip** (~`library.rs:847-868`): cache is fresh and the remote
   count matches, so the live fetch is skipped entirely and the on-disk
   cache's contents are shown as-is.
2. **Partial fetch success** (~`library.rs:900-928`): a growth-only
   additive fetch merges new/updated items via `apply_partial_fetch`.
3. **Full fetch success** (~`library.rs:1089-1114`, inside the `Ok(())` arm
   of `match fetch.await`): the complete live dataset is reconciled via
   `set_catalog`.

`select_item` (`library.rs:2298`) is the existing single-click selection
entry point, already used to trigger `maybe_check_item`'s on-demand
availability check — the natural place to also trigger an on-demand
file-presence check for that one item.

## Goals / Non-Goals

**Goals:**
- Make `downloaded` self-correcting in both directions: a file present on
  disk is marked `downloaded: true` regardless of what the flag said before;
  a file absent from disk is marked `downloaded: false` regardless of what
  the flag said before.
- Run this verification as a background-executor task so stat() calls never
  block the UI thread, matching the existing pattern for availability
  checks and thumbnail fetches.
- Cover both the catalog-wide case (every load) and the single-item case
  (selecting an item), per the chosen "both" trigger scope.

**Non-Goals:**
- Replacing or removing the two flag-preservation fixes
  (`fix-download-status-lost-on-restart`,
  `fix-item-check-clears-download-status`) — they still matter for reducing
  unnecessary status flicker between this verification pass's runs; this
  change is a backstop, not a replacement.
- Watching the filesystem for live changes (e.g. `notify`-based file
  watching) while the app is running with no user action and no catalog
  reload in progress. Verification is pull-based (triggered by a load or a
  selection), not push-based.
- Verifying files for items never selected and never part of a catalog
  reload cycle (there is no such state — every item goes through at least
  one catalog load).

## Decisions

- **New module `crates/dtrpg-ui/src/util/file_presence.rs`** owns:
  - `resolved_file_path(storage: &StorageConfig, item: &LibraryItem, file: &LibraryItemFile) -> PathBuf`
    — extracted from `dispatch_download`'s inline computation, so both the
    download path and this verification path share one source of truth for
    "where does this file live."
  - `verify_item_downloads(item: &mut LibraryItem, storage: &StorageConfig) -> bool`
    — sets every file's `downloaded` to `resolved_file_path(...).exists()`,
    calls `item.recompute_status()`, and returns whether anything actually
    changed (so callers only re-render/re-save when needed).
  - Placed in `util/`, not `controllers/library.rs`, per this repo's
    modular-file convention — `library.rs` is already large and this logic
    has no dependency on `LibraryController`'s async/gpui plumbing.
- **Catalog-wide verification runs as one background-executor pass per
  settled load**, not a per-item spawn: clone `ctrl.catalog`, load
  `StorageConfig` once, run `verify_item_downloads` over every item on the
  background executor, then apply the (possibly) mutated catalog back via
  a single `ctrl.update` that also recomputes `section_counts` and calls
  `invalidate_cache()`. One batch avoids spawning hundreds of tiny tasks
  for a large library and mirrors `save_catalog_cache`'s existing
  single-round-trip shape.
- **Called at all three catalog-settled points in `start_load_inner`**
  (auto-load skip, partial-fetch success, full-fetch success), not just the
  full-fetch path — the skip-fetch path is exactly the case most likely to
  be showing stale `downloaded` state indefinitely (that's the whole reason
  it was skipped), so it needs verification the most.
- **On-demand single-item verification in `select_item`**, spawned the same
  way as `maybe_check_item`'s existing availability check — independent of
  it (a missing/present file is a local disk fact, unrelated to whether the
  server still lists the item).
- **No cooldown on the on-demand path.** Unlike the network-bound
  availability check (which has `ITEM_CHECK_COOLDOWN_SECS` to avoid
  hammering the API), a local `Path::exists()` call per file is cheap
  enough that gating it adds complexity without a real cost problem to
  solve.
- **Bidirectional correction, not just upgrade-to-downloaded.** The
  proposal's motivating case (an externally-deleted file still showing
  `Downloaded`) requires downgrading `true → false`, not just detecting a
  previously-missed download. `verify_item_downloads` always sets
  `downloaded = path.exists()` rather than only setting it `true`.

## Risks / Trade-offs

- [Stat-ing every file in a large library on every load adds I/O] →
  Confined to a single background-executor task per load (not one task per
  file), and local filesystem `exists()` calls are cheap relative to the
  network-bound live fetch already happening in the same load cycle.
- [A file temporarily unavailable for a transient reason — e.g. a network
  drive briefly disconnected — gets marked not-downloaded even though it
  will reappear] → Matches the existing open-time behavior
  (`ItemOpener::open` already reports `FileNotFound` in this situation);
  this change makes the catalog's displayed status consistent with what
  opening the file would already show, not a new failure mode.
- [Verifying on every load duplicates work if nothing changed since the
  last verification] → Accepted: correctness here matters more than
  avoiding a cheap, infrequent (once per load) local stat pass; no caching
  of "already verified" state is introduced.
