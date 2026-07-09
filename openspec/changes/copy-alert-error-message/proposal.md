## Why

The alert history panel (`alert_history_view.rs`, "Window > Show Alert History") shows the
full error message text for each past failure, but the only way to get that text out is to
select it by hand — there's no copy affordance. Error messages here often include useful
detail (URLs, status codes, decode errors) a user might want to paste into a bug report or
support request, and manual text selection inside a fixed-height popover row is error-prone.

## What Changes

- Each alert history entry row (`render_entry_row` in `alert_history_view.rs`) gains a copy
  affordance for its error message, using `gpui_component::clipboard::Clipboard` — the same
  component already used for copyable fields in the detail panel
  (`detail_panel_view.rs::copyable_value`).
- The copy control appears on hover (matching `copyable_value`'s
  `.group_hover(...).visible()` pattern) next to the message text, copies `entry.message`
  verbatim, and shows a tooltip (`alert_history.copy_tooltip`).
- No change to `AlertEntry`, the activity controller, or the alert log's data model — this
  is a presentation-only addition to the existing panel.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `alert-history-view`: alert history entries can have their error message copied to the
  clipboard directly from the panel, in addition to being displayed.

## Impact

- `dtrpg-ui/src/ui/views/alert_history_view.rs` — add copy control to `render_entry_row`
- `dtrpg-ui/i18n/{en,de,fr}.yaml` — add `alert_history.copy_tooltip` key
