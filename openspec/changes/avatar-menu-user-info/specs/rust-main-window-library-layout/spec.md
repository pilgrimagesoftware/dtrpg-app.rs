## MODIFIED Requirements

### Requirement: Rust account menu MUST expose account actions safely
The Rust desktop app MUST provide an account button menu or equivalent compact popover that displays DriveThruRPG account identity or connection status, token set/reset actions, and settings navigation without passively showing raw access-token values. When a user is signed in, the account menu SHALL display the account email address as a non-interactive label at the top of the menu, followed by a separator, followed by account action items. The email address SHALL NOT reveal any credential value (API key, access token, or refresh token).

#### Scenario: Opening the Rust account menu
- **WHEN** the user opens the account menu
- **THEN** the menu exposes account status, token management actions, and settings access without raw token disclosure

#### Scenario: Account menu shows email identity
- **WHEN** a user is signed in and opens the avatar button menu
- **THEN** the menu displays the account email address as a non-interactive header above a visual separator, followed by the "Log Out" action

#### Scenario: Account menu without signed-in user
- **WHEN** no user is signed in and the unauthenticated avatar button is present
- **THEN** no dropdown menu with identity information is shown
