# zip-content-preview Specification

## Purpose
Hover-to-preview / click-to-pin popover that lists the internal entries of a Zip file row
(rendered by `detail-file-list`) in the detail tab, scoped to that detail tab's visibility.

## Requirements
### Requirement: Hovering a Zip file row shows its contents in a popover
Hovering a file row whose `ItemFile.is_zip` is `true` SHALL open an anchored popover
listing the archive's internal entries (name and size per entry) in a scrollable list.

#### Scenario: Hover opens the popover
- **WHEN** the pointer hovers over a Zip file row in the detail tab
- **THEN** a popover anchored to that row opens, listing the archive's internal entries

#### Scenario: Hover ends without a click
- **WHEN** the pointer moves off a hovered (not clicked) Zip file row
- **THEN** the popover closes

#### Scenario: Archive contents exceed the popover's visible height
- **WHEN** the archive contains more entries than fit in the popover's fixed height
- **THEN** the entry list scrolls within the popover instead of resizing the popover beyond
  a fixed maximum height

#### Scenario: Zip file cannot be read
- **WHEN** the hovered file is a Zip row but the underlying file is missing, unreadable, or
  not a valid Zip archive
- **THEN** the popover opens showing an inline "preview unavailable" state instead of an
  entry list, and the application does not panic or crash

### Requirement: Clicking a Zip file row pins the popover open
Clicking a Zip file row SHALL pin its popover open so that it remains visible after the
pointer leaves the row, until explicitly dismissed.

#### Scenario: Click pins the popover
- **WHEN** a user clicks a Zip file row while its popover is open (or to open it)
- **THEN** the popover remains open and visible after the pointer subsequently moves off
  the row

#### Scenario: Second click or explicit close dismisses a pinned popover
- **WHEN** a user clicks the same Zip file row again, or activates the popover's close
  control, while its popover is pinned open
- **THEN** the popover closes and its pinned state is cleared

### Requirement: Popover visibility is scoped to its owning detail tab
The Zip content preview popover (hovered or pinned) SHALL only be visible while its
owning item's detail tab is the active tab in the main window.

#### Scenario: Switching tabs hides the popover
- **WHEN** a Zip content preview popover is open (hovered or pinned) for the active detail
  tab, and the user switches to a different tab (catalog or another detail tab)
- **THEN** the popover is not rendered while that other tab is active

#### Scenario: Closing the detail tab clears popover state
- **WHEN** a Zip content preview popover is pinned open for a detail tab, and that detail
  tab is closed
- **THEN** the popover's hover and pinned state for that item are cleared, and reopening a
  detail tab for the same item later does not show a stale popover
