## ADDED Requirements

### Requirement: Account info labels share a right-aligned label column
In the Account settings section's authenticated view, the Email and API Key info row labels SHALL right-align against a shared fixed-width label column, so both rows' value text begins at the same horizontal position regardless of label text length.

#### Scenario: Email and API Key rows both present
- **WHEN** the user views the Account settings section while signed in with an API key hint present
- **THEN** the "Email" and "API Key" labels are right-aligned within the same column width, and both rows' value text starts at the same horizontal offset

### Requirement: Account info label and value text align vertically within each row
Each Email/API Key row SHALL vertically center its label and value text within the row, regardless of the different font families used for label text and value text.

#### Scenario: Row with mixed label and monospace value fonts
- **WHEN** the user views an Email or API Key row, where the label uses the default UI font and the value uses the monospace font
- **THEN** the label and value text appear vertically centered together within the row, with no visible offset between them
