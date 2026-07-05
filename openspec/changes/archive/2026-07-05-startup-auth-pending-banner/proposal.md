## Why

When the app has a stored API key, background re-authentication runs after the window opens. During the brief auth window, the notification banner shows "Not signed in" even though credentials exist and auth is in progress, which is misleading. This adds an `Authenticating` state so the banner reflects what is actually happening, then transitions to "Not signed in" only if auth fails.

## What Changes

- Add an `Authenticating` state to `AuthState` (alongside `Unauthenticated`, `Authenticated`, `SessionExpired`).
- `AuthStateController` gains an `Authenticating` notice kind displayed while startup auth is in flight.
- `startup_auth` in `SettingsController` transitions `AuthStateController` to `Authenticating` when it begins, then to `Authenticated` on success or `Unauthenticated` on failure.
- The notification banner renders a neutral "Signing in..." row for the `Authenticating` notice kind (no action button, no dismiss button).
- On auth failure the `Authenticating` notice is replaced by the existing `NotSignedIn` notice.

## Capabilities

### New Capabilities

- `startup-auth-pending`: Auth-in-progress state surfaced in `AuthState` and `AuthStateController`; the notification banner renders a neutral "Signing in..." indicator while startup re-authentication is in flight.

### Modified Capabilities

(none)

## Impact

- `crates/dtrpg-ui/src/data/auth_state.rs` — new `AuthState::Authenticating` variant.
- `crates/dtrpg-ui/src/data/notification.rs` — new `NoticeKind::Authenticating`.
- `crates/dtrpg-ui/src/controllers/auth_state.rs` — handle `Authenticating` state in notice management.
- `crates/dtrpg-ui/src/controllers/settings.rs` — `startup_auth` emits new events to drive state transitions.
- `crates/dtrpg-ui/src/ui/views/notification_banner_view.rs` — render the `Authenticating` notice row.
- `crates/dtrpg-ui/src/data/events.rs` — new `StartupAuthBegun` event (or reuse `AuthStateChanged`).
