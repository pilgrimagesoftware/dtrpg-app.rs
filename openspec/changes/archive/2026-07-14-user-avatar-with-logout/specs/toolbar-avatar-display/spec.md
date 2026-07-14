## ADDED Requirements

### Requirement: Toolbar displays a circular avatar button reflecting account state
The toolbar SHALL render a circular avatar button to the right of the settings gear button at all times. The button's visual content depends on the current authentication state.

#### Scenario: Avatar shown when user is logged in with a Gravatar
- **WHEN** the user is authenticated and a Gravatar image is available for their email address
- **THEN** the avatar button displays the Gravatar image cropped to a circle (32×32 px at standard resolution)

#### Scenario: Avatar shown when user is logged in but no Gravatar is found
- **WHEN** the user is authenticated but the Gravatar fetch returns the default fallback (404, `d=mp`, or network error)
- **THEN** the avatar button displays a generated initials avatar using the first character of the email address, rendered as white text on an accent-color circle

#### Scenario: Avatar shown when user is not logged in
- **WHEN** the user is not authenticated
- **THEN** the avatar button displays a generic person-silhouette icon (Unicode `👤` or equivalent SVG-based icon) on a muted surface circle

### Requirement: Gravatar is fetched asynchronously and cached for the session
The system SHALL fetch the Gravatar URL in the background when an authenticated email address becomes known. The result SHALL be cached in memory and SHALL NOT be re-fetched until the app restarts or the account changes.

#### Scenario: Gravatar fetch completes successfully
- **WHEN** the app has an authenticated email and the Gravatar HTTP request returns a 200 response with image data
- **THEN** the avatar button transitions from the fallback to the Gravatar image without requiring any user action

#### Scenario: Gravatar fetch fails or times out
- **WHEN** the Gravatar HTTP request returns a non-200 response, network error, or exceeds a 5-second timeout
- **THEN** the avatar button continues to show the generated fallback and no error is surfaced to the user
