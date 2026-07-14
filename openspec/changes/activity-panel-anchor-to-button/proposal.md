Issue: https://github.com/pilgrimagesoftware/dtrpg-app.rs/issues/57

## Resolution: already fixed, no implementation needed

The bug this proposal describes no longer exists on `develop`. Commit `47c0661` ("feat(activity):
Add direct panel open state setters") removed the hardcoded `.absolute().bottom(px(56.0))`
positioning from both `render_activity_panel` and `render_alert_history_panel`. Both panels are
now rendered by `status_bar_view.rs` as `gpui-component` `Popover`s anchored directly to their
trigger buttons (`Anchor::BottomLeft` / `Anchor::BottomRight`), which is exactly the behavior this
proposal asked for. Doc comments on both render functions and at the top of `status_bar_view.rs`
confirm this is the intended, current design.

No code changes were made under this change. Archived as obsolete without implementation.

## Why

`render_activity_panel` is anchored with `.absolute().bottom(px(56.0)).left(px(8.0))`, a
position left over from when the activity indicator lived in the sidebar footer. The
activity button that opens the panel now lives in the status bar's right-hand group (see
`status_bar_view.rs`), so the panel opens in the bottom-left corner of the window, visually
disconnected from the button that triggered it.

## What Changes

- The activity panel's anchor position is computed from the status bar activity button's
  on-screen bounds rather than a fixed bottom-left offset, so the panel opens directly
  above (or beside) the button that opened it.
- The alert history panel (`render_alert_history_panel`), which has the same
  bottom-left-anchor pattern tied to the notification button now in the status bar, gets
  the same anchor fix.

## Capabilities

### New Capabilities

- `activity-panel-anchoring`: The activity panel and alert history panel open anchored to
  the status bar button that triggered them, rather than a fixed window corner.

### Modified Capabilities

_(none — no landed spec currently governs status bar panel positioning; this introduces
that behavior as a new capability)_

## Impact

- `crates/dtrpg-ui/src/ui/views/activity_panel_view.rs`: Anchor position parameterized
  from the status bar button's bounds instead of hardcoded `bottom`/`left` offsets.
- `crates/dtrpg-ui/src/ui/views/root_view.rs`: Pass the activity/notification button's
  bounds (captured via `on_mouse_move`/layout query, or a stored `Bounds<Pixels>` set at
  render time) into `render_activity_panel` / `render_alert_history_panel`.
- `crates/dtrpg-ui/src/ui/views/alert_history_panel_view.rs` (or equivalent): Same anchor
  fix as the activity panel.
