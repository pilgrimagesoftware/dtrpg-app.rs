## ADDED Requirements

### Requirement: Toolbar MUST display the current section title and matched item count
The toolbar MUST render a title area showing the name of the active library section and a count badge showing how many items are currently visible after filtering.

#### Scenario: Title reflects active sidebar section
- **WHEN** the user selects a sidebar section or publisher
- **THEN** the toolbar title changes to the name of that section or publisher

#### Scenario: Count badge reflects filtered result set
- **WHEN** a search query or filter is active
- **THEN** the count badge shows the number of matched items, not the total library count

### Requirement: Toolbar MUST provide a search input with a clear action
The toolbar MUST render a search input field. When the input contains text, the toolbar MUST show a clear button that removes the query. The search input MUST match against item title, publisher, and game line.

#### Scenario: Typing in the search input filters the catalog
- **WHEN** the user types in the search input
- **THEN** the catalog updates to show only items whose title, publisher, or game line contains the query (case-insensitive)

#### Scenario: Clear button removes the active query
- **WHEN** the search input contains text and the user activates the clear button
- **THEN** the search query is cleared and the catalog returns to the unfiltered result set

#### Scenario: Clear button is hidden when search is empty
- **WHEN** the search input is empty
- **THEN** no clear button is rendered in the toolbar

### Requirement: Toolbar MUST provide a sort dropdown
The toolbar MUST render a sort control that allows the user to choose from: Title (A–Z), Publisher, Date Added, and Page Count. The selected sort method MUST persist while the user switches layout modes.

#### Scenario: Selecting a sort method reorders the catalog
- **WHEN** the user selects a sort option from the sort dropdown
- **THEN** the catalog items reorder according to the selected method without changing the active filter or layout

#### Scenario: Sort state is preserved across layout switches
- **WHEN** the user changes the catalog layout while a non-default sort is active
- **THEN** the sort method remains unchanged

### Requirement: Toolbar MUST provide a group-by-publisher toggle
The toolbar MUST render a toggle control labeled "Group by publisher". When active, the catalog groups items into publisher sections. When inactive, items are shown as a flat list (or ungrouped grid).

#### Scenario: Enabling grouping adds publisher section headers
- **WHEN** the user enables the group-by-publisher toggle
- **THEN** the catalog renders publisher section headers and groups items under their respective publishers

#### Scenario: Disabling grouping removes section headers
- **WHEN** the user disables the group-by-publisher toggle
- **THEN** the catalog renders items without publisher section headers in a flat arrangement

### Requirement: Toolbar MUST provide a segmented layout switcher
The toolbar MUST render a segmented control with three options: List (text rows), Thumbs (thumbnail rows), and Grid (cover cards). The active layout MUST be visually indicated. Switching layout MUST NOT change the active filter, sort, or grouping state.

#### Scenario: Selecting List renders text-only rows
- **WHEN** the user selects the List layout
- **THEN** the catalog renders items as text rows with title, publisher, system, pages, size, date added, and status columns

#### Scenario: Selecting Thumbs renders rows with cover thumbnails
- **WHEN** the user selects the Thumbs layout
- **THEN** the catalog renders items as rows with a cover thumbnail alongside title, publisher, system line, kind tag, dimensions, date, and status

#### Scenario: Selecting Grid renders cover cards
- **WHEN** the user selects the Grid layout
- **THEN** the catalog renders items as grid cards showing cover art, title, publisher, and status glyph

#### Scenario: Layout switch does not affect filtered result set
- **WHEN** the user switches the catalog layout while a search query is active
- **THEN** the matched item count and visible items remain the same
