## ADDED Requirements

### Requirement: Alert history entries support copying the error message
Each row in the alert history panel SHALL provide a control that copies that entry's error
message text to the system clipboard.

#### Scenario: Copying an alert's error message
- **WHEN** the user hovers an alert history row and clicks its copy control
- **THEN** the entry's error message text is copied to the system clipboard exactly as
  displayed, and the label or timestamp are not included

#### Scenario: Copy control only visible on hover
- **WHEN** the user is not hovering an alert history row
- **THEN** that row's copy control is not visible
