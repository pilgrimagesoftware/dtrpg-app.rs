## ADDED Requirements

### Requirement: API key hint uses a monospaced font
The API key hint text in the authenticated Account settings section SHALL be rendered in a monospaced font so that the masked credential value (`abcd••••••••1`) is visually distinct from surrounding proportional-font text.

#### Scenario: Hint text rendered in monospace
- **WHEN** the user is signed in and opens Settings → Account
- **THEN** the API key hint row displays in a monospaced font, distinguishable from the "Account" label and email text above it

#### Scenario: Other account text unaffected
- **WHEN** the user is signed in and opens Settings → Account
- **THEN** the "Account" label and email address text remain in the application's default proportional font
