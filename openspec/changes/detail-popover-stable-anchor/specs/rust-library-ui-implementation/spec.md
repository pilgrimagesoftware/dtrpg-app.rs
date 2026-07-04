## MODIFIED Requirements

### Requirement: Item popover anchors to the position at which it was opened

The single-click item popover SHALL anchor to a position captured at the moment the item
was selected, and SHALL NOT move in response to subsequent mouse movement while it remains
open.

#### Scenario: Popover stays put while the mouse moves

- **WHEN** the user single-clicks a catalog item to open its popover and then moves the
  mouse elsewhere within the catalog area
- **THEN** the popover remains anchored at its original opening position

#### Scenario: Selecting a different item repositions the popover

- **WHEN** the popover is open for one item and the user clicks a different item
- **THEN** the popover closes and reopens anchored to the new item's click position

### Requirement: Item popover anchors beside the catalog entry, not over it

The single-click item popover SHALL anchor to the right of the catalog entry that opened
it, and SHALL anchor to the left of the entry instead when there is not enough room to its
right within the window. In either case the popover SHALL NOT cover the entry.

#### Scenario: Popover opens to the right of the entry

- **WHEN** the user single-clicks a catalog entry with enough room to its right for the
  popover to fit within the window
- **THEN** the popover opens immediately to the right of the entry, top-aligned with it,
  without covering it

#### Scenario: Popover falls back to the left of the entry

- **WHEN** the user single-clicks a catalog entry too close to the right edge of the
  window for the popover to fit on the right
- **THEN** the popover opens immediately to the left of the entry instead, without
  covering it
