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
The "Catalog" menu SHALL contain an "Add Collection" menu item that opens the new-collection input flow, bound to the keyboard shortcut `cmd-shift-n`.

#### Scenario: Add Collection opens input
- **WHEN** the user selects "Catalog > Add Collection"
- **THEN** the sidebar displays the new-collection name input field, identical to clicking the add button in the sidebar

#### Scenario: Add Collection keyboard shortcut
- **WHEN** the user presses `cmd-shift-n` anywhere in the library window
- **THEN** the sidebar displays the new-collection name input field, identical to selecting "Catalog > Add Collection" from the menu

### Requirement: Catalog menu contains Reload action
The "Catalog" menu SHALL contain a "Reload" menu item, bound to the keyboard shortcut `cmd-r`, that invokes the same catalog auto-load freshness policy (`catalog-auto-load-policy`) used by passive and timer-triggered loads. A full live paginated fetch runs only when that policy determines one is needed; otherwise the action completes as a no-op against the existing catalog.

#### Scenario: Reload triggers a live fetch when the cache is stale
- **WHEN** the user selects "Catalog > Reload" and the catalog cache is stale, empty, or its remote item count no longer matches
- **THEN** the catalog loading indicator appears and the app fetches all catalog pages from the API, replacing the current catalog when complete

#### Scenario: Reload is a no-op when the cache is already fresh
- **WHEN** the user selects "Catalog > Reload" and the catalog cache is non-empty, was saved within the freshness window, and the remote item count matches
- **THEN** the app performs the lightweight remote count check, then completes without running a full paginated fetch or changing the catalog

#### Scenario: Reload is available regardless of current catalog state
- **WHEN** the catalog is loaded and the user selects "Catalog > Reload"
- **THEN** the freshness policy is invoked and the catalog is refreshed if the policy determines one is needed, without requiring a restart

#### Scenario: Reload keyboard shortcut
- **WHEN** the user presses `cmd-r` anywhere in the library window
- **THEN** the same freshness-policy-gated behavior runs, identical to selecting "Catalog > Reload" from the menu

### Requirement: Catalog menu contains Refresh Thumbnails action
The "Catalog" menu SHALL contain a "Refresh Thumbnails" menu item that re-fetches cover thumbnails for every catalog item with a cover URL, bypassing both the in-memory and on-disk thumbnail caches, bound to the keyboard shortcut `cmd-shift-r`.

#### Scenario: Refresh Thumbnails re-fetches every cover
- **WHEN** the user selects "Catalog > Refresh Thumbnails"
- **THEN** every catalog item with a cover URL is re-queued for a forced network fetch, overwriting any cached image in memory and on disk

#### Scenario: Refresh Thumbnails keyboard shortcut
- **WHEN** the user presses `cmd-shift-r` anywhere in the library window
- **THEN** every catalog item with a cover URL is re-queued for a forced network fetch, identical to selecting "Catalog > Refresh Thumbnails" from the menu

