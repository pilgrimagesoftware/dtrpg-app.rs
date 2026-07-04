## ADDED Requirements

### Requirement: Tooltip on truncated catalog title
The catalog view SHALL show a tooltip containing the full, untruncated item title when the rendered title text in
the grid card, flat list row, grouped list row, or thumbnail row is visually truncated (ellipsized) because it
does not fit within its available width.

#### Scenario: Truncated title in grid card
- **WHEN** a catalog item's title is wider than the grid card's title text slot and is rendered with an ellipsis
- **THEN** hovering over the title shows a tooltip containing the item's full title

#### Scenario: Truncated title in flat list row
- **WHEN** a catalog item's title is wider than the title column in the flat list layout and is rendered with an
  ellipsis
- **THEN** hovering over the title shows a tooltip containing the item's full title

#### Scenario: Truncated title in grouped list row
- **WHEN** a catalog item's title is wider than the title column in the grouped list layout and is rendered with
  an ellipsis
- **THEN** hovering over the title shows a tooltip containing the item's full title

#### Scenario: Truncated title in thumbnail row
- **WHEN** a catalog item's title is wider than the title text slot in the thumbnail layout and is rendered with
  an ellipsis
- **THEN** hovering over the title shows a tooltip containing the item's full title

### Requirement: No tooltip on untruncated catalog title
The catalog view SHALL NOT show a tooltip on a title's text when the full title fits within its available width
without truncation.

#### Scenario: Short title fits fully
- **WHEN** a catalog item's title is narrower than its available title width in any catalog layout (grid, flat
  list, grouped list, thumbnail)
- **THEN** hovering over the title does not show a tooltip
