## ADDED Requirements

### Requirement: List view columns are user-resizable
The catalog list DataTable SHALL allow the user to drag column dividers to resize individual columns. The title column and all other data columns (Publisher, System, Pages, Size, Added) MUST be marked as resizable. The Status and Reveal columns MUST remain non-resizable.

#### Scenario: User drags a column divider
- **WHEN** the user drags the divider between two resizable columns
- **THEN** the column width updates in real time and the other columns stay at their current widths

#### Scenario: Non-resizable columns cannot be dragged
- **WHEN** the user attempts to drag the border of the Status or Reveal column
- **THEN** no resize handle is shown and the column width does not change
