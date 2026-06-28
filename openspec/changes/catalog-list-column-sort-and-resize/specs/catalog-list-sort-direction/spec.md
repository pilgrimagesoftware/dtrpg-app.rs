## ADDED Requirements

### Requirement: Sort direction is selectable from the toolbar sort menu
The toolbar sort dropdown SHALL include "Ascending" and "Descending" items below a separator. The currently active direction SHALL be shown as checked. Selecting a direction item SHALL change the sort direction for whichever sort method is currently active without changing the method.

#### Scenario: Ascending selected from dropdown
- **WHEN** the user selects "Ascending" from the sort dropdown
- **THEN** the current sort method is applied in ascending order and "Ascending" is shown as checked

#### Scenario: Descending selected from dropdown
- **WHEN** the user selects "Descending" from the sort dropdown
- **THEN** the current sort method is applied in descending order and "Descending" is shown as checked

#### Scenario: Direction auto-updates from column header click
- **WHEN** the user clicks a column header and the sort direction changes (e.g., from descending to ascending)
- **THEN** the toolbar sort dropdown automatically shows the new direction as checked

### Requirement: Sort direction applies to all sort methods
Every sort method (Title, Publisher, Date Added, Pages, and Custom column sorts) SHALL respect the active sort direction. Ascending sorts data from smallest/earliest/A first; descending sorts from largest/latest/Z first.

#### Scenario: Publisher sort ascending
- **WHEN** the sort method is Publisher and direction is Ascending
- **THEN** items are listed A → Z by publisher name

#### Scenario: Pages sort descending
- **WHEN** the sort method is Pages and direction is Descending
- **THEN** items are listed from highest page count to lowest
