## ADDED Requirements

### Requirement: Cmd-0 always selects the Catalog tab
Pressing `cmd-0` SHALL activate the Catalog tab, regardless of how many detail tabs are
open or which tab is currently active.

#### Scenario: Cmd-0 from a detail tab
- **WHEN** a detail tab is active and the user presses `cmd-0`
- **THEN** the Catalog tab becomes active

#### Scenario: Cmd-0 when Catalog is already active
- **WHEN** the Catalog tab is already active and the user presses `cmd-0`
- **THEN** the Catalog tab remains active (no-op, no error)

### Requirement: Cmd-1 through Cmd-9 select the open tab at that position
Pressing `cmd-<n>` for `n` in `1..=9` SHALL activate the tab at the 1-indexed position `n`
in the tab strip's current open-tab order (Catalog first, followed by open detail tabs in
the order they were opened).

#### Scenario: Cmd-1 targets the Catalog tab
- **WHEN** the user presses `cmd-1`
- **THEN** the Catalog tab becomes active, since it always occupies position 1

#### Scenario: Cmd-2 targets the first open detail tab
- **WHEN** one detail tab is open and the user presses `cmd-2`
- **THEN** that detail tab becomes active

#### Scenario: Cmd-<n> with no tab at that position is a no-op
- **WHEN** fewer than `n` tabs are open and the user presses `cmd-<n>`
- **THEN** the active tab does not change and no error occurs

### Requirement: Window menu exposes tab-selection items for positions 0 through 9
The Window menu SHALL contain ten items, one per position `0` through `9`, each dispatching
the same action as the corresponding `cmd-<n>` shortcut. Each item's label SHALL reflect the
open tab's title when a tab occupies that position, and each item SHALL be disabled (not
removed) when no tab occupies that position.

#### Scenario: Menu item enabled and labeled for an open tab
- **WHEN** a detail tab titled "Curse of Strahd" is open at position 2
- **THEN** the Window menu's position-2 item is enabled and labeled with that tab's title
  (or a truncated form of it)

#### Scenario: Menu item disabled for an unoccupied position
- **WHEN** only the Catalog tab is open (positions 2 through 9 unoccupied)
- **THEN** the Window menu's items for positions 2 through 9 are present but disabled

#### Scenario: Menu item selecting a tab via click
- **WHEN** the user clicks an enabled position-<n> Window menu item
- **THEN** the tab at that position becomes active, identical to pressing `cmd-<n>`

### Requirement: Tab-selection menu state stays live as tabs open and close
The Window menu's tab-selection items SHALL reflect the current open-tab list without
requiring the user to reopen the menu bar or restart the app.

#### Scenario: Opening a detail tab enables its menu item
- **WHEN** the user double-clicks a catalog item to open a new detail tab at position 2
- **THEN** the Window menu's position-2 item becomes enabled and labeled with that tab's
  title on the next time the menu bar is queried

#### Scenario: Closing a detail tab disables its former menu item
- **WHEN** the user closes the only open detail tab, previously at position 2
- **THEN** the Window menu's position-2 item becomes disabled again
