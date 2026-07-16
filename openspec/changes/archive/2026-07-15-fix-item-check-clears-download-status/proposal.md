## Why

The "On This Device" sidebar section reports a count of 0 even when files are
genuinely downloaded, and selecting it shows an empty view. The cause is in
`apply_check_result` (`crates/dtrpg-ui/src/controllers/library.rs`), which
backs both the on-demand single-item check (triggered by opening an item's
details) and the periodic background check batch (`request_check_batch`,
firing every `ITEM_CHECK_BATCH_TIMER_SECS`). On a successful check it does
`*item = fresh;` — a wholesale replacement with the single-item API
response. Like the live catalog fetch's mapping, that response always carries
`downloaded: false` on every file, because the API has no notion of what's on
the user's disk. Within `ITEM_CHECK_BATCH_COOLDOWN_SECS` of any download
completing, the periodic batch (or an on-demand detail-view check) silently
flips that item's files back to not-downloaded and its status back to
`Cloud` — this is the same defect class already fixed for the live-catalog
reconcile path in `fix-download-status-lost-on-restart`, but in a second,
independent code path that fix didn't touch.

A related, compounding bug: `start_item_check`'s completion handler calls
`ctrl.invalidate_cache()` after applying a check result, but never
recomputes `ctrl.section_counts`. So even where the visible "On This Device"
list is correct, the sidebar badge count can lag behind it after a check
completes, until some unrelated event (a download, a full catalog reload)
happens to recompute `section_counts` from scratch.

## What Changes

- `apply_check_result` preserves each file's existing `downloaded` flag
  (matched by file `id`) when adopting the fresh single-item response's
  fields, then recomputes the item's `status`, mirroring
  `reconcile_catalog`'s fix.
- `start_item_check`'s completion handler recomputes `ctrl.section_counts`
  immediately after applying a check result, so the sidebar badge never lags
  the actual filtered list.

## Capabilities

### New Capabilities

- `item-check-preserves-downloaded-state`: A single-item availability check
  (on-demand or from the periodic batch) MUST NOT discard the local
  downloaded state of an item's files.

### Modified Capabilities

_(none — no existing capability spec governs `apply_check_result`'s
per-file behavior or `section_counts` freshness after a check; see the new
capability above)_

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: `apply_check_result` gains
  per-file `downloaded` merge logic and a `recompute_status()` call;
  `start_item_check`'s completion handler gains a `section_counts`
  recompute.
