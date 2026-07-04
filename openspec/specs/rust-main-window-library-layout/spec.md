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
The Rust desktop app MUST use one controller-facing browsing state for list, tree, and grid presentations so mode changes preserve the current filtered and sorted result set.

#### Scenario: Switching between Rust list, tree, and grid views
- **WHEN** the user switches library presentation mode
- **THEN** the same matched items, grouping, and sort order are represented in the selected GPUI presentation

### Requirement: Rust account menu MUST expose account actions safely
The Rust desktop app MUST provide an account button menu or equivalent compact popover that reflects the current authentication state. When authenticated, it displays DriveThruRPG account identity, settings navigation, and a sign-out action. When unauthenticated, it displays a "Not signed in" indicator and a "Sign In" action that opens the Settings Account tab — without passively showing raw access-token values in either state.

#### Scenario: Opening the Rust account menu when authenticated
- **WHEN** the user opens the account menu while signed in
- **THEN** the menu exposes account identity (email or initial), settings access, and a sign-out action without raw token disclosure

#### Scenario: Opening the Rust account menu when unauthenticated
- **WHEN** the user opens the account menu while not signed in
- **THEN** the menu shows a "Not signed in" label and a "Sign In" item that opens Settings to the Account tab

### Requirement: Rust sync and thumbnail loading MUST be non-blocking
The Rust desktop app MUST keep background library sync and thumbnail loading from blocking main-window interaction.

#### Scenario: Syncing or loading thumbnails in Rust
- **WHEN** the Rust app syncs library metadata or resolves grid thumbnails
- **THEN** the user can continue interacting with library controls and visible title/size metadata

### Requirement: Catalog pane fills remaining horizontal space without a resize handle
The catalog pane SHALL occupy all horizontal space between the sidebar and the right window edge. There SHALL be no resize splitter between the catalog and the detail panel. The detail panel SHALL be an overlay or fixed-width panel that does not reduce catalog width.

#### Scenario: Catalog fills window width when detail panel is hidden
- **WHEN** no item is selected and the detail panel is not shown
- **THEN** the catalog occupies the full width from the sidebar right edge to the window right edge with no visible resize handle

#### Scenario: Detail panel appears without shrinking the catalog
- **WHEN** an item is selected and the detail panel opens
- **THEN** the detail panel is displayed at a fixed width overlapping or adjacent to the catalog without pushing catalog content to resize

