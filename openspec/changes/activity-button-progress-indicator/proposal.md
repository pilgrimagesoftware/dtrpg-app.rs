## Why

The status bar's activity indicator (`render_status_bar` in `status_bar_view.rs`) is a
plain `Button` whose label is a glyph (spinner arrow, dot, or empty circle) plus a raw
count of in-progress + recent items. It was designed to be a circular progress indicator —
`gpui-component` ships `progress::ProgressCircle` for exactly this — showing the aggregate
completion of all active background loaders (catalog load, thumbnail queue, downloads) at
a glance, rather than a text glyph the user has to learn to read.

## What Changes

- The activity indicator in the status bar becomes a `ProgressCircle`, sized to fit the
  status bar row, with `value` derived from the sum of all active `ActivityController`
  loaders' completed/total counts (falling back to an indeterminate/spinning state when a
  loader has no known total, e.g. thumbnail queue draining).
- When there are no in-progress or recent items, the circle renders in an idle/empty state
  (no fill, no animation) instead of the current "○" glyph.
- The click behavior (`toggle_panel`) and tooltip (in-progress/completed counts) are
  unchanged.

## Capabilities

### New Capabilities

- `status-bar-activity-progress`: The status bar's activity indicator is a circular
  progress element reflecting the aggregate completion of all active background loaders,
  replacing the glyph-and-count button label.

### Modified Capabilities

_(none — this replaces the indicator's visual representation without changing the click
action, tooltip content, or `ActivityController` event flow)_

## Impact

- `crates/dtrpg-ui/src/ui/views/status_bar_view.rs`: `render_status_bar` builds a
  `gpui_component::progress::ProgressCircle` in place of the glyph `Button` label; the
  `Button`'s click handler and tooltip wrap the circle instead.
- `crates/dtrpg-ui/src/controllers/activity.rs` (or equivalent): may need an aggregate
  completed/total accessor if one does not already exist, summing across concurrent
  loaders (catalog load, thumbnail queue, downloads).
- Supersedes the glyph-legibility fix in `activity-panel-button-icon-size` — once the
  indicator is a `ProgressCircle` there is no text glyph to resize; that change should be
  dropped or folded in if not yet landed.
