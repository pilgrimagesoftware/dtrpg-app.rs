## ADDED Requirements

### Requirement: Account section shows avatar when signed in
When the user is signed in and avatar bytes have been fetched, the Account tab in the settings panel SHALL display the avatar image in a circular crop at the top of the authenticated section.

#### Scenario: Avatar image shown when bytes are available
- **WHEN** the user is signed in and Gravatar image bytes are present in the auth state
- **THEN** the account section displays a circular avatar image using those bytes

#### Scenario: Initial-letter fallback when avatar bytes are absent
- **WHEN** the user is signed in but no Gravatar bytes are available (fetch in flight, failed, or email not entered)
- **THEN** the account section displays a circular placeholder containing the first letter of the email address in uppercase

#### Scenario: Avatar area not shown when signed out
- **WHEN** the user is not signed in
- **THEN** the account section does not display an avatar circle or initial-letter placeholder

### Requirement: Account section shows email address when signed in
When the user is signed in, the Account tab in the settings panel SHALL display the signed-in email address below the avatar.

#### Scenario: Email displayed when available
- **WHEN** the user is signed in with an email address on record
- **THEN** the account section displays the email address as text below the avatar circle

#### Scenario: Email area not shown when signed out
- **WHEN** the user is not signed in
- **THEN** the account section does not display an email address

#### Scenario: No email available hides email row
- **WHEN** the user is signed in but no email address was entered (email is absent from the profile)
- **THEN** the account section does not display an email row (the avatar initial fallback uses "?" in this case)
