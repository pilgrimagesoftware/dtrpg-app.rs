## ADDED Requirements

### Requirement: Collections section supports sorting by name, date created, or item count
The sidebar Collections section header SHALL provide a sort control offering three sort
methods — name (alphabetical), date created, and item count — combined with an ascending or
descending direction.

#### Scenario: Sorting by name
- **WHEN** the user selects "Name" as the sort method
- **THEN** collection rows are ordered alphabetically by their display name

#### Scenario: Sorting by date created
- **WHEN** the user selects "Date Created" as the sort method
- **THEN** collection rows are ordered by the collection's creation timestamp

#### Scenario: Sorting by item count
- **WHEN** the user selects "Item Count" as the sort method
- **THEN** collection rows are ordered by the number of catalog items belonging to each
  collection (the same count shown in each row's badge), not the raw API item count

#### Scenario: Reversing sort direction
- **WHEN** the user toggles the sort direction between ascending and descending
- **THEN** the Collections section re-renders with the row order reversed for the current
  sort method

### Requirement: Collections sort preference persists across sessions
The selected collections sort method and direction SHALL be saved to `UiPrefs` and restored
on the next app launch.

#### Scenario: Sort choice survives restart
- **WHEN** the user selects a sort method and direction, then restarts the app
- **THEN** the Collections section renders using the previously selected sort method and
  direction without requiring the user to reselect it

#### Scenario: Default sort on first launch
- **WHEN** no collections sort preference has ever been saved
- **THEN** the Collections section defaults to ascending name sort
