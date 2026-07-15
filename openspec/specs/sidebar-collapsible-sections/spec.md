# sidebar-collapsible-sections Specification

## Purpose
TBD - created by archiving change sidebar-collections-and-collapsible. Update Purpose after archive.
## Requirements
### Requirement: Publishers section is collapsible
The Publishers section header SHALL be a clickable toggle. When expanded (default), all publisher rows are visible. When collapsed, the publisher rows are hidden and only the header is shown.

#### Scenario: Publishers section starts expanded
- **WHEN** the library window opens
- **THEN** the Publishers section header is visible and all publisher rows are shown

#### Scenario: Clicking header collapses publishers
- **WHEN** the user clicks the Publishers section header
- **THEN** the publisher rows are hidden and the header shows a collapsed indicator

#### Scenario: Clicking collapsed header expands publishers
- **WHEN** the Publishers section is collapsed and the user clicks its header
- **THEN** the publisher rows are shown again

### Requirement: Collections section is collapsible
The Collections section header SHALL be a clickable toggle with the same expand/collapse behavior as the Publishers section.

#### Scenario: Collections section starts expanded
- **WHEN** the Collections section first appears
- **THEN** its rows are visible and the header shows an expanded indicator

#### Scenario: Clicking header collapses collections
- **WHEN** the user clicks the Collections section header
- **THEN** the collection rows are hidden and the header shows a collapsed indicator

#### Scenario: Clicking collapsed header expands collections
- **WHEN** the Collections section is collapsed and the user clicks its header
- **THEN** the collection rows are shown again

### Requirement: Section collapse state is independent
Collapsing one section SHALL NOT affect the other section's state.

#### Scenario: Collapsing publishers does not collapse collections
- **WHEN** the user collapses the Publishers section
- **THEN** the Collections section remains in its current state (expanded or collapsed)

