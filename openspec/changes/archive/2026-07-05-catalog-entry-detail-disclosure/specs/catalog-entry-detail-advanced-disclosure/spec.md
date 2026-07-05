## ADDED Requirements

### Requirement: Advanced details section is collapsed by default
The catalog entry detail tab SHALL render an "Advanced details" disclosure section below
the existing metadata (or item tier, for multi-item entries), collapsed by default.

#### Scenario: Opening the detail tab shows the section collapsed
- **WHEN** a user double-clicks a catalog entry to open its detail tab
- **THEN** the "Advanced details" section is rendered collapsed, showing only its header

#### Scenario: Reopening a previously-viewed entry starts collapsed again
- **WHEN** a user closes a detail tab whose "Advanced details" section was expanded, then
  reopens the same entry's detail tab
- **THEN** the "Advanced details" section renders collapsed

### Requirement: Advanced details section shows fields not in the primary metadata
The "Advanced details" section SHALL, when expanded, show the catalog entry's stable id,
numeric id, order product id, product id, added-order value, and generative cover color
(as both hex value and a color swatch) — fields the primary metadata table and item tier
do not already display.

#### Scenario: Expanding the section reveals identifier fields
- **WHEN** a user clicks the "Advanced details" header to expand it
- **THEN** the section shows the entry's stable id, numeric id, order product id, product
  id, and added-order value as labeled fields

#### Scenario: Expanding the section reveals the cover color
- **WHEN** a user clicks the "Advanced details" header to expand it
- **THEN** the section shows the entry's generative cover color as both its hex string and
  a small color swatch matching that color

### Requirement: Advanced details toggle is per-entry and does not persist
The expanded/collapsed state of the "Advanced details" section SHALL be tracked per catalog
entry for the current app session only, and SHALL NOT be shared across different entries
or persisted across app restarts.

#### Scenario: Expanding one entry's section does not affect another entry
- **WHEN** a user expands the "Advanced details" section for one catalog entry, then opens
  the detail tab for a different entry
- **THEN** the second entry's "Advanced details" section renders collapsed

#### Scenario: Toggle state does not survive app restart
- **WHEN** a user expands the "Advanced details" section for an entry, then restarts the
  app and reopens that entry's detail tab
- **THEN** the "Advanced details" section renders collapsed
