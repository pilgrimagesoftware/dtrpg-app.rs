## ADDED Requirements

### Requirement: View menu contains a Select Tab submenu
The View menu SHALL contain a "Select Tab" submenu holding ten tab-selection items,
one for each position `0` through `9`, in addition to its existing Full Screen,
Presentation, Sort, and Find in Library items. The items are nested in the submenu
rather than listed directly in the View menu, so the ten fixed slots don't lengthen the
View menu itself. Behavior of the submenu's items is defined by the
`catalog-tab-cmd-number-shortcuts` capability.

#### Scenario: Select Tab submenu appears alongside existing View menu items
- **WHEN** the user opens the View menu
- **THEN** it shows Full Screen, Presentation, Sort, Find in Library, and a "Select Tab"
  submenu item

#### Scenario: Select Tab submenu holds the ten tab-selection items
- **WHEN** the user opens the "Select Tab" submenu
- **THEN** it shows ten items for positions `0` through `9`
