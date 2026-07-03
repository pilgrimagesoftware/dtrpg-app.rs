## ADDED Requirements

### Requirement: Grouped list presentation uses native table sections

The system SHALL render the "group by publisher" list presentation through
`CatalogListDelegate`'s sections API (`sections_count`, `items_count`,
`render_section_header`), sharing the same virtualized `DataTable` instance and column
behavior as the ungrouped list presentation.

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
- **THEN** the section header shows the publisher name and the number of items in that
  section
