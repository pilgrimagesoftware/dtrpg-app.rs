## MODIFIED Requirements

### Requirement: Toolbar MUST provide a sort dropdown
The toolbar MUST render a sort control that allows the user to choose from: Title (A–Z), Publisher, Date Added, and Page Count. The selected sort method MUST persist while the user switches layout modes. The control MUST display a leading sort icon alongside its label, shown regardless of which sort method or direction is currently active.

#### Scenario: Selecting a sort method reorders the catalog
- **WHEN** the user selects a sort option from the sort dropdown
- **THEN** the catalog items reorder according to the selected method without changing the active filter or layout

#### Scenario: Sort state is preserved across layout switches
- **WHEN** the user changes the catalog layout while a non-default sort is active
- **THEN** the sort method remains unchanged

#### Scenario: Sort icon is always visible
- **WHEN** the toolbar is rendered, regardless of the active sort method or direction
- **THEN** a leading sort icon is shown on the sort dropdown button alongside its label
