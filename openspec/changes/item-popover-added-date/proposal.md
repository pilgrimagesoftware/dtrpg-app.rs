## Why

The item popover leaves a `TODO: add updated date HUMAN READABLE` in place of the
date-added field. The expanded detail tab already shows this field as a relative,
human-readable string with an absolute-date tooltip; the popover shows nothing,
so users lose that information in the lighter-weight view.

## What Changes

- Add a date-added row to the item popover's `DescriptionList`, shown only when
  `item.date_added` is present.
- Reuse the existing relative/absolute date formatting and tooltip pattern from
  the detail panel (`format_relative`, `format_absolute`) rather than
  introducing a second formatting scheme.
- Remove the `TODO` comment once the field is implemented.

## Capabilities

### New Capabilities
- `item-popover-added-date`: The item popover shows a human-readable relative
  date-added row (with an absolute-date tooltip) whenever the item has a known
  `date_added` value.

### Modified Capabilities
(none — the detail panel's existing date-added behavior is unchanged; the
popover gains an equivalent, independent presentation.)

## Impact

- `dtrpg-ui/src/ui/views/item_popover_view.rs`: adds a conditional
  `DescriptionItem` for date-added, using shared formatting helpers from
  `dtrpg-ui/src/util/datetime.rs`.
- No API, storage, or cross-crate changes.
