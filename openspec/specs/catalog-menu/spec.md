# catalog-menu Specification

## Purpose
TBD - created by archiving change catalog-collections-improvements. Update Purpose after archive.
## Requirements
### Requirement: Catalog menu exists in the menu bar
The application menu bar SHALL include a "Catalog" menu positioned between the "Edit" and "View" menus.

#### Scenario: Catalog menu is visible
- **WHEN** the application launches
- **THEN** a "Catalog" menu item appears in the menu bar between "Edit" and "View"

### Requirement: Catalog menu contains Add Collection action
The "Catalog" menu SHALL contain an "Add Collection" menu item that opens the new-collection input flow.

#### Scenario: Add Collection opens input
- **WHEN** the user selects "Catalog > Add Collection"
- **THEN** the sidebar displays the new-collection name input field, identical to clicking the add button in the sidebar

### Requirement: Catalog menu contains Reload action
The "Catalog" menu SHALL contain a "Reload" menu item that triggers a full live catalog fetch from the API.

#### Scenario: Reload triggers a live fetch
- **WHEN** the user selects "Catalog > Reload"
- **THEN** the catalog loading indicator appears and the app fetches all catalog pages from the API, replacing the current catalog when complete

#### Scenario: Reload is available regardless of current catalog state
- **WHEN** the catalog is loaded and the user selects "Catalog > Reload"
- **THEN** the fetch runs and the catalog is refreshed without requiring a restart

