## ADDED Requirements

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
