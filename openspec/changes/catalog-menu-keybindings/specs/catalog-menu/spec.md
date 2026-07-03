## MODIFIED Requirements

### Requirement: Catalog menu contains Add Collection action
The "Catalog" menu SHALL contain an "Add Collection" menu item that opens the new-collection input flow, bound to the keyboard shortcut `cmd-shift-n`.

#### Scenario: Add Collection opens input
- **WHEN** the user selects "Catalog > Add Collection"
- **THEN** the sidebar displays the new-collection name input field, identical to clicking the add button in the sidebar

#### Scenario: Add Collection keyboard shortcut
- **WHEN** the user presses `cmd-shift-n` anywhere in the library window
- **THEN** the sidebar displays the new-collection name input field, identical to selecting "Catalog > Add Collection" from the menu

### Requirement: Catalog menu contains Reload action
The "Catalog" menu SHALL contain a "Reload" menu item that triggers a full live catalog fetch from the API, bound to the keyboard shortcut `cmd-r`.

#### Scenario: Reload triggers a live fetch
- **WHEN** the user selects "Catalog > Reload"
- **THEN** the catalog loading indicator appears and the app fetches all catalog pages from the API, replacing the current catalog when complete

#### Scenario: Reload is available regardless of current catalog state
- **WHEN** the catalog is loaded and the user selects "Catalog > Reload"
- **THEN** the fetch runs and the catalog is refreshed without requiring a restart

#### Scenario: Reload keyboard shortcut
- **WHEN** the user presses `cmd-r` anywhere in the library window
- **THEN** the catalog loading indicator appears and the app fetches all catalog pages from the API, identical to selecting "Catalog > Reload" from the menu

## ADDED Requirements

### Requirement: Catalog menu contains Refresh Thumbnails action
The "Catalog" menu SHALL contain a "Refresh Thumbnails" menu item that re-fetches cover thumbnails for every catalog item with a cover URL, bypassing both the in-memory and on-disk thumbnail caches, bound to the keyboard shortcut `cmd-shift-r`.

#### Scenario: Refresh Thumbnails re-fetches every cover
- **WHEN** the user selects "Catalog > Refresh Thumbnails"
- **THEN** every catalog item with a cover URL is re-queued for a forced network fetch, overwriting any cached image in memory and on disk

#### Scenario: Refresh Thumbnails keyboard shortcut
- **WHEN** the user presses `cmd-shift-r` anywhere in the library window
- **THEN** every catalog item with a cover URL is re-queued for a forced network fetch, identical to selecting "Catalog > Refresh Thumbnails" from the menu
