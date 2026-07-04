# detail-panel-resizable Specification

## Purpose
TBD - created by archiving change detail-panel-resizable-and-wrapping. Update Purpose after archive.
## Requirements
### Requirement: Detail panel width is user-adjustable via drag
The detail panel SHALL expose a drag handle on its left edge. Dragging the handle left or right SHALL continuously adjust the panel width. The panel width SHALL be constrained to a minimum of 240 px and a maximum of 600 px.

#### Scenario: Drag handle widens the panel
- **WHEN** the user drags the left-edge handle to the left
- **THEN** the detail panel width increases up to the 600 px maximum

#### Scenario: Drag handle narrows the panel
- **WHEN** the user drags the left-edge handle to the right
- **THEN** the detail panel width decreases down to the 240 px minimum

#### Scenario: Width is clamped at minimum
- **WHEN** the user drags the handle such that the computed width would be less than 240 px
- **THEN** the panel width stays at 240 px and does not shrink further

#### Scenario: Width is clamped at maximum
- **WHEN** the user drags the handle such that the computed width would exceed 600 px
- **THEN** the panel width stays at 600 px and does not grow further

### Requirement: Detail panel width persists within a session
The panel width chosen by dragging SHALL be preserved for the remainder of the app session. Selecting a different item or triggering any re-render SHALL NOT reset the panel to its default width.

#### Scenario: Width preserved after item change
- **WHEN** the user resizes the panel and then selects a different catalog item
- **THEN** the panel renders at the previously chosen width, not the default 320 px

#### Scenario: Width resets to default on app restart
- **WHEN** the app is restarted
- **THEN** the detail panel opens at its default width of 320 px (persistence to disk is not in scope)

### Requirement: Cover thumbnail is capped and re-centered as the panel resizes
The detail panel's cover thumbnail SHALL NOT grow past its default 320 px width when the panel is widened. As the panel width changes, the thumbnail SHALL remain horizontally centered within the panel and top-aligned within the panel body.

#### Scenario: Thumbnail does not grow past its cap when panel widens
- **WHEN** the user drags the panel wider than 320 px
- **THEN** the cover thumbnail stays at its capped width rather than growing to fill the panel

#### Scenario: Thumbnail re-centers as the panel is resized
- **WHEN** the user resizes the panel to any width
- **THEN** the cover thumbnail is horizontally centered within the panel and remains at the top of the panel body

