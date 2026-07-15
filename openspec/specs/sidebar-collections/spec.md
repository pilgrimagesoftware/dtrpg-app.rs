# sidebar-collections Specification

## Purpose
TBD - created by archiving change sidebar-collections-and-collapsible. Update Purpose after archive.
## Requirements
### Requirement: Collections section displayed in sidebar
The sidebar SHALL display a Collections section below the Publishers section. The section SHALL list the user's DTRPG product lists by name, each showing a count of how many catalog items belong to that collection.

#### Scenario: Collections section appears after load
- **WHEN** the library finishes loading and the user has at least one DTRPG product list
- **THEN** a Collections section is visible in the sidebar with one row per collection

#### Scenario: Empty collections list hides section
- **WHEN** the user has no DTRPG product lists
- **THEN** the Collections section is not rendered

#### Scenario: Count reflects catalog intersection
- **WHEN** a collection row is displayed
- **THEN** the item count shown is the number of items in the user's catalog that belong to that collection, not the total API item_count

### Requirement: Catalog filtered by selected collection
When the user selects a collection in the sidebar, the catalog SHALL show only items whose product ID is in that collection's membership set.

#### Scenario: Clicking a collection filters the catalog
- **WHEN** the user clicks a collection row in the sidebar
- **THEN** the catalog updates to show only items in that collection

#### Scenario: Items not in library are excluded
- **WHEN** a collection contains products the user does not own
- **THEN** those products do not appear in the catalog view

#### Scenario: Selecting active collection row is a no-op
- **WHEN** the user clicks the already-selected collection row
- **THEN** the filter remains unchanged

### Requirement: Collections loaded after catalog
The app SHALL fetch collection names and membership after the catalog finishes loading, in a background task.

#### Scenario: Collections appear after catalog
- **WHEN** the catalog has finished loading
- **THEN** a background task fetches product lists and their members and populates the Collections section

#### Scenario: Collections load failure is non-fatal
- **WHEN** the collections fetch fails (network error or API error)
- **THEN** the Collections section remains hidden, the catalog is unaffected, and the error is logged

