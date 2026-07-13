# account-section-layout Specification

## Purpose
TBD - created by archiving change account-reset-api-key-inline. Update Purpose after archive.
## Requirements
### Requirement: Reset API Key button is positioned inline to the right of the account info
In the Account settings section, the Reset API Key button SHALL appear on the same horizontal row as the account info text, right-aligned, rather than in the vertical button stack below the divider. The button SHALL display a reload/refresh symbol (↺) as its label rather than text, and SHALL have a tooltip reading "Reset API Key".

#### Scenario: Account section renders with Reset API Key inline
- **WHEN** the user opens the Account settings section
- **THEN** the "Account" label and "Signed in to DriveThruRPG" subtitle appear on the left, and a circular reload symbol button appears on the right of the same row

#### Scenario: Reset API Key button shows tooltip on hover
- **WHEN** the user hovers over the reload symbol button in the account info row
- **THEN** a tooltip displaying "Reset API Key" appears

#### Scenario: Log Out is the only button below the divider
- **WHEN** the user views the actions area below the divider in the Account settings section
- **THEN** only the "Log Out" button is present; the Reset API Key button does not appear there

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

