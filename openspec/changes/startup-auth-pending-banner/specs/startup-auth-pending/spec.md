## ADDED Requirements

### Requirement: Auth-pending notice while startup authentication is in flight
When the app starts and background re-authentication begins, `AuthStateController` SHALL display a neutral "Signing in..." notice in place of the "Not signed in" notice for the duration of the in-flight request.

#### Scenario: Pending notice appears when startup auth begins
- **WHEN** the app launches with a stored API key and `startup_auth` is called
- **THEN** the notification banner shows a "Signing in..." row with no action button and no dismiss button

#### Scenario: Pending notice removed on successful auth
- **WHEN** startup re-authentication succeeds
- **THEN** the "Signing in..." notice is gone and no notification banner is visible

#### Scenario: Not-signed-in notice shown on auth failure
- **WHEN** startup re-authentication fails (e.g. invalid key, network error)
- **THEN** the "Signing in..." notice is replaced by the standard "Not signed in to DriveThruRPG" notice with its "Set Up Account" action button

### Requirement: Auth-pending notice is not dismissible
The "Signing in..." notice SHALL have no dismiss control and no primary action button. The user cannot interact with it.

#### Scenario: No dismiss button on pending notice
- **WHEN** the notification banner renders the `Authenticating` notice kind
- **THEN** neither a dismiss button nor an action button is rendered for that row

### Requirement: `AuthStateController.is_auth_pending` drives notice substitution
`AuthStateController` SHALL expose `set_auth_pending(bool, cx)`. When `is_auth_pending` is `true`, `active_notices()` SHALL return an `Authenticating` notice instead of any `NotSignedIn` notice that would otherwise be derived from the underlying `AuthState`.

#### Scenario: Pending flag suppresses NotSignedIn notice
- **WHEN** `auth_state` is `Unauthenticated` and `is_auth_pending` is `true`
- **THEN** `active_notices()` returns one notice with `kind == NoticeKind::Authenticating`

#### Scenario: Clearing pending flag restores normal notice derivation
- **WHEN** `is_auth_pending` is set to `false` while `auth_state` is `Unauthenticated`
- **THEN** `active_notices()` returns one notice with `kind == NoticeKind::NotSignedIn`

#### Scenario: Setting state clears the pending flag
- **WHEN** `set_state(AuthState::Authenticated, cx)` is called while `is_auth_pending` is `true`
- **THEN** `is_auth_pending` becomes `false` and `active_notices()` is empty
