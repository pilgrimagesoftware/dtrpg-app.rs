## Why

The alert history panel (`Window > Show Alert History`) lists past error messages (failed
catalog loads, collection operations, downloads, etc.) so users can review what went wrong
after the transient activity toast has expired. Those messages are currently plain,
unselectable text — there's no way to copy an error message out of the panel to paste into a
bug report, a support request, or a search engine. A copy button removes that friction.

## What Changes

- Each entry row in the alert history panel gains a copy-to-clipboard button that copies the
  entry's error message text.
- The button follows the same appear-on-hover pattern already used for copyable fields in the
  detail panel (`gpui_component::clipboard::Clipboard`), rather than introducing a new
  interaction style.
- No change to what gets logged, how entries expire/cap, or the panel's header actions
  (Clear, close) — this only adds a copy affordance to existing rows.

## Capabilities

### New Capabilities
- `alert-history-copy-to-clipboard`: Alert history entries can be copied to the system
  clipboard via a per-row button.

### Modified Capabilities
<!-- none -->

## Impact

- `dtrpg-app/rust/crates/dtrpg-ui/src/ui/views/alert_history_view.rs`: `render_entry_row` gains
  a copy button next to (or below) the error message.
- Possibly `dtrpg-app/rust/crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: if the existing
  `copyable_value` helper is generalized/relocated for reuse rather than duplicated.
- Localization: a new tooltip string key alongside the existing `detail.copy_tooltip`, or reuse
  of that same key if the wording is generic enough.
