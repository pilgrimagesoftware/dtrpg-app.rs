## ADDED Requirements

### Requirement: Catalog rows show a rich HoverCard on hover

The system SHALL display a `gpui-component` `HoverCard` containing the item's title,
publisher, and status when the user hovers over a catalog row, replacing the plain
single-line tooltip previously shown for truncated titles.

#### Scenario: Hover over a catalog row with a truncated title

- **WHEN** the user hovers over a catalog row whose title is truncated
- **THEN** a `HoverCard` appears showing the full title, publisher, and status

#### Scenario: Mouse leaves the row

- **WHEN** the user moves the mouse away from a catalog row showing a `HoverCard`
- **THEN** the card dismisses
