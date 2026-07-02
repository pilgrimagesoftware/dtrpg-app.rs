# auth-state-init Specification

## Purpose
TBD - created by archiving change fix-signed-in-banner. Update Purpose after archive.
## Requirements
### Requirement: Library window opens with authenticated state
When the library window is opened, the `AuthStateController` SHALL be initialized with `AuthState::Authenticated`. No "not signed in" or "session expired" notice SHALL appear at startup.

#### Scenario: No banner on library window open
- **WHEN** the library window is opened after successful authentication
- **THEN** the notification banner area is empty (no "not signed in" notice is shown)

#### Scenario: Banner appears only after explicit state transition
- **WHEN** `AuthStateController::set_state` is called with `AuthState::Unauthenticated`
- **THEN** the "not signed in" notice becomes visible

