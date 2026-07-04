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
- **Guard with `.when_some(item.date_added, |list, ts| ...)`** rather than
  `.when(item.date_added.is_some(), ...)` plus an inner `unwrap`/`expect` —
  `when_some` hands the unwrapped timestamp straight to the closure, so the
  guard and the value stay in one place with no risk of a panicking unwrap
  (this crate denies `clippy::expect_used`). Still keeps the popover's
  builder-chain style consistent with the existing `line`/`files.len() > 1`
  conditional rows.
- **Row label**: reuse `t!("detail.field_added")`, the same i18n key the
  detail panel uses, since it is the same concept in both views.

## Risks / Trade-offs

- [Popover width is fixed (`ITEM_POPOVER_WIDTH`), and the date-added value
  is dynamic text] → Mitigation: the relative string (e.g. "3 days ago") is
  short and matches the visual weight of neighboring rows; the absolute
  date lives in a tooltip so it doesn't need to fit inline.
