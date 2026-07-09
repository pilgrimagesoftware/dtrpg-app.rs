## ADDED Requirements

### Requirement: Alert history entries can be copied to the clipboard
Each entry row in the alert history panel SHALL provide a button that copies that entry's
full, untruncated error message text to the system clipboard.

#### Scenario: Copying an error message
- **WHEN** the user clicks the copy button on an alert history entry row
- **THEN** the entry's error message text is written to the system clipboard

#### Scenario: Copy button is discoverable without cluttering the row
- **WHEN** the user is not hovering over an alert history entry row
- **THEN** the copy button is not visible
- **WHEN** the user hovers over an alert history entry row
- **THEN** the copy button becomes visible for that row

#### Scenario: Copying a truncated message copies the full text
- **WHEN** an entry's error message is long enough that its display text is visually truncated
- **THEN** clicking the copy button copies the entry's full, untruncated error message, not the
  truncated display text
