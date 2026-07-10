## MODIFIED Requirements

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
