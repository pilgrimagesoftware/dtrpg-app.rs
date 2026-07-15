# rust-main-window-library-layout Specification

## Purpose
TBD - created by archiving change define-rust-main-window-library-layout. Update Purpose after archive.
## Requirements
### Requirement: Rust main window MUST provide GPUI layout regions
The Rust desktop app MUST implement the shared main-window library layout using GPUI view modules and controller state. The layout MUST use `h_resizable` panels for the sidebar, catalog content, and detail panel columns. Panel widths MUST be draggable by the user within configured bounds and MUST persist across app launches.

#### Scenario: Rendering the Rust main library window
- **WHEN** the Rust app displays the library browsing window
- **THEN** it presents GPUI regions for search/filter controls, account menu access, library content, summary, and sync status
- **AND** the sidebar, catalog, and detail panel are separated by draggable resize handles

### Requirement: Rust search and filter controls MUST be disclosable
The Rust desktop app MUST provide a low-profile disclosable search/filter area with a functional text search input, view mode, grouping, and sort controls, plus a collapsed summary of active browsing state. The search input SHALL be an editable text field that filters the catalog on every keystroke. A clear button SHALL appear when the input is non-empty and SHALL reset the search and the input field.

#### Scenario: Toggling Rust filter disclosure
- **WHEN** the user expands or collapses the search/filter area
- **THEN** the Rust app preserves active search, filter, view mode, grouping, and sort state

#### Scenario: Typing in the search field filters the catalog
- **WHEN** the user types text into the search input
- **THEN** the catalog immediately narrows to items whose title or publisher matches the typed text

#### Scenario: Clearing the search field
- **WHEN** the user activates the clear button (✕) while a search query is active
- **THEN** the search input is emptied and the catalog returns to the unfiltered result set

#### Scenario: Empty search shows placeholder
- **WHEN** the search input is empty
- **THEN** the field displays its placeholder text ("Search…") and no clear button is shown

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

### Requirement: File openers action buttons are icon-only with tooltips
The "Add" button in the File Openers header and the "Remove" button on each entry row SHALL display only an icon with no visible text label. Each button SHALL expose its action label as a tooltip that appears on hover. The "Add" button SHALL use a "+" icon with tooltip "Add file opener". The remove button on each entry SHALL use a "×" icon with tooltip "Remove".

#### Scenario: Add button shows icon and tooltip
- **WHEN** the user views the File Openers settings section
- **THEN** the Add button shows a "+" icon with no visible text, and hovering the button shows the tooltip "Add file opener"

#### Scenario: Remove button shows icon and tooltip
- **WHEN** the user views a file opener entry row
- **THEN** the remove button shows a "×" icon with no visible text, and hovering the button shows the tooltip "Remove"

### Requirement: Catalog panel fills all available horizontal space between sidebar and detail
The catalog (multi-item) panel SHALL be a pure flex-fill panel with no independent initial size or size-range constraints of its own. It fills whatever horizontal space remains after the sidebar and detail panel are placed.

#### Scenario: Catalog fills space when detail is hidden
- **WHEN** no item is selected and the detail panel is hidden
- **THEN** the catalog panel spans from the sidebar's right edge to the right window edge with no gap

#### Scenario: Catalog fills remaining space when detail is visible
- **WHEN** an item is selected and the detail panel is visible
- **THEN** the catalog panel fills the space between the sidebar's right edge and the detail panel's left edge

### Requirement: Detail panel takes zero layout width when hidden
The detail (single-item) panel SHALL occupy zero horizontal space when no item is selected; it SHALL NOT leave a gap or placeholder column at the right of the window.

#### Scenario: No layout gap when detail is hidden
- **WHEN** no item is selected
- **THEN** the right edge of the catalog panel aligns with the right edge of the window; no blank column is visible

#### Scenario: Detail panel appears from the right edge when an item is selected
- **WHEN** an item is selected and the detail panel transitions from hidden to visible
- **THEN** the detail panel occupies its configured width from the right side; the catalog compresses accordingly

### Requirement: Catalog has an enforced minimum visible width when detail is present
When the detail panel is visible, the catalog panel SHALL enforce a minimum width of at least 280 px. The detail panel's left-edge handle SHALL not be dragable past the point that would compress the catalog below this minimum.

#### Scenario: Detail handle stops at catalog minimum
- **WHEN** the user drags the detail panel's left handle leftward
- **THEN** the handle stops when the catalog reaches its minimum width; the catalog's right edge remains visible

#### Scenario: Catalog never disappears behind detail panel
- **WHEN** the detail panel is at maximum width
- **THEN** at least 280 px of catalog content remains visible to the left of the detail panel

### Requirement: Resize handles appear only on the left edges of the sidebar and detail panels
The only resize handles in the main layout SHALL be: one on the right edge of the sidebar (= the left edge of the catalog panel, controlling sidebar width) and one on the left edge of the detail panel (controlling detail width). The catalog panel SHALL NOT have an independent handle on its right side.

#### Scenario: Sidebar handle controls sidebar width
- **WHEN** the user drags the handle between the sidebar and catalog
- **THEN** the sidebar width changes and the catalog adjusts to fill remaining space

#### Scenario: Detail handle controls detail width
- **WHEN** the user drags the handle on the detail panel's left edge
- **THEN** the detail panel width changes and the catalog adjusts to fill remaining space

