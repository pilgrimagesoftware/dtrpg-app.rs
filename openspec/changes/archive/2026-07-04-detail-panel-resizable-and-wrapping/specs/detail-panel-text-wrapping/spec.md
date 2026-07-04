## ADDED Requirements

### Requirement: Detail panel body is scrollable
The detail panel body (the region below the cover image containing the title, description, actions, and metadata table) SHALL scroll vertically when its content height exceeds the available viewport height. Content SHALL NOT be clipped.

#### Scenario: Long description is scrollable
- **WHEN** an item's description text is long enough to exceed the visible panel body height
- **THEN** the user can scroll the panel body to read the full description

#### Scenario: All metadata rows are reachable
- **WHEN** the panel body content exceeds the viewport height
- **THEN** the metadata table at the bottom of the panel body is reachable by scrolling

### Requirement: Title and description text wraps within the panel width
The item title and description in the detail panel SHALL wrap to multiple lines at the current panel width rather than overflowing or being truncated.

#### Scenario: Long title wraps to multiple lines
- **WHEN** an item's title is longer than the panel width
- **THEN** the title text wraps to a second (or further) line rather than overflowing the panel boundary

#### Scenario: Long description wraps within the panel
- **WHEN** an item's description contains long lines or a continuous string wider than the panel
- **THEN** the description text wraps within the panel width

### Requirement: Metadata table values wrap within their column
Metadata table value cells SHALL wrap their text when the value string is wider than the available column space.

#### Scenario: Long metadata value wraps
- **WHEN** a metadata field value (e.g., publisher name or system) is longer than the right column of the metadata table
- **THEN** the value text wraps to additional lines within the right column rather than overflowing the row or the panel
