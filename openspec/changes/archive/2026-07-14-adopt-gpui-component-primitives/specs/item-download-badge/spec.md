## ADDED Requirements

### Requirement: Thumbs and grid cards show download status badge

Each catalog item card in thumbs and grid views SHALL display a `Badge` overlay that visually indicates whether the item is downloaded to the device or cloud-only.

#### Scenario: Downloaded item shows filled badge
- **WHEN** an item has `ItemStatus::Downloaded`
- **THEN** its thumbs/grid card shows a green dot `Badge` (or equivalent accent indicator) in the corner

#### Scenario: Cloud item shows no badge or a muted indicator
- **WHEN** an item has `ItemStatus::Cloud`
- **THEN** its thumbs/grid card either shows no badge or a muted dot so downloaded items are visually distinct
