## MODIFIED Requirements

### Requirement: Rust search and filter controls MUST be disclosable
The Rust desktop app MUST provide a low-profile disclosable search/filter area with search input, view mode, and a sort/grouping control, plus a collapsed summary of active browsing state. The sort control SHALL display a dropdown caret to indicate it opens a menu. Grouping by publisher SHALL be accessible as a toggleable menu item inside the sort menu, not as a separate button.

#### Scenario: Toggling Rust filter disclosure
- **WHEN** the user expands or collapses the search/filter area
- **THEN** the Rust app preserves active search, filter, view mode, grouping, and sort state

#### Scenario: Sort button indicates interactivity
- **WHEN** the user views the toolbar
- **THEN** the sort button displays the current sort label and a dropdown caret (chevron) indicator

#### Scenario: Group by Publisher is accessible from sort menu
- **WHEN** the user opens the sort menu
- **THEN** the menu shows sort order items (Title, Publisher, Date Added, Pages) followed by a separator and a checkable "Group by Publisher" item

#### Scenario: Toggling Group by Publisher from sort menu
- **WHEN** the user clicks "Group by Publisher" in the sort menu
- **THEN** the grouping state toggles and the checkmark reflects the new state on next menu open

#### Scenario: No standalone group toggle button
- **WHEN** the user views the toolbar
- **THEN** no separate "Group" toggle button is present; grouping is accessible only through the sort menu
