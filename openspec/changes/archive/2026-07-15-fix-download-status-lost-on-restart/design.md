## Context

`LibraryController::start_load` loads the cached catalog on startup, displays
it immediately, then kicks off a live SDK fetch in the background
(`catalog-live-data-swap`). When that fetch completes, `set_catalog` calls
`reconcile_catalog(existing, live)` whenever a local baseline exists
(`reconcile = !catalog_was_empty`), which is the normal restart path.

`reconcile_catalog` matches items by id and, for a match, does
`item = live_item; item.is_available = true;` — a wholesale replace. Live
items always carry `downloaded: false` on every `LibraryItemFile`
(`crates/dtrpg-core/src/services/sdk/library/mapping.rs`), because the API
has no notion of what's on the user's disk. The replace silently discards the
cached item's `files[*].downloaded` flags and, with them, the aggregate
`status` (`Downloaded` → `Cloud`) — this is one half of the bug.

The other half: `save_catalog_cache` is only called from two places, both
inside `start_load_inner` (the full-fetch completion at line ~1109 and the
partial-fetch completion at line ~933). `dispatch_download`'s completion
handler sets `file.downloaded = true` and calls `item.recompute_status()` on
`self.catalog` in memory, but never writes that catalog to disk. The
auto-load policy (`start_load_inner`, ~lines 837-951) skips the live fetch
entirely when the on-disk cache is fresh (< 7 days old) and the remote item
count matches the cached count — the common case for a quick
download-then-restart — so a fixed `reconcile_catalog` never even runs; the
stale on-disk cache is shown as-is, still missing the download.

## Goals / Non-Goals

**Goals:**
- Preserve per-file `downloaded` state (and the derived entry `status`)
  through a live-fetch reconcile.
- Persist a successful download to the on-disk catalog cache immediately, so
  the cache doesn't depend on the next live fetch to reflect it.
- Keep the fix scoped to `reconcile_catalog` and `dispatch_download`'s
  completion handler; no changes to the SDK mapping, cache schema, or the
  auto-load policy's freshness/count-check logic.

**Non-Goals:**
- Verifying downloaded files still exist on disk during reconcile (that's the
  existing on-open/on-demand file-presence check's job, not reconcile's).
- Changing behavior for items only in `live` (new items) or only in
  `existing` (server no longer lists them) — both are already correct.
- Debouncing or batching the post-download cache write. Each successful
  download writes the full catalog once; this matches the existing
  full/partial-fetch save calls and downloads are user-paced, not a hot loop.

## Decisions

- **Merge `downloaded` by file `id`, not by index.** File order isn't
  guaranteed stable between a cached snapshot and a fresh API response
  (e.g. after the publisher reorders files). Matching by `LibraryItemFile::id`
  is the same key `dedupe_files` and `recompute_status` already treat as
  authoritative.
- **Take the live item's file list as the base, not the cached one.** The live
  fetch is the source of truth for file metadata (name, format, size,
  index) — a file could be renamed, resized, or removed upstream. Only the
  `downloaded` flag is carried over from the cached file with a matching id;
  a live file with no cached counterpart keeps `downloaded: false` (it's
  either new or wasn't downloaded before).
- **Call `recompute_status()` after the merge**, rather than trying to compute
  the right `status` inline, so `reconcile_catalog` stays consistent with
  every other call site that mutates `files[*].downloaded`.

- **Save unconditionally on any successful, non-cancelled download**, rather
  than trying to detect whether the download actually changed persisted
  state. `dispatch_download` already only reaches the save point after
  confirming `!cancelled && outcome.is_ok()`, so this is a single
  `save_catalog_cache` call guarded the same way the existing full/partial
  fetch saves are — no new conditional logic to get wrong.

## Risks / Trade-offs

- [A cached file id disappears from the live response while still marked
  downloaded locally] → Its `downloaded` flag is simply not carried over
  (no live file to attach it to); the file stays on disk and openable via the
  existing file-presence path, it just no longer counts toward the entry's
  `status`. This matches how a publisher-side file removal should behave.
- [Every successful download now does a full catalog write to disk] → Matches
  the cost already paid by the full/partial-fetch save paths; catalogs are
  small JSON and downloads are infrequent relative to page loads, so this
  isn't a new hot path.
