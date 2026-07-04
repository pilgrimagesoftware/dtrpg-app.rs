## Context

`item_popover_view.rs` builds a `DescriptionList` for a `LibraryItem` but
leaves a `TODO` where a date-added row should go. `detail_panel_view.rs`
already solves the identical problem for the expanded detail tab: it formats
`item.date_added` (an `Option<i64>` Unix timestamp) via `format_relative` and
`format_absolute` from `util/datetime.rs`, and wraps the relative string in a
`div` with a `Tooltip` showing the absolute date.

## Goals / Non-Goals

**Goals:**
- Show a date-added row in the popover, matching the detail panel's
  relative-plus-tooltip presentation.
- Reuse `format_relative` / `format_absolute` directly; no new formatting
  logic.
- Omit the row when `date_added` is `None`, consistent with how the popover
  already omits other optional fields (e.g. the `line`/`files.len()` guards).

**Non-Goals:**
- Changing the detail panel's existing date-added behavior.
- Adding a distinct "updated" timestamp field to `LibraryItem` — the item
  model only tracks `date_added`; the TODO's "updated date" wording refers to
  the same field already used elsewhere as the added-date display.

## Decisions

- **Reuse `format_relative`/`format_absolute` as-is** rather than duplicating
  logic in the popover module. Alternative considered: inline formatting in
  `item_popover_view.rs`; rejected because it would drift from the detail
  panel's behavior over time.
- **Guard with `.when(item.date_added.is_some(), ...)`** to match the
  existing conditional-row pattern already used in this file (`line`,
  `files.len() > 1`), rather than introducing an `if`-based list mutation
  like the detail panel does. Keeps the popover's builder chain consistent
  with itself.
- **Row label**: reuse `t!("detail.field_added")`, the same i18n key the
  detail panel uses, since it is the same concept in both views.

## Risks / Trade-offs

- [Popover width is fixed (`ITEM_POPOVER_WIDTH`), and the date-added value
  is dynamic text] → Mitigation: the relative string (e.g. "3 days ago") is
  short and matches the visual weight of neighboring rows; the absolute
  date lives in a tooltip so it doesn't need to fit inline.
