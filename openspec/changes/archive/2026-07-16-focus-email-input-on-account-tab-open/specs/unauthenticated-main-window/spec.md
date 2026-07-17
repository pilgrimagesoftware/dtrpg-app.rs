## MODIFIED Requirements

### Requirement: Sign-in MUST be available from the Settings Account tab
The Settings panel Account tab MUST provide email and password inputs and a
sign-in action when the user is not authenticated, replacing the removed
standalone login window. On submission, the app SHALL exchange the email and
password for an application key via the SDK credential exchange, then
exchange that application key for session tokens via the existing
authentication call. When Settings is opened via the "not signed in"
notification banner's sign-in action, the Account tab's email input SHALL
be focused as soon as the tab is shown, without requiring the user to click
into it first.

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

#### Scenario: Email input is focused when Settings opens via the sign-in banner
- **WHEN** the user clicks the "not signed in" notification banner's sign-in action
- **THEN** the Settings window opens (or comes to front) on the Account tab
- **AND** the email input field receives keyboard focus without any further click

#### Scenario: Manually opening the Account tab does not force focus
- **WHEN** the user manually switches to the Account tab while Settings is already open
  (not via the banner's sign-in action)
- **THEN** the email input's focus state is left as it was, matching today's behavior

### Requirement: Avatar button MUST provide sign-in action when unauthenticated
When the user is not authenticated, the avatar button MUST show a sign-in affordance (tooltip or menu item) in addition to the existing "Not signed in" visual state.

#### Scenario: Avatar menu when unauthenticated
- **WHEN** the user clicks the avatar button while not signed in
- **THEN** a dropdown or action is available to open Settings to the Account tab
