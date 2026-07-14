## MODIFIED Requirements

### Requirement: Detail tab layout places the thumbnail left of item information

The expanded detail tab SHALL render the cover thumbnail in a fixed-width left column,
inset from the tab's left edge, and the item's publisher, title, description, actions,
and metadata in an independently scrolling right column.

#### Scenario: Detail tab layout

- **WHEN** a detail tab is open
- **THEN** the cover renders in a fixed-width left column with left padding separating it
  from the tab's edge, and item information renders in a scrollable right column

#### Scenario: Scrolling long content

- **WHEN** the user scrolls a long description or metadata list in the right column
- **THEN** the cover in the left column remains fixed in place
