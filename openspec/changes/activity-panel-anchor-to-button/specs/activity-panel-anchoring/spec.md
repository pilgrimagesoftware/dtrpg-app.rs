## ADDED Requirements

### Requirement: Activity and alert history panels anchor to their triggering button

The system SHALL open the activity panel and alert history panel anchored to the on-screen
position of the status bar button that triggered them, rather than a fixed window corner.

#### Scenario: Opening the activity panel

- **WHEN** the user clicks the status bar activity button
- **THEN** the activity panel opens directly above (or adjacent to) that button

#### Scenario: Opening the alert history panel

- **WHEN** the user clicks the status bar notification button
- **THEN** the alert history panel opens directly above (or adjacent to) that button

#### Scenario: Window resize

- **WHEN** the window is resized while a panel is open
- **THEN** the panel's anchor tracks the button's new on-screen position on next open
