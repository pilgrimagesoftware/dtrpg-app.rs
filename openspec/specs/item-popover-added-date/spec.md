# item-popover-added-date Specification

## Purpose
TBD - created by archiving change item-popover-added-date. Update Purpose after archive.
## Requirements

### Requirement: Item popover shows date added
The item popover SHALL display a date-added row in its description list
whenever the item's `date_added` is present. The row SHALL show a
human-readable relative date (e.g. "3 days ago") and SHALL expose the
absolute date as a tooltip on hover. When `date_added` is `None`, the
popover SHALL omit the row entirely rather than showing a placeholder.

#### Scenario: Item has a known date added
- **WHEN** the popover is rendered for an item with `date_added` set
- **THEN** the description list includes a date-added row showing a
  relative, human-readable date, and hovering the row reveals the absolute
  date in a tooltip

#### Scenario: Item has no known date added
- **WHEN** the popover is rendered for an item with `date_added` equal to
  `None`
- **THEN** the description list does not include a date-added row
