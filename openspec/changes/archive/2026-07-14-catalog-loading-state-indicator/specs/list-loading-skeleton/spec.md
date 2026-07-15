## ADDED Requirements

### Requirement: Catalog list presentation shows a skeleton loading view

The system SHALL render `gpui-component`'s built-in skeleton loading view for the catalog
list presentation while the initial catalog fetch is in flight, instead of an empty table.

#### Scenario: Cold start with no cache

- **WHEN** the app starts with no on-disk catalog cache and the initial API fetch has not
  yet returned
- **THEN** the list presentation shows skeleton rows instead of an empty table with only
  column headers

#### Scenario: Fetch completes

- **WHEN** the initial catalog fetch completes
- **THEN** the skeleton rows are replaced by the actual catalog rows

### Requirement: Sidebar sections show a loading indicator before data arrives

The system SHALL show a loading indicator in the Publishers and Collections sidebar
sections while their respective data has not yet loaded, instead of an empty-state
message.

#### Scenario: Publishers not yet known

- **WHEN** the sidebar renders before the first catalog page has been processed
- **THEN** the Publishers section shows a loading indicator instead of "no publishers"
