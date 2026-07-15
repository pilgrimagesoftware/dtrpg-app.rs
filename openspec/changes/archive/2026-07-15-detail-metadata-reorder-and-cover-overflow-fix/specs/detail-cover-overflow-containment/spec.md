## MODIFIED Requirements

### Requirement: Rust main window MUST provide GPUI layout regions

The Rust desktop app MUST implement the shared main-window library layout using GPUI view
modules and controller state. The detail tab's fixed-width cover column MUST NOT overflow
past the tab content area into the sidebar at narrow tab widths.

#### Scenario: Rendering the Rust main library window

- **WHEN** the Rust app displays the library browsing window
- **THEN** it presents GPUI regions for search/filter controls, account menu access,
  library content, summary, and sync status

#### Scenario: Narrow detail tab width

- **WHEN** the detail tab's content area is narrower than the cover column's fixed width
- **THEN** the cover column clips or scrolls within its own container instead of painting
  over the sidebar
