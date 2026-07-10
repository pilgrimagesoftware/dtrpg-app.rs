# rust-main-window-library-layout Specification

## Purpose
TBD - created by archiving change define-rust-main-window-library-layout. Update Purpose after archive.
## Requirements
### Requirement: Rust main window MUST provide GPUI layout regions
The Rust desktop app MUST implement the shared main-window library layout using GPUI view modules and controller state.

#### Scenario: Rendering the Rust main library window
- **WHEN** the Rust app displays the library browsing window
- **THEN** it presents GPUI regions for search/filter controls, account menu access, library content, summary, and sync status

### Requirement: Rust search and filter controls MUST be disclosable
The Rust desktop app MUST provide a low-profile disclosable search/filter area with search input, view mode, grouping, and sort controls, plus a collapsed summary of active browsing state.

#### Scenario: Toggling Rust filter disclosure
- **WHEN** the user expands or collapses the search/filter area
- **THEN** the Rust app preserves active search, filter, view mode, grouping, and sort state

### Requirement: Rust library presentations MUST share browsing state
The Rust desktop app MUST use one controller-facing browsing state for list, tree, and grid presentations so mode changes preserve the current filtered and sorted result set. The layout switcher SHALL present the three presentations (List, Thumbs, Grid) as icon-only tabs, each carrying a tooltip with the localized mode name instead of a visible text label.

#### Scenario: Switching between Rust list, tree, and grid views
- **WHEN** the user switches library presentation mode
- **THEN** the same matched items, grouping, and sort order are represented in the selected GPUI presentation

#### Scenario: Layout switcher shows icons instead of text
- **WHEN** the user views the toolbar's layout switcher
- **THEN** each of the three tabs displays an icon (list, thumbnails, grid) with no visible text label

#### Scenario: Layout switcher tooltip names the mode
- **WHEN** the user hovers a layout switcher tab
- **THEN** a tooltip shows the localized mode name ("List", "Thumbs", or "Grid")

### Requirement: Rust account menu MUST expose account actions safely
The Rust desktop app MUST provide an account button menu or equivalent compact popover that displays DriveThruRPG account identity or connection status, token set/reset actions, and settings navigation without passively showing raw access-token values. When a user is signed in, the account menu SHALL display the account email address as a non-interactive label at the top of the menu, followed by a separator, followed by account action items. The email address SHALL NOT reveal any credential value (API key, access token, or refresh token).

#### Scenario: Opening the Rust account menu
- **WHEN** the user opens the account menu
- **THEN** the menu exposes account status, token management actions, and settings access without raw token disclosure

#### Scenario: Account menu shows email identity
- **WHEN** a user is signed in and opens the avatar button menu
- **THEN** the menu displays the account email address as a non-interactive header above a visual separator, followed by the "Log Out" action

#### Scenario: Account menu without signed-in user
- **WHEN** no user is signed in and the unauthenticated avatar button is present
- **THEN** no dropdown menu with identity information is shown

### Requirement: Rust sync and thumbnail loading MUST be non-blocking
The Rust desktop app MUST keep background library sync and thumbnail loading from blocking main-window interaction. The number of concurrent background fetch operations (thumbnail loads and file downloads combined) MUST be bounded by a user-configurable limit, defaulting to 3, so that aggressive fetching does not degrade UI responsiveness.

#### Scenario: Syncing or loading thumbnails in Rust
- **WHEN** the Rust app syncs library metadata or resolves grid thumbnails
- **THEN** the user can continue interacting with library controls and visible title/size metadata

#### Scenario: Concurrency limit enforced during heavy load
- **WHEN** more thumbnail fetches or downloads are pending than the configured limit allows
- **THEN** the excess requests wait in their respective queues and the main window remains responsive

### Requirement: Catalog pane fills remaining horizontal space without a resize handle
The catalog pane SHALL occupy all horizontal space between the sidebar and the right window edge. There SHALL be no resize splitter between the catalog and the detail panel. The detail panel SHALL be an overlay or fixed-width panel that does not reduce catalog width.

#### Scenario: Catalog fills window width when detail panel is hidden
- **WHEN** no item is selected and the detail panel is not shown
- **THEN** the catalog occupies the full width from the sidebar right edge to the window right edge with no visible resize handle

#### Scenario: Detail panel appears without shrinking the catalog
- **WHEN** an item is selected and the detail panel opens
- **THEN** the detail panel is displayed at a fixed width overlapping or adjacent to the catalog without pushing catalog content to resize

