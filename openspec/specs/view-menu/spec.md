# view-menu

## Requirements

### Requirement: View menu exposes presentation, sort, and search controls
The application menu bar's "View" menu SHALL contain a "Presentation" submenu (List, Thumbs, Grid), a "Sort" submenu (Title, Publisher, Date Added, Pages; Ascending, Descending; Group by Publisher), and a "Find in Library" item that focuses the catalog search input.

#### Scenario: Presentation submenu switches catalog layout
- **WHEN** the user selects "View > Presentation > Thumbs"
- **THEN** the catalog switches to the thumbs layout, identical to using the toolbar control

#### Scenario: Sort submenu changes catalog sort
- **WHEN** the user selects "View > Sort > Publisher"
- **THEN** the catalog re-sorts by publisher, identical to using the toolbar control

#### Scenario: Find in Library focuses search
- **WHEN** the user selects "View > Find in Library"
- **THEN** keyboard focus moves to the catalog search input

### Requirement: View menu reflects current selection with checkmarks
The Presentation submenu, Sort submenu, and Group toggle SHALL show a checkmark next to the item matching the catalog's current presentation, sort field, sort direction, and grouping state, respectively. The menu bar SHALL be rebuilt whenever any of these change.

#### Scenario: Checkmark follows presentation changes
- **WHEN** the catalog presentation changes (via menu, toolbar, or keyboard)
- **THEN** the View > Presentation submenu shows a checkmark next to the newly active mode and no checkmark on the others

#### Scenario: Checkmark follows sort changes made via a column header
- **WHEN** the user sorts by clicking a `DataTable` column header rather than the menu
- **THEN** the View > Sort submenu's checkmark still moves to the corresponding named sort item

#### Scenario: Checkmark follows sort direction changes
- **WHEN** the sort direction toggles between ascending and descending
- **THEN** the corresponding menu item shows the checkmark and the other does not

#### Scenario: Checkmark follows the Group by Publisher toggle
- **WHEN** "Group by Publisher" is enabled or disabled
- **THEN** the View > Sort > "Group by Publisher" item's checkmark matches the current state
