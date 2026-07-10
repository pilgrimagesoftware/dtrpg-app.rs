## Context

`alert_history_view.rs::render_entry_row` renders `entry.message` as a plain `div().child
(message)` with no interaction. `detail_panel_view.rs` already solved "copy this text" for
detail panel fields via a `copyable_value` helper: a hover-revealed
`gpui_component::clipboard::Clipboard` widget next to the value, with a tooltip. There's no
existing shared helper for this pattern outside `detail_panel_view.rs` — it's a private
`fn` in that file.

## Goals / Non-Goals

**Goals:**
- Let a user copy an alert history entry's error message with one click.
- Reuse the established `Clipboard` + hover-reveal visual pattern rather than inventing a
  new one.

**Non-Goals:**
- Copying the label or timestamp — only the error message text, since that's the field
  users need for bug reports.
- Adding copy support to the transient activity panel's tooltip/expanded error text (out of
  scope for this change; the activity panel already loses the message on expiry, and this
  change only targets the durable alert history log).
- Extracting `copyable_value` into a shared module — it's a small, single-use-site helper
  in `detail_panel_view.rs`; duplicating a few lines in `alert_history_view.rs` is simpler
  than adding a cross-module dependency for one call site each.

## Decisions

- **Add the copy control inline in `render_entry_row`, not by importing
  `detail_panel_view::copyable_value`.** That function is private to its module and the
  two call sites render inside different layout contexts (a metadata table row vs. an
  alert list row) — copying the ~15-line pattern locally, scoped to
  `alert_history_view.rs`, avoids making the two views depend on each other for a small
  hover-reveal widget.
- **Hover-reveal on the row, not always-visible.** Matches the existing detail panel
  convention and keeps the message-only row visually quiet when not being interacted with.
- **Copy the raw `entry.message` string, not a formatted "label: message".** The message is
  already what's useful to paste into a report; prefixing it with the label the row already
  shows next to it would be redundant.

## Risks / Trade-offs

- [Minor duplication of the hover-reveal-copy pattern between `detail_panel_view.rs` and
  `alert_history_view.rs`] → Acceptable at two call sites; extract to a shared helper only
  if a third view needs the same pattern (YAGNI).
