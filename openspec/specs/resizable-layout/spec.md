# resizable-layout Specification

## Purpose
TBD - created by archiving change adopt-gpui-component-primitives. Update Purpose after archive.

## Requirements

### Requirement: Main window layout uses resizable panels

The main library window SHALL use `h_resizable` to split the layout into three panels: left navigation sidebar, catalog content area, and right detail panel. The user SHALL be able to drag the resize handles between panels.

#### Scenario: User drags sidebar resize handle
- **WHEN** the user drags the handle between the sidebar and the catalog content panel
- **THEN** the sidebar width changes in real time within its configured min/max bounds

#### Scenario: Sidebar has a minimum width
- **WHEN** the user drags the sidebar narrower than the minimum
- **THEN** the sidebar width clamps at the minimum (180 px) and does not collapse further

#### Scenario: Detail panel only occupies space when an item is selected
- **WHEN** no item is selected
- **THEN** the detail panel is not rendered and the catalog content fills the remaining width
- **WHEN** an item is selected
- **THEN** the detail panel appears with a minimum width of 240 px

### Requirement: Panel sizes persist across sessions

The user's preferred sidebar and detail panel widths SHALL be restored when the app relaunches.

#### Scenario: Resized panel is remembered
- **WHEN** the user resizes a panel and relaunches the app
- **THEN** the panel reopens at the previously set width
