# window-menu Specification

## Purpose
TBD - created by archiving change catalog-collections-improvements. Update Purpose after archive.
## Requirements
### Requirement: Window menu contains Show Activity action
The "Window" menu SHALL contain a "Show Activity" menu item that makes the activity panel visible.

#### Scenario: Show Activity reveals the panel
- **WHEN** the user selects "Window > Show Activity"
- **THEN** the activity panel becomes visible, equivalent to clicking the activity toolbar button

#### Scenario: Show Activity is available when panel is hidden
- **WHEN** the activity panel is hidden and the user selects "Window > Show Activity"
- **THEN** the activity panel appears

### Requirement: Window menu contains Show Alert History action
The "Window" menu SHALL contain a "Show Alert History" menu item that opens the alert history panel.

#### Scenario: Show Alert History dispatches the action
- **WHEN** the user selects "Window > Show Alert History"
- **THEN** the ShowAlertHistory action is dispatched and the alert history panel is shown (or a stub panel is displayed if the feature is not yet fully implemented)

