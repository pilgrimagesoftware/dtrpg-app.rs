## ADDED Requirements

### Requirement: Column headers are clickable for sorting
Each data column header in the list DataTable SHALL show a sort indicator icon and respond to clicks to change the active sort. Clicking a column header cycles: no active indicator → descending → ascending → (reset to default). The Status and Reveal columns SHALL NOT be sortable.

#### Scenario: First click on an unsorted column
- **WHEN** the user clicks a column header that is not the current sort column
- **THEN** the catalog is sorted by that column descending and the header shows a descending indicator

#### Scenario: Second click toggles to ascending
- **WHEN** the user clicks the currently-sorted column header (which is sorted descending)
- **THEN** the sort direction becomes ascending and the header shows an ascending indicator

#### Scenario: Third click resets to default sort
- **WHEN** the user clicks the currently-sorted column header (which is sorted ascending)
- **THEN** the sort resets to the default (Title, Ascending) and the column header returns to the neutral indicator

### Requirement: Column sort is reflected in the toolbar sort selector
When the user sorts via a column header, the toolbar sort selector SHALL update to reflect the new state: showing the matching named sort (Title, Publisher, Date Added, Pages) if applicable, or showing "Custom" for columns with no named toolbar equivalent (System, Size).

#### Scenario: Column header sort matches named sort entry
- **WHEN** the user clicks the Publisher column header to sort
- **THEN** the sort selector shows "Publisher" as the active item

#### Scenario: Column header sort has no named entry
- **WHEN** the user clicks the System column header to sort
- **THEN** the sort selector shows "Custom" as the active item (read-only indicator)

### Requirement: Toolbar sort selection updates column header indicator
When the user picks a named sort from the toolbar dropdown, the DataTable column header for the corresponding column SHALL show the active sort indicator, and all other column headers SHALL show the neutral indicator.

#### Scenario: Named sort selected from toolbar
- **WHEN** the user selects "Title" from the sort dropdown
- **THEN** the Title column header shows the active sort direction indicator and all other columns show the neutral indicator
