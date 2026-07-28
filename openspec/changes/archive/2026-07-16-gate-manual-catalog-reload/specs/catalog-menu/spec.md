## MODIFIED Requirements

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
