## Why

The avatar button's dropdown menu currently contains only a single "Log Out" action. There is no identity information visible, so the user has no way to confirm which account they are signed into without opening the full Settings panel. The email address is already stored in `AuthState::LoggedIn` but is not included in `AuthStateSnapshot` and therefore not reachable from the toolbar.

## What Changes

- Add `email: Option<String>` to `AuthStateSnapshot`, populated from `AuthState::LoggedIn { email, .. }` in `SettingsController::snapshot()`
- In `render_avatar_button`, add a non-interactive email label and a separator above the existing "Log Out" menu item

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- `rust-main-window-library-layout`: The avatar button menu SHALL display the signed-in account email as a non-interactive header above a separator, followed by the existing "Log Out" action

## Impact

- `controllers/settings.rs` (`AuthStateSnapshot`): add `pub email: Option<String>` field; update both `AuthState::LoggedOut` and `AuthState::LoggedIn` branches in `snapshot()`
- `ui/views/toolbar_view.rs` (`render_avatar_button`): clone `auth.email` into the dropdown closure; add `PopupMenuItem::label(email)` and `PopupMenuItem::separator()` before the "Log Out" item
- No controller logic changes, no data model changes
