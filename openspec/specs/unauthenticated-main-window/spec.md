# unauthenticated-main-window Specification

## Purpose
TBD - created by archiving change always-open-main-window. Update Purpose after archive.
## Requirements
### Requirement: Main window MUST open unconditionally at startup
The app MUST open the main library window regardless of authentication state. No standalone login window SHALL be presented at startup or at any other time.

#### Scenario: App starts with no stored API key
- **WHEN** the app starts and no API key is found in the keychain
- **THEN** the main library window opens with an unauthenticated auth state and an empty catalog

#### Scenario: App starts with a stored API key but re-authentication fails
- **WHEN** the app starts, an API key is found, and the silent re-authentication call fails
- **THEN** the main library window opens with an unauthenticated auth state rather than blocking or showing a login window

#### Scenario: App starts with a stored API key and re-authentication succeeds
- **WHEN** the app starts and silent re-authentication succeeds
- **THEN** the main library window opens with an authenticated auth state and the catalog loads normally

### Requirement: Catalog MUST reflect local content when unauthenticated
The catalog MUST display locally downloaded content when the user is not authenticated. When no local content exists, an appropriate empty state SHALL be shown rather than an error.

#### Scenario: Unauthenticated user with no local content
- **WHEN** the user is not signed in and no content has been downloaded
- **THEN** the catalog shows an empty state (not an error banner)

#### Scenario: Unauthenticated user with downloaded content
- **WHEN** the user is not signed in but has previously downloaded titles
- **THEN** the catalog displays those titles without requiring authentication

### Requirement: Notification banner MUST indicate unauthenticated state
When the user is not authenticated, a notification banner MUST appear at the top of the main window informing them and providing a direct action to sign in.

#### Scenario: Banner shown when not signed in
- **WHEN** auth state is Unauthenticated or SessionExpired
- **THEN** a notification banner is visible with a message and a "Sign In" or "Sign in again" action button

#### Scenario: Banner dismissed by user
- **WHEN** the user dismisses the banner
- **THEN** the banner is hidden for the current session but may reappear on the next launch

#### Scenario: Banner clears on successful sign-in
- **WHEN** the user successfully authenticates (from settings or the avatar menu)
- **THEN** the banner is removed and the auth state transitions to Authenticated

### Requirement: Sign-in MUST be available from the Settings Account tab
The Settings panel Account tab MUST provide API key input and a sign-in action when the user is not authenticated, replacing the removed standalone login window.

#### Scenario: Account tab shows sign-in form when unauthenticated
- **WHEN** the user opens Settings and the Account tab while not signed in
- **THEN** an API key input field and a "Sign In" button are presented

#### Scenario: Successful sign-in from settings
- **WHEN** the user enters a valid API key and clicks "Sign In"
- **THEN** the session is established, the Account tab updates to show the signed-in state, and the catalog begins loading

#### Scenario: Failed sign-in from settings
- **WHEN** the user enters an invalid API key and clicks "Sign In"
- **THEN** an error message is shown in the Account tab and the user remains unauthenticated

### Requirement: Avatar button MUST provide sign-in action when unauthenticated
When the user is not authenticated, the avatar button MUST show a sign-in affordance (tooltip or menu item) in addition to the existing "Not signed in" visual state.

#### Scenario: Avatar menu when unauthenticated
- **WHEN** the user clicks the avatar button while not signed in
- **THEN** a dropdown or action is available to open Settings to the Account tab

