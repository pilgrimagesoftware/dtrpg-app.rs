## Context

`alert_history_view.rs::render_entry_row` renders each `AlertEntry` as a label, a truncated
error message, and a relative/absolute timestamp. There is no way to select or copy the
message text short of dragging a mouse selection over plain, non-interactive `div` text.

The codebase already has an established copy-to-clipboard pattern: `detail_panel_view.rs`'s
private `copyable_value` helper wraps a value in a `group`, and reveals a
`gpui_component::clipboard::Clipboard` button on hover via `group_hover`. This design reuses
that pattern rather than inventing a second one.

## Goals / Non-Goals

**Goals:**
- Let a user copy an alert entry's error message to the system clipboard with one click.
- Match the existing copy-button visual/interaction pattern (appear-on-hover, tooltip) already
  used in the detail panel, so the app doesn't grow a second "copy" affordance that looks or
  behaves differently.

**Non-Goals:**
- Copying the label or timestamp — only the error message is the thing worth pasting elsewhere
  (a bug report, a search query); the label is already visible as a small heading and the
  timestamp is not typically useful outside the app.
- Changing alert logging, capping, expiry, or the panel's Clear/close actions.
- A "copied!" toast or persistent confirmation state — `Clipboard`'s own hover tooltip is
  sufficient for this low-stakes action, consistent with the detail panel's existing usage.

## Decisions

- **Reuse `gpui_component::clipboard::Clipboard`, not a hand-rolled button.** It already handles
  the actual OS clipboard write; a hand-rolled `div` + manual clipboard call would duplicate
  behavior the crate provides for free and risks diverging in styling from the detail panel.
- **Relocate `copyable_value` out of `detail_panel_view.rs` into a shared location** (e.g.
  `crate::ui::widgets` or a small new `crate::ui::clipboard_field` module) rather than
  duplicating its ~15 lines into `alert_history_view.rs`. Two call sites is exactly the point
  where a private, single-file helper should move to shared code (per project style: reuse
  before duplicating). The helper's signature (`field_id`, `value`) is already generic enough;
  no changes needed to its body, only its location and visibility (`pub(crate)`).
  - Alternative considered: duplicate a small copy button directly in
    `alert_history_view.rs`. Rejected — two near-identical hover-reveal-clipboard
    implementations are more likely to drift in styling than one shared helper.
- **Button placement: inline with the error message row, not the label row.** The message is
  the thing being copied, so the affordance should appear where the user's attention already
  is when reading it, consistent with `copyable_value`'s pattern of placing the button directly
  next to the value it copies.
- **Localization: reuse `detail.copy_tooltip` rather than adding a new key.** The tooltip text
  ("Copy") is generic enough to apply to both the detail panel and the alert history panel; a
  new `alert_history.copy_tooltip` key would be a redundant translation to keep in sync across
  `en.yaml`/`de.yaml`/`fr.yaml` for identical wording.

## Risks / Trade-offs

- [Long error messages currently `.truncate()` in the row's visual layout, but `copyable_value`
  copies the full `value` string it's given, not the truncated display text] → No mitigation
  needed: `Clipboard::value(...)` is already given the untruncated `SharedString`, so what's
  copied is correct even though what's displayed is visually truncated. Worth confirming
  manually during verification (task 4) since this is easy to get backwards by accident.
- [Relocating `copyable_value` touches a file (`detail_panel_view.rs`) this change isn't
  otherwise modifying] → Low risk: the move is mechanical (cut/paste + `pub(crate)`), covered by
  `cargo check`/`clippy` catching any missed import at the old or new call site.
