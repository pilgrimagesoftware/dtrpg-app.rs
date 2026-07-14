## MODIFIED Requirements

### Requirement: Rust main window MUST provide GPUI layout regions

The Rust desktop app MUST implement the shared main-window library layout using GPUI view modules and controller state. The layout MUST use `h_resizable` panels for the sidebar, catalog content, and detail panel columns. Panel widths MUST be draggable by the user within configured bounds and MUST persist across app launches.

#### Scenario: Rendering the Rust main library window
- **WHEN** the Rust app displays the library browsing window
- **THEN** it presents GPUI regions for search/filter controls, account menu access, library content, summary, and sync status
- **AND** the sidebar, catalog, and detail panel are separated by draggable resize handles
