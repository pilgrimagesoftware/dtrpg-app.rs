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
