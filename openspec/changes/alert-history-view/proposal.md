## Why

The "Window > Show Alert History" menu item exists but is a stub — it only logs a tracing
event (`crates/dtrpg-ui/src/ui/views/root_view.rs`, `ShowAlertHistory` handler). There is no
alert history panel. Error activity items (failed catalog loads, thumbnail fetches,
collection operations, downloads) currently only appear in the transient activity panel,
where they expire after `ERROR_EXPIRY_SECS` (2 minutes) and are evicted once `RECENT_CAP`
non-expired items accumulate. Once expired or evicted, there is no way to review what went
wrong earlier in the session.

## What Changes

- `ActivityController` gains a second, non-expiring log — `alert_log: VecDeque<AlertEntry>`
  — capped at a fixed size (oldest entries drop off once full). Every call to `error(...)`
  appends an `AlertEntry` to this log in addition to the existing expiring `recent` list.
- `ActivityController` gains `alert_panel_open: bool` state and `toggle_alert_panel`,
  `clear_alert_log`, and `alert_snapshot()` methods, mirroring the existing activity panel
  pattern (`panel_open`, `toggle_panel`, `snapshot()`).
- New `AlertEntry` data type in `data/activity.rs`: `{ id, label, message, occurred_at:
  SystemTime }`. New `AlertHistorySnapshot { open: bool, entries: Vec<AlertEntry> }`.
- New `ui/views/alert_history_view.rs` renders an overlay panel listing alert entries newest
  first, each showing the label, error message, and a relative timestamp (`util::datetime::
  format_relative`) with an absolute-time tooltip. Includes an empty state and a "Clear"
  action in the header.
- The `ShowAlertHistory` action handler in `root_view.rs` now calls `toggle_alert_panel`
  instead of only logging. The panel is composed as a root-level overlay sibling (same
  pattern as the existing activity panel) so it paints above the rest of the window.

## Capabilities

### New Capabilities

- `alert-history-view`: A non-expiring, capped log of error-status activity items is
  maintained independently of the transient activity panel and can be reviewed via a
  "Window > Show Alert History" panel.

### Modified Capabilities

_(none — `ShowAlertHistory` action and menu item already existed as a stub from
`catalog-collections-improvements`; this change replaces the stub implementation)_

## Impact

- `dtrpg-ui/src/data/activity.rs` — add `AlertEntry`, `AlertHistorySnapshot`
- `dtrpg-ui/src/data/constants.rs` — add `ALERT_LOG_CAP`
- `dtrpg-ui/src/controllers/activity.rs` — add alert log field, panel state, and methods
- `dtrpg-ui/src/ui/views/alert_history_view.rs` — new file, panel rendering
- `dtrpg-ui/src/ui/views/root_view.rs` — wire `ShowAlertHistory` to the new panel, compose
  the overlay
- `dtrpg-ui/i18n/{en,de,fr}.yaml` — add `alert_history.*` strings
