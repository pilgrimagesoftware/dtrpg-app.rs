# catalog-scroll Specification

## Purpose
TBD - created by archiving change fix-catalog-scroll. Update Purpose after archive.
## Requirements
### Requirement: Catalog content area scrolls vertically
When the catalog contains more items than fit in the visible area, the catalog content region SHALL scroll vertically. Items below the fold SHALL be reachable by scrolling.

#### Scenario: List layout scrolls
- **WHEN** the catalog is in list layout and the item count exceeds the visible rows
- **THEN** the user can scroll the catalog to reveal items below the visible area

#### Scenario: Thumbs layout scrolls
- **WHEN** the catalog is in thumbs layout and the item count exceeds the visible rows
- **THEN** the user can scroll the catalog to reveal items below the visible area

#### Scenario: Grid layout scrolls
- **WHEN** the catalog is in grid layout and the item count exceeds the visible rows
- **THEN** the user can scroll the catalog to reveal items below the visible area

### Requirement: Sidebar publisher list scrolls vertically
When the publisher list is longer than the sidebar's available height, the sidebar body SHALL scroll vertically so all publishers are reachable.

#### Scenario: Long publisher list scrolls
- **WHEN** the publisher list exceeds the sidebar height
- **THEN** the user can scroll the sidebar body to see all publishers
