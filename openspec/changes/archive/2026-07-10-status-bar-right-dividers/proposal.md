## Why

The status bar's right-hand side groups three unrelated controls — the theme picker, the
activity indicator, and the notifications button — with no visual separation between them.
The left-hand side already uses a `Separator::vertical()` between the library total and
active-tab summary, so the right side reads inconsistently by comparison and the three
buttons can look like a single control cluster rather than three distinct actions.

## What Changes

- Insert a `Separator::vertical()` between the theme picker and the activity indicator,
  and another between the activity indicator and the notifications button, on the status
  bar's right side — matching the existing left-side divider pattern in the same view.
- No behavior change to any of the three controls; purely a layout/visual addition.

## Capabilities

### New Capabilities

- `status-bar-button-group-dividers`: the status bar has no existing spec at all (the
  module's own doc comment references a `main-window-status-bar` capability that was
  never actually written to `openspec/specs/`). This change introduces a narrowly-scoped
  spec for the one behavior it changes — visual separation between the right-side
  buttons — rather than leaving it undocumented or overreaching into an unrelated
  full status-bar spec.

### Modified Capabilities

<!-- none -->

## Impact

- `crates/dtrpg-ui/src/ui/views/status_bar_view.rs`: `render_status_bar`'s `StatusBar`
  builder chain gains two additional `.right(Separator::vertical())` calls interleaved
  between the existing `.right(theme_picker)`, `.right(activity_indicator)`, and
  `.right(notification_indicator)` calls.
- No changes to any other file — `Separator` is already imported and used in this file.
