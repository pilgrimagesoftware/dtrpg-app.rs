# gpui-component-description-list Specification

## Purpose
TBD - created by archiving change gpui-component-view-rework. Update Purpose after archive.
## Requirements
### Requirement: Detail panel metadata uses DescriptionList component
The item metadata section in the detail panel SHALL use `gpui_component::DescriptionList` in horizontal (two-column) mode, replacing the hand-crafted loop of key/value `div()` rows in `render_metadata_table`.

#### Scenario: Metadata displayed in labeled rows
- **WHEN** an item is selected in the detail panel
- **THEN** a `DescriptionList` renders rows for: System, Category, Format, Pages, File size, Released, Status — each with a label column and a value column

#### Scenario: Status row reflects download state
- **WHEN** an item has `status == ItemStatus::Downloaded`
- **THEN** the Status row in the DescriptionList shows "On this device"

#### Scenario: Status row reflects cloud state
- **WHEN** an item has `status == ItemStatus::Cloud`
- **THEN** the Status row in the DescriptionList shows "In the cloud"

