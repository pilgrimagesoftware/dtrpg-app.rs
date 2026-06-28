## MODIFIED Requirements

### Requirement: Rust account menu MUST expose account actions safely
The Rust desktop app MUST provide an account button menu or equivalent compact popover that reflects the current authentication state. When authenticated, it displays DriveThruRPG account identity, settings navigation, and a sign-out action. When unauthenticated, it displays a "Not signed in" indicator and a "Sign In" action that opens the Settings Account tab — without passively showing raw access-token values in either state.

#### Scenario: Opening the Rust account menu when authenticated
- **WHEN** the user opens the account menu while signed in
- **THEN** the menu exposes account identity (email or initial), settings access, and a sign-out action without raw token disclosure

#### Scenario: Opening the Rust account menu when unauthenticated
- **WHEN** the user opens the account menu while not signed in
- **THEN** the menu shows a "Not signed in" label and a "Sign In" item that opens Settings to the Account tab
