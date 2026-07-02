## ADDED Requirements

### Requirement: Optional email field in login form
The login form SHALL display an optional email input field below the API key field. The field SHALL be labeled to indicate it is optional and used only for the avatar. The email SHALL NOT be required to complete sign-in.

#### Scenario: Login succeeds without email
- **WHEN** the user submits the login form with an API key but no email
- **THEN** authentication succeeds and the library window opens with the initial-letter avatar fallback

#### Scenario: Login succeeds with email
- **WHEN** the user submits the login form with both an API key and an email address
- **THEN** authentication succeeds, the email is persisted, and the library window opens and begins loading the avatar

#### Scenario: Email field pre-populated from stored profile
- **WHEN** a previously stored email exists in the profile config
- **THEN** the email field is pre-populated with that value when the login form opens

### Requirement: Email persisted to local profile config
When the user provides an email at login, it SHALL be saved to `{config_dir}/dtrpg/profile.toml`. It SHALL be loaded from that file on subsequent launches.

#### Scenario: Email saved after login
- **WHEN** the user enters an email and successfully signs in
- **THEN** the email is written to the profile config file before the library window opens

#### Scenario: Empty email clears stored profile email
- **WHEN** the user clears the email field and successfully signs in
- **THEN** the previously stored email is removed from the profile config

### Requirement: Avatar loaded on login and startup
After successful authentication (both first-time login and startup re-auth), the app SHALL call `set_logged_in` with the stored email (if any) so that the avatar fetch is triggered.

#### Scenario: Avatar fetch triggered on successful login
- **WHEN** the user signs in and an email is stored in the profile config
- **THEN** `set_logged_in(email)` is called and the Gravatar fetch begins

#### Scenario: Avatar fetch triggered on startup re-auth
- **WHEN** the app starts and silently re-authenticates with a stored API key
- **THEN** if a profile email is stored, `set_logged_in(email)` is called and the avatar fetch begins

#### Scenario: No avatar fetch when no email stored
- **WHEN** the user authenticates but no email is in the profile config
- **THEN** the initial-letter fallback is shown and no network request is made
