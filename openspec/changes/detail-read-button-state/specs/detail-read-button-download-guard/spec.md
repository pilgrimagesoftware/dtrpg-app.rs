## ADDED Requirements

### Requirement: Read button is disabled when item is not downloaded
The detail panel Read button SHALL be visually disabled and non-interactive when the item's status is not `Downloaded`. The button SHALL remain visible in the layout at all times.

#### Scenario: Read button is enabled when item is downloaded
- **WHEN** the detail panel displays an item with `status == Downloaded`
- **THEN** the Read button is fully interactive with accent background and no tooltip

#### Scenario: Read button is disabled when item is not downloaded
- **WHEN** the detail panel displays an item with `status != Downloaded`
- **THEN** the Read button appears at reduced opacity, has no click handler, and shows no cursor pointer

### Requirement: Disabled Read button shows a download-prerequisite tooltip
The disabled Read button SHALL display a tooltip explaining that the item must be downloaded before it can be read.

#### Scenario: Tooltip appears on hover over disabled Read button
- **WHEN** the user hovers over the Read button and the item is not downloaded
- **THEN** a tooltip appears with the text "Download this item first"
