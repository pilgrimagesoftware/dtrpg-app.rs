## Context

`ActivityController` currently has two ways an error reaches `alert_log`: `error(id,
message, cx)`, which requires a prior `start(...)` call and resolves that in-progress item
as failed. `create_collection` and `create_collection_and_add_member` use this correctly.
`add_item_to_collection` and `remove_item_from_collection` never call `start` — they apply
the change optimistically and only spawn a background confirmation — so there's no `id` to
resolve as errored on failure, and their failure branches fall through to only
`cx.emit(...Failed)`, which `root_view.rs` turns into a toast with no alert-log side effect.

## Goals / Non-Goals

**Goals:**
- Every collection-related failure that produces a toast notification also produces an
  alert history entry, matching the existing behavior of collection creation failures.

**Non-Goals:**
- Changing `add_item_to_collection`/`remove_item_from_collection` to use the
  `start`/`error` lifecycle. They're synchronous-feeling optimistic updates with no
  meaningful "in progress" state to show a spinner for; forcing them through `start` would
  add a spurious in-progress activity-panel row for an operation that already appears to
  have completed instantly.
- Touching the toast notification code in `root_view.rs` or the dialog's own inline error
  state in `manage_collections_dialog.rs` — both already work correctly and are unrelated
  to the alert-log gap.

## Decisions

- **New `log_alert` method rather than reusing `error`.** `error` mutates `in_progress`
  (removing the resolved item) as a precondition — calling it without a matching `start`
  would silently no-op (the `if let Some(pos) = ... find ...` guard). Adding a separate
  method that writes directly to `alert_log` (reusing the existing private `push_alert`
  helper) is simpler than retrofitting `start`/`error` calls into two optimistic-update
  methods that don't otherwise need activity-panel visibility.
- **Assign the new alert entry's `id` from the same `next_id` counter used by
  `start`.** Keeps alert entry ids unique across both code paths without introducing a
  second counter.

## Risks / Trade-offs

- [Two ways to reach `alert_log` (`error` via the start/error lifecycle, and the new
  `log_alert` for one-shot failures) adds a small amount of API surface to
  `ActivityController`] → Acceptable: the two call patterns are genuinely different (a
  resolved in-progress operation vs. a failure with no in-progress state), and conflating
  them would require a fake `start` call at each of the two existing call sites.
