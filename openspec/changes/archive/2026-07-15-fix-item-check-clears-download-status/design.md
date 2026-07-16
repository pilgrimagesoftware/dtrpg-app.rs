## Context

`apply_check_result(item: &mut LibraryItem, result: Result<LibraryItem, LibraryServiceError>, checked_at: SystemTime)`
is the shared completion handler for both the on-demand single-item check
(`maybe_check_item`, triggered by opening an item's details) and the
periodic background batch (`request_check_batch` → `enqueue_checks` →
`drain_check_queue` → `start_item_check`). On `Ok(fresh)` it does
`*item = fresh;` then restores only `id`/`numeric_id`/`order_product_id`/
`product_id`/`is_available`/`availability_last_checked` — every other field,
including `files[*].downloaded`, is silently overwritten with the fresh
single-item response's defaults (`downloaded: false` on every file, same as
the live catalog list mapping — the API has no local-download concept at
all). This is the identical defect `reconcile_catalog` had before
`fix-download-status-lost-on-restart`, in a second, independent call site
that fix didn't cover, since `apply_check_result` is a distinct function
serving a distinct trigger (per-item checks, not the catalog-wide live
fetch).

Because `ITEM_CHECK_BATCH_TIMER_SECS` (5 min) wakes the periodic batch
trigger repeatedly and `ITEM_CHECK_BATCH_COOLDOWN_SECS` (15 min) is the only
gate on an actual batch running, and `select_check_batch` selects items
*overdue* for a check (a just-downloaded item, never previously checked, is
maximally overdue), a freshly downloaded item is very likely to be swept
into the next periodic batch and revert to `Cloud` within roughly 15
minutes — matching the reported symptom of the "On This Device" count
reaching 0 even though real downloads exist.

Separately, `start_item_check`'s completion handler
(`crates/dtrpg-ui/src/controllers/library.rs`, inside the `cx.spawn` closure)
calls `ctrl.invalidate_cache()` after `apply_check_result`, which updates the
filtered `visible_items()` result, but never touches `ctrl.section_counts`
(the sidebar's badge counts). `section_counts` is otherwise only recomputed
at specific mutation points (`set_catalog`, `dispatch_download`,
`apply_partial_fetch`) — none of which run as part of a per-item check
completing.

## Goals / Non-Goals

**Goals:**
- Preserve per-file `downloaded` state (and the derived `status`) through a
  single-item check, on-demand or batched — the same guarantee
  `reconcile_catalog` already provides for the catalog-wide live fetch.
- Keep the sidebar's smart-section badge counts (`section_counts`)
  synchronized with the actual filtered result set after every check
  completion, not just after catalog loads and downloads.

**Non-Goals:**
- Changing the check-scheduling policy itself (cooldowns, batch size, batch
  timer interval) — this is a data-correctness fix, not a scheduling change.
- Verifying downloaded files still exist on disk during a check — that's the
  existing on-open file-presence check's job, unrelated to this defect.
- Deduplicating the merge logic between `reconcile_catalog` and
  `apply_check_result` into a shared helper. They operate on different
  shapes (`Vec<LibraryItem>` reconciliation vs. a single `&mut LibraryItem`
  update in place) and duplicating five lines of straightforward merge logic
  is simpler than introducing an abstraction for two call sites.

## Decisions

- **Apply the same merge as `reconcile_catalog`**: before `*item = fresh`,
  build a `HashMap<Arc<str>, bool>` of the existing item's
  `files[*].id → downloaded`; after the assignment, walk `item.files` and
  restore `downloaded` for any file whose id has an entry; call
  `item.recompute_status()` afterward. This is a direct application of the
  design already established (and tested) in `fix-download-status-lost-on-restart`.
- **Recompute `section_counts` inside `start_item_check`'s completion
  handler**, right after `ctrl.invalidate_cache()`, using the same
  `section_counts(&ctrl.catalog)` call already used elsewhere — no new
  computation logic, just adding the missing call site.
- **No change to `apply_check_result`'s existing identity/membership field
  preservation** (`id`, `numeric_id`, `order_product_id`, `product_id`) —
  that guard is unrelated to download state and stays exactly as-is.

## Risks / Trade-offs

- [A cached file id disappears from the fresh single-item response while
  still marked downloaded locally] → Same behavior as `reconcile_catalog`:
  the `downloaded` flag simply isn't carried over (no fresh file to attach
  it to); the file stays on disk and openable via the existing
  file-presence path, it just no longer counts toward the entry's `status`.
- [Recomputing `section_counts` on every check completion, including
  batched ones, adds a catalog-length scan per check] → Matches the cost
  already paid by `dispatch_download` on every download completion; check
  batches are capped at `ITEM_CHECK_BATCH_SIZE` (50) items and gated by a
  15-minute cooldown, so this isn't a hot loop.
