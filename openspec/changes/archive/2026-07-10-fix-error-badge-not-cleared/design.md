## Context

`ActivityController` (`crates/dtrpg-ui/src/controllers/activity.rs`) tracks two separate
error-related lists:
- `recent: VecDeque<ActivityItem>` — short-lived, each item self-expiring after
  `ERROR_EXPIRY_SECS` (2 minutes) via a scheduled `expire_item` call.
- `alert_log: VecDeque<AlertEntry>` — durable, explicitly documented as "never cleared by
  `clear()`", only emptied by the user via `clear_alert_log`.

The status bar's notification bell popover renders `alert_log` (via `AlertHistorySnapshot`)
as the "notifications view" — this is what the user clears. The red badge dot and the
tooltip's "n unread errors" text, however, both read `ActivitySnapshot::recent_error_count`,
which is computed by filtering `recent` for `Error` status. These are two independent
counters that happen to move together only by coincidence (an error is pushed into both
lists at the same moment in `error()`), and diverge as soon as either list changes on its
own: `recent`'s entry silently expires after 2 minutes regardless of user action, and
`clear_alert_log` empties `alert_log` without touching `recent` at all — so the badge can
stay lit after the user has cleared everything they can see, or (less visibly) disappear on
its own timer before the user ever looked.

## Goals / Non-Goals

**Goals:**
- Make the badge/tooltip and the notifications view agree: the badge SHALL be present if
  and only if the notifications view (`alert_log`) is non-empty.
- Clearing the notifications view SHALL immediately clear the badge.

**Non-Goals:**
- Adding a per-entry "read/unread" concept to `AlertEntry` — the existing behavior is
  binary (badge shows if there are any entries), and the bug report only asks for the
  clear action to work; a finer-grained read-state model is unrequested scope.
- Changing `recent`/`ERROR_EXPIRY_SECS` transient-item expiry behavior in the activity
  panel itself — that panel's own display of recently-completed items (successes and
  errors alike) is unaffected; only the badge's data source changes.

## Decisions

**Drive the badge count from `alert_log.len()` instead of a filtered count of `recent`.**
`alert_log` is already the single source of truth for "what's in the notifications view";
computing the badge from anything else is what created the divergence. Once the badge
reads `alert_log.len()` directly, `clear_alert_log` clearing `alert_log` and the badge
disappearing become the same event by construction — no separate synchronization code is
needed.

**Rename `ActivitySnapshot::recent_error_count` to `alert_count`.** The field's only two
call sites are the badge dot and the tooltip text in `status_bar_view.rs`. Keeping the old
name after changing its source (`recent` → `alert_log`) would leave a name that no longer
describes what it holds — the next person to touch this code would reasonably assume
"recent" still means the transient list. Renaming is a small, mechanical change confined to
`data/activity.rs`, `controllers/activity.rs`, and `ui/views/status_bar_view.rs`.

**Why not instead clear `recent`'s error items from `clear_alert_log`?** That would fix the
symptom but leave two counters that still drift apart the next time `recent` expires an
item on its own timer while `alert_log` still holds the corresponding entry (the common
case, since `alert_log` never auto-expires). Reading `alert_log.len()` directly removes the
divergence at its source instead of re-synchronizing two lists on every mutation.

## Risks / Trade-offs

- **Badge now persists longer than before in the common case** (previously it could
  self-clear after 2 minutes via `recent`'s expiry; now it only clears when the user
  clears the notifications view) → this is the intended fix, not a regression: a badge
  that silently disappears without the user having seen or acknowledged the error is the
  more confusing behavior of the two, and the proposal's premise is that clearing should be
  the action that dismisses it.
- **Rename touches three files** → mechanical, no behavior change beyond the fix itself;
  covered by existing/updated unit tests on `ActivityController::snapshot`.

## Migration Plan

No data migration. In-memory-only state; the rename is source-only and has no on-disk or
serialized representation.
