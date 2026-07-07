## ADDED Requirements

### Requirement: Window menu contains tab-selection items
The "Window" menu SHALL contain ten tab-selection items, one for each position `0` through
`9`, in addition to its existing Minimize, Zoom, Show Activity, and Show Alert History
items. Behavior of these items is defined by the `catalog-tab-cmd-number-shortcuts`
capability.

#### Scenario: Tab-selection items appear alongside existing Window menu items
- **WHEN** the user opens the Window menu
- **THEN** it shows Minimize, Zoom, Show Activity, Show Alert History, and the ten
  tab-selection items
