## ADDED Requirements

### Requirement: Rust item list columns MUST be user-resizable

The item list rendered by `render_item_tier` for multi-item entries SHALL use `gpui-component`'s
`DataTable` with `TableState::col_resizable(true)`, and each of the Name, Type, and Status columns
SHALL be individually marked `resizable(true)`, so the user can drag a column divider to resize
that column.

#### Scenario: Dragging a column divider resizes the column

- **WHEN** the user drags the divider between two item list column headers
- **THEN** the column to the left of the divider takes on the new width and the item list layout
  reflows to match

#### Scenario: Resized widths persist for the life of the open tab

- **WHEN** the user resizes an item list column and then selects a different row or scrolls the
  list
- **THEN** the column keeps the user-set width

#### Scenario: Resized widths reset when the tab is closed and reopened

- **WHEN** the user resizes an item list column, closes the detail tab, and reopens it for the
  same entry
- **THEN** the item list columns render at their default widths
