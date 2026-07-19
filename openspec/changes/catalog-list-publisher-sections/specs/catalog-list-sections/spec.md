## ADDED Requirements

### Requirement: Grouped list presentation uses a virtualized table

The system SHALL render the "group by publisher" list presentation through a dedicated
`TableDelegate` (`GroupedCatalogListDelegate`) over a flattened list of header and item rows,
giving it the same `DataTable` virtualization as the ungrouped list presentation, and
sharing the ungrouped list's column definitions and (via propagated
`TableEvent::ColumnWidthsChanged`) user-resized column widths.

#### Scenario: Grouped list with a large catalog scrolls smoothly

- **WHEN** "group by publisher" is enabled in list view with a catalog large enough to
  require virtualization
- **THEN** scrolling is smooth and only visible rows are rendered

#### Scenario: Column widths match the ungrouped list

- **WHEN** the user resizes a column in the ungrouped list view and then switches to
  grouped view
- **THEN** the same column width is applied in the grouped view

#### Scenario: Section header shows publisher and item count

- **WHEN** the grouped list view renders a publisher section
- **THEN** the header row shows the publisher name and the number of items in that section

#### Scenario: Header rows are not selectable

- **WHEN** the user clicks or double-clicks a publisher header row in the grouped list
- **THEN** no item is selected and no detail tab opens
