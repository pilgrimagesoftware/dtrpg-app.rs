## ADDED Requirements

### Requirement: Column widths are preserved across library updates
The catalog list view SHALL retain user-resized column widths when the library data updates, including sort changes, filter changes, and data reloads. Calling `TableState::refresh()` SHALL NOT reset any column to its static default width if the user has previously resized it.

#### Scenario: Width survives a sort change
- **WHEN** the user resizes a column in the catalog list view
- **AND** then changes the sort order (via toolbar dropdown or column header click)
- **THEN** the column retains the user-set width after the sort update

#### Scenario: Width survives a filter change
- **WHEN** the user resizes a column in the catalog list view
- **AND** then changes the sidebar filter (e.g., All Titles → On This Device)
- **THEN** the column retains the user-set width after the filter update

#### Scenario: Unresized columns use static defaults
- **WHEN** the user has not resized a column
- **THEN** the column SHALL use the width defined in `list_columns()`

#### Scenario: All columns are independently preserved
- **WHEN** the user resizes multiple columns
- **THEN** each column's width is preserved independently; resizing one column does not affect another column's stored width
