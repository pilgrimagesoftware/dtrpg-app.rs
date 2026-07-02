# sidebar-section-counts Specification

## Purpose
TBD - created by archiving change catalog-collections-improvements. Update Purpose after archive.
## Requirements
### Requirement: Collections section header shows collection count
The "Collections" sidebar section header SHALL display the number of collections next to the section title.

#### Scenario: Collection count rendered in header
- **WHEN** the sidebar renders with one or more collections
- **THEN** the "Collections" section header shows a count badge with the number of collections

#### Scenario: Zero count still visible
- **WHEN** the user has no collections
- **THEN** the "Collections" section header shows a count badge with "0"

### Requirement: Publishers section header shows publisher count
The "Publishers" sidebar section header SHALL display the number of distinct publishers next to the section title.

#### Scenario: Publisher count rendered in header
- **WHEN** the catalog contains items from one or more publishers
- **THEN** the "Publishers" section header shows a count badge with the number of distinct publishers

#### Scenario: Publisher count updates after catalog reload
- **WHEN** the catalog is reloaded and the publisher count changes
- **THEN** the "Publishers" section header count updates to match the new set

