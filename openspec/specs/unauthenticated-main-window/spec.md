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
The Settings panel Account tab MUST provide email and password inputs and a
sign-in action when the user is not authenticated, replacing the removed
standalone login window. On submission, the app SHALL exchange the email and
password for an application key via the SDK credential exchange, then
exchange that application key for session tokens via the existing
authentication call.

#### Scenario: Account tab shows sign-in form when unauthenticated
- **WHEN** the user opens Settings and the Account tab while not signed in
- **THEN** email and password input fields and a "Sign In" button are
  presented

#### Scenario: Successful sign-in from settings
- **WHEN** the user enters a valid email and password and clicks "Sign In"
- **THEN** the app exchanges the credentials for an application key, then
  exchanges that key for session tokens, the session is established, the
  Account tab updates to show the signed-in state, and the catalog begins
  loading

#### Scenario: Failed credential exchange
- **WHEN** the user enters an email/password pair DriveThruRPG rejects
- **THEN** an error message is shown in the Account tab, the application key
  exchange is not attempted, and the user remains unauthenticated

#### Scenario: Failed application key exchange after valid credentials
- **WHEN** the credential exchange succeeds but the resulting application key
  is rejected by the existing authentication call
- **THEN** an error message distinct from the credential-exchange failure is
  shown in the Account tab and the user remains unauthenticated

#### Scenario: Sign-in button enablement for the two-field form
- **WHEN** either the email or password field is empty or contains only
  whitespace
- **THEN** the "Sign In" button is disabled

#### Scenario: Loading state spans both exchanges
- **WHEN** the user clicks "Sign In" and either the credential exchange or
  the application key exchange has not yet completed
- **THEN** a loading indicator is visible and the input fields and "Sign In"
  button are disabled

#### Scenario: Email pre-filled on reauthentication
- **WHEN** a stored credential entry has an email but the stored application
  key is invalid or expired
- **THEN** the Account tab pre-fills the email field so the user only
  re-enters their password

### Requirement: Avatar button MUST provide sign-in action when unauthenticated
When the user is not authenticated, the avatar button MUST show a sign-in affordance (tooltip or menu item) in addition to the existing "Not signed in" visual state.

#### Scenario: Avatar menu when unauthenticated
- **WHEN** the user clicks the avatar button while not signed in
- **THEN** a dropdown or action is available to open Settings to the Account tab

