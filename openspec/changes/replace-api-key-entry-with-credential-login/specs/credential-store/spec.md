## ADDED Requirements

### Requirement: Stored credential includes account email
`KeyringCredentialStore` SHALL persist the account email alongside the
application key under the existing `com.pilgrimagesoftware.dtrpg` / `api-key`
keychain entry, so the entry can be tied back to an email for reauthentication
and support/debugging.

#### Scenario: Email stored alongside key on sign-in
- **WHEN** the user successfully signs in with email and password
- **THEN** the keychain entry under `api-key` contains both the resulting
  application key and the signed-in email

#### Scenario: Email available on load
- **WHEN** a credential entry with an email is loaded at startup
- **THEN** the loaded credential exposes the email so the UI can pre-fill it
  on reauthentication

### Requirement: Legacy key-only entries remain valid
`KeyringCredentialStore::load` SHALL tolerate existing entries that contain
only an application key with no email, returning the credential with no email
rather than failing to load.

#### Scenario: Legacy entry loads without an email
- **WHEN** a pre-existing keychain entry written before this change contains
  only an application key
- **THEN** loading the credential succeeds and returns no email, and the
  application key is still used for silent reauthentication

#### Scenario: Legacy entry stops working only when the key is rejected
- **WHEN** a legacy key-only entry's application key is rejected by the
  existing authentication call
- **THEN** the app prompts the user to sign in again with email and password,
  rather than treating the missing email as an immediate failure
