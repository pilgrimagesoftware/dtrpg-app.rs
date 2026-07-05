# toolbar-avatar-display Specification

## Purpose
TBD - created by archiving change avatar-circular-image. Update Purpose after archive.
## Requirements

### Requirement: Toolbar displays a circular avatar button reflecting account state
The toolbar SHALL render a circular avatar button to the right of the settings gear button at all times. The button SHALL be 30×30 px and use `rounded_full()` in all authentication states. The button's visual content depends on the current authentication state.

#### Scenario: Avatar shown when user is logged in with a Gravatar
- **WHEN** the user is authenticated and `avatar_bytes` is `Some`
- **THEN** the avatar button displays the image clipped to a circle using a 30×30 px container with `rounded_full()`

#### Scenario: Avatar shown when user is logged in but no Gravatar is available
- **WHEN** the user is authenticated and `avatar_bytes` is `None`
- **THEN** the avatar button displays the user's initial as white text centered in an accent-color 30×30 px circle with `rounded_full()`

#### Scenario: Avatar shown when user is not logged in
- **WHEN** the user is not authenticated
- **THEN** the avatar button displays a generic person-silhouette icon (`👤`) centered in a 30×30 px `surface_alt` circle with `rounded_full()` and a `border_strong` border

#### Scenario: Authenticated avatar button opens logout menu on click
- **WHEN** the user clicks the avatar button while authenticated
- **THEN** a dropdown menu appears with a "Log Out" action

#### Scenario: Unauthenticated avatar button has no click action
- **WHEN** the user clicks the avatar button while not authenticated
- **THEN** no action is taken
