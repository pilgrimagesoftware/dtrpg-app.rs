## Why

Clearing the alert history (the notifications/error log view reachable from the status bar)
does not clear the red unread-error badge on the status bar. The badge and the alert history
list are driven by two different pieces of state that were never wired together, so a user
who clears their notifications still sees a persistent red badge until the underlying
transient activity items individually expire on their own timer, minutes later.

## What Changes

- The status bar's unread-error badge (`has_errors` in `status_bar_view.rs`, driven by
  `ActivitySnapshot::recent_error_count`) SHALL clear when the user clears the alert history
  log via `ActivityController::clear_alert_log`.
- `recent_error_count` currently counts `Error` items in `ActivityController::recent` (a
  short-lived list that self-expires after `ERROR_EXPIRY_SECS`), which is a separate list
  from the durable `alert_log` the notifications view actually clears. This change makes
  clearing the alert log also dismiss the badge-relevant error state, without changing the
  existing auto-expiry behavior for errors the user never manually clears.

## Capabilities

### New Capabilities

- `activity-error-badge`: defines when the status bar's unread-error badge SHALL appear and
  SHALL clear, including its relationship to the alert history log.

## Impact

- `crates/dtrpg-ui/src/controllers/activity.rs` — `clear_alert_log`, `snapshot`, and the
  error-count computation.
- `crates/dtrpg-ui/src/ui/views/status_bar_view.rs` — reads `recent_error_count` to decide
  whether to render the badge; no rendering changes expected, only correctness of the
  underlying count.
