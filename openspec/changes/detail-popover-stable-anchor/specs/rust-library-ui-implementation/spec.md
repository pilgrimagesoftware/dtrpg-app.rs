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
