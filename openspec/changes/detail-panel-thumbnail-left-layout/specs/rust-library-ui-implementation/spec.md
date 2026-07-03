## MODIFIED Requirements

### Requirement: Detail tab layout places the thumbnail left of item information

The expanded detail tab SHALL render the cover thumbnail in a fixed-width left column and
the item's publisher, title, description, actions, and metadata in an independently
scrolling right column, at tab widths above a minimum threshold. Below that threshold the
layout SHALL fall back to a stacked (cover-above-info) arrangement.

#### Scenario: Normal tab width

- **WHEN** a detail tab is open at or above the minimum layout width
- **THEN** the cover renders in a fixed-width left column and item information renders in
  a scrollable right column

#### Scenario: Scrolling long content

- **WHEN** the user scrolls a long description or metadata list in the right column
- **THEN** the cover in the left column remains fixed in place

#### Scenario: Narrow tab width

- **WHEN** the detail tab's content width falls below the minimum layout threshold
- **THEN** the layout falls back to the cover stacked above the item information
