## Requirements

### Requirement: Masked API key hint in authenticated account view
When the user is signed in, the Account settings section SHALL display a masked representation of the API key. The hint SHALL reveal the first 4 characters and the last 1 character of the key, with the intervening characters replaced by a fixed-width bullet string (`••••••••`). If the key is 5 characters or fewer, the hint SHALL display only bullet characters with no revealed content.

#### Scenario: Key long enough for partial reveal
- **WHEN** the stored API key is 6 or more characters long
- **THEN** the hint shown is `<first-4>••••••••<last-1>` (e.g. `abcd••••••••1`)

#### Scenario: Key too short for partial reveal
- **WHEN** the stored API key is 5 or fewer characters long
- **THEN** the hint shown is `••••••••` with no revealed characters

#### Scenario: No key stored (unauthenticated)
- **WHEN** no API key is in the keyring
- **THEN** no key hint is shown

#### Scenario: Hint placement in the account section
- **WHEN** the user opens Settings and is authenticated
- **THEN** the key hint appears below the email address (or below the "DriveThruRPG Account" fallback label) in the identity section, in a tertiary text color

#### Scenario: Hint survives avatar state changes
- **WHEN** the Gravatar avatar fetch completes after sign-in
- **THEN** the key hint remains visible and unchanged
