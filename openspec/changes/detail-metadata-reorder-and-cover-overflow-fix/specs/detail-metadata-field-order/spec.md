## ADDED Requirements

### Requirement: Detail tab metadata table groups related fields per row

The expanded detail tab's metadata table SHALL render system paired with release date on
one row, format paired with file size on the next row, and category last, with the
category value prefixed by a folder icon.

#### Scenario: Rendering the metadata table

- **WHEN** the detail tab renders an item's metadata
- **THEN** system and release date appear on the same row, format and file size appear on
  the same row, and category is the last row with a folder icon before its value
