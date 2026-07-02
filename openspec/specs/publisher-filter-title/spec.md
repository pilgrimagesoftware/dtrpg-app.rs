# publisher-filter-title Specification

## Purpose
TBD - created by archiving change publisher-filter-title. Update Purpose after archive.
## Requirements
### Requirement: Toolbar title includes publisher name when publisher filter is active
When a publisher filter is active, the toolbar section title SHALL be `"Publisher: <name>"` where `<name>` is the selected publisher's name exactly as stored in the catalog.

#### Scenario: Publisher filter active
- **WHEN** `SidebarFilter::Publisher("Kobold Press")` is the active filter
- **THEN** the toolbar title SHALL display `"Publisher: Kobold Press"`

#### Scenario: Non-publisher filters unaffected
- **WHEN** any filter other than `Publisher` is active (AllTitles, RecentlyAdded, OnDevice, InCloud)
- **THEN** the toolbar title SHALL display the existing static label for that filter unchanged

