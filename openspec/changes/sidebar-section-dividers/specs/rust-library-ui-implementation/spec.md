## ADDED Requirements

### Requirement: Sidebar renders dividers between section groups
The sidebar SHALL display a thin horizontal divider between the smart-filter group and the Publishers group, and a second divider between the Publishers group and the Collections group.

#### Scenario: Divider appears between smart filters and publishers
- **WHEN** the sidebar is rendered and both the smart-filter items and the Publishers section are visible
- **THEN** a thin horizontal rule is visible between the last smart-filter item and the Publishers section header

#### Scenario: Divider appears between publishers and collections
- **WHEN** the sidebar is rendered and both the Publishers section and the Collections section are visible
- **THEN** a thin horizontal rule is visible between the Publishers section and the Collections section header
