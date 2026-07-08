## ADDED Requirements

### Requirement: Cmd-0 always selects the Catalog tab
Pressing `cmd-0` SHALL activate the Catalog tab, regardless of how many detail tabs are
open or which tab is currently active. Catalog SHALL NOT be a target of any other
`cmd-<n>` shortcut.

#### Scenario: Cmd-0 from a detail tab
- **WHEN** a detail tab is active and the user presses `cmd-0`
- **THEN** the Catalog tab becomes active

#### Scenario: Cmd-0 when Catalog is already active
- **WHEN** the Catalog tab is already active and the user presses `cmd-0`
- **THEN** the Catalog tab remains active (no-op, no error)

### Requirement: Cmd-1 through Cmd-9 select the open detail tab at that position
Pressing `cmd-<n>` for `n` in `1..=9` SHALL activate the nth open *detail* tab (the
closable tabs opened by double-clicking a catalog item), counted in the order they appear
in the tab strip after the Catalog tab. The Catalog tab is never a target of `cmd-1`
through `cmd-9`.

#### Scenario: Cmd-1 targets the first open detail tab
- **WHEN** one detail tab is open and the user presses `cmd-1`
- **THEN** that detail tab becomes active

#### Scenario: Cmd-2 targets the second open detail tab
- **WHEN** two detail tabs are open and the user presses `cmd-2`
- **THEN** the second detail tab becomes active

#### Scenario: Cmd-<n> with no tab at that position is a no-op
- **WHEN** fewer than `n` detail tabs are open and the user presses `cmd-<n>`
- **THEN** the active tab does not change and no error occurs

### Requirement: Window menu exposes a Select Tab submenu with items for positions 0 through 9
The Window menu SHALL contain a "Select Tab" submenu holding ten items, one per position
`0` through `9`, each dispatching the same action as the corresponding `cmd-<n>` shortcut
(position `0` is Catalog; positions `1` through `9` are the 1st through 9th open detail
tab). Each item's label SHALL reflect the open tab's title when a tab occupies that
position, and each item SHALL be disabled (not removed) when no tab occupies that
position. The items are nested under this submenu, not listed directly in the Window
menu, so the ten fixed slots don't lengthen the Window menu itself.

#### Scenario: Menu item enabled and labeled for an open detail tab
- **WHEN** a detail tab titled "Curse of Strahd" is the 2nd open detail tab
- **THEN** the "Select Tab" submenu's position-2 item is enabled and labeled with that
  tab's title (or a truncated form of it)

#### Scenario: Menu item disabled for an unoccupied position
- **WHEN** only the Catalog tab is open (no detail tabs, so positions 1 through 9 are
  unoccupied)
- **THEN** the "Select Tab" submenu's items for positions 1 through 9 are present but
  disabled

#### Scenario: Menu item selecting a tab via click
- **WHEN** the user clicks an enabled position-<n> item in the "Select Tab" submenu
- **THEN** the tab at that position becomes active, identical to pressing `cmd-<n>`

### Requirement: Select Tab submenu check-marks the currently active tab
The "Select Tab" submenu item whose position holds the currently active tab SHALL be
check-marked. At most one item is check-marked at a time. An unoccupied or disabled
position is never check-marked.

#### Scenario: Catalog is active
- **WHEN** the Catalog tab is the active tab
- **THEN** the "Select Tab" submenu's position-0 item is check-marked and no other item is

#### Scenario: A detail tab is active
- **WHEN** the 2nd open detail tab is the active tab
- **THEN** the "Select Tab" submenu's position-2 item is check-marked and no other item is

### Requirement: Tab-selection menu state stays live as tabs open and close
The "Select Tab" submenu's items SHALL reflect the current open-tab list without
requiring the user to reopen the menu bar or restart the app.

#### Scenario: Opening a detail tab enables its menu item
- **WHEN** the user double-clicks a catalog item to open a new detail tab as the 2nd open
  detail tab
- **THEN** the "Select Tab" submenu's position-2 item becomes enabled and labeled with
  that tab's title on the next time the menu bar is queried

#### Scenario: Closing a detail tab disables its former menu item
- **WHEN** the user closes the only open detail tab, previously at position 1
- **THEN** the "Select Tab" submenu's position-1 item becomes disabled again

#### Scenario: Activating a tab via the tab strip updates the checkmark
- **WHEN** the user clicks a different tab directly in the tab strip
- **THEN** the "Select Tab" submenu's checkmark moves to that tab's position on the next
  time the menu bar is queried
