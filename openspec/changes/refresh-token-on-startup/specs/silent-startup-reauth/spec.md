## ADDED Requirements

### Requirement: App authenticates with stored API key on startup
When a stored API key is found in the keyring on application startup, the app SHALL call the
auth endpoint to acquire a fresh access token before opening the library window. The new
access token (and refresh token if provided) SHALL be stored in the keyring, replacing any
previously stored values.

#### Scenario: Successful silent re-authentication
- **WHEN** the app starts and a valid API key is in the keyring
- **AND** the auth endpoint returns a new token
- **THEN** the new access token SHALL be stored in the keyring
- **AND** the library window SHALL open immediately (no login prompt shown)

#### Scenario: Auth call fails on startup
- **WHEN** the app starts and a valid API key is in the keyring
- **AND** the auth endpoint returns an error (network failure, invalid key, etc.)
- **THEN** the stored access token SHALL be deleted from the keyring
- **AND** the login window SHALL open instead of the library window

#### Scenario: No stored API key — behavior unchanged
- **WHEN** the app starts and no API key is in the keyring
- **THEN** the login window SHALL open as before, with no auth attempt made

### Requirement: Login window pre-populates API key after failed silent auth
When the silent re-authentication fails, the login window SHALL pre-populate the API key
field with the key that was used in the failed attempt so the user does not have to re-type it.

#### Scenario: Failed auth pre-fills the API key field
- **WHEN** silent re-authentication fails
- **AND** the login window opens
- **THEN** the API key input field SHALL contain the stored API key value
