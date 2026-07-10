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

### Requirement: Window menu contains a Select Tab submenu
The "Window" menu SHALL contain a "Select Tab" submenu holding ten tab-selection items,
one for each position `0` through `9`, in addition to its existing Minimize, Zoom, Show
Activity, and Show Alert History items. The items are nested in the submenu rather than
listed directly in the Window menu, so the ten fixed slots don't lengthen the Window menu
itself. Behavior of the submenu's items is defined by the `catalog-tab-cmd-number-shortcuts`
capability.

#### Scenario: Select Tab submenu appears alongside existing Window menu items
- **WHEN** the user opens the Window menu
- **THEN** it shows Minimize, Zoom, Show Activity, Show Alert History, and a "Select Tab"
  submenu item

#### Scenario: Select Tab submenu holds the ten tab-selection items
- **WHEN** the user opens the "Select Tab" submenu
- **THEN** it shows ten items for positions `0` through `9`

