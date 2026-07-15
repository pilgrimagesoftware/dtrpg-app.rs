## MODIFIED Requirements

### Requirement: Rust library presentations MUST share browsing state
The Rust desktop app MUST use one controller-facing browsing state for list, tree, and grid presentations so mode changes preserve the current filtered and sorted result set. In the list presentation, each data row SHALL use column widths that match the corresponding header column widths exactly, so that header labels and data values are visually aligned.

#### Scenario: Switching between Rust list, tree, and grid views
- **WHEN** the user switches library presentation mode
- **THEN** the same matched items, grouping, and sort order are represented in the selected GPUI presentation

#### Scenario: List view columns align header to data rows
- **WHEN** the user views the catalog in list presentation
- **THEN** the header label for each column (Title, Publisher, System, Pages, Size, Added, status, reveal) is horizontally aligned with the corresponding data value in every row
