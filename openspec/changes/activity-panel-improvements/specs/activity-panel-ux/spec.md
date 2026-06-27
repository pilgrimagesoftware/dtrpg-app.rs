## ADDED Requirements

### Requirement: Activity item list is scrollable

The system SHALL render the activity item list in a scrollable container so that items beyond the panel's maximum height are reachable by scrolling.

#### Scenario: More items than visible height

- **WHEN** the activity panel is open and the number of items exceeds the panel's maximum height
- **THEN** the user can scroll the item list to reach items that were initially out of view

#### Scenario: Items fit within panel height

- **WHEN** the activity panel is open and all items fit within the maximum height
- **THEN** no scroll indicator is shown and all items are immediately visible

### Requirement: Hovering an activity item row shows a tooltip with full text

The system SHALL display a tooltip containing the item's full label when the user hovers over an activity item row. For error items, the tooltip SHALL additionally include the error message.

#### Scenario: Hover over a label-only item

- **WHEN** the user hovers over an activity item row whose status is `InProgress` or `Complete`
- **THEN** a tooltip appears showing the item's full label text

#### Scenario: Hover over an error item

- **WHEN** the user hovers over an activity item row whose status is `Error`
- **THEN** a tooltip appears showing the full label and the full error message

### Requirement: Clicking an activity item row toggles its expanded state

The system SHALL expand an activity item row when clicked, showing the full label and error message with word wrap instead of truncation. Clicking the same row again SHALL collapse it back to the truncated single-line view.

#### Scenario: Click to expand

- **WHEN** the user clicks an activity item row that is not expanded
- **THEN** the row expands to show the full label (word-wrapped, no truncation) and the full error message if applicable

#### Scenario: Click to collapse

- **WHEN** the user clicks an activity item row that is currently expanded
- **THEN** the row collapses back to the single-line truncated view

#### Scenario: Clicking a second item collapses the first

- **WHEN** an activity item row is expanded and the user clicks a different item row
- **THEN** the previously expanded row collapses and the newly clicked row expands

### Requirement: Activity panel shows a visually prominent empty state

The system SHALL render a centered empty state with an icon and descriptive copy when the activity item list is empty (no in-progress or recent items).

#### Scenario: Empty panel with no activity

- **WHEN** the activity panel is open and both `in_progress` and `recent` are empty
- **THEN** the panel body shows a centered icon, the message "No recent activity.", and a secondary line "Activity will appear here as operations run."
