## Why

The toolbar has no visible indicator of the current account state. Users have no at-a-glance way to know whether they are logged in or which account is active. An avatar button in the toolbar adds persistent identity context and provides a fast path to log out without opening the full Settings panel.

## What Changes

- **Toolbar avatar button**: A circular avatar image rendered to the right of the existing gear/settings button. When the user is authenticated it displays their Gravatar (fetched by MD5-hashing the account email address); when no Gravatar exists or the fetch fails, a generated fallback avatar (initials or identicon) is shown instead. When unauthenticated, a generic person icon placeholder is shown.
- **Logout popup**: Clicking the avatar when authenticated shows a small popover menu with a single "Log Out" action. Clicking the avatar when unauthenticated performs no action (or shows a disabled/empty state).
- **Avatar state in `SettingsController`**: The controller gains knowledge of the current auth state (logged-in email / logged-out) so it can supply avatar data to the toolbar; actual credential retrieval remains stubbed until `secure-credential-storage` is connected.
- **Gravatar fetch**: The app fetches the Gravatar image URL (`https://www.gravatar.com/avatar/<md5>?d=mp&s=64`) over HTTP on a background task when the email is known; the result is cached in memory for the session.

## Capabilities

### New Capabilities

- `toolbar-avatar-display`: The toolbar renders a circular avatar image reflecting the current account's Gravatar or a generated fallback.
- `avatar-logout-action`: Clicking the avatar while authenticated presents a logout option; clicking while unauthenticated is a no-op.

### Modified Capabilities

## Impact

- **`dtrpg-ui/src/ui/views/toolbar_view.rs`**: Add `render_avatar_button()` function; insert it after `render_settings_button()` in the controls row.
- **`dtrpg-ui/src/controllers/settings.rs`**: Add `auth_state: AuthState` field (`enum AuthState { LoggedIn { email: String, avatar_bytes: Option<Arc<Vec<u8>>> }, LoggedOut }`); expose `set_auth_state()`, `request_gravatar()`, and `logout()` methods; include auth state in `SettingsSnapshot`.
- **`dtrpg-ui/src/data/avatar.rs`** (new): MD5 hashing of email, Gravatar URL construction, fetch logic using `reqwest` (already a transitive dep via the SDK, or added explicitly).
- **`dtrpg-ui/Cargo.toml`**: Add `md5 = "0.7"` (or use `format!` with a sha/hex helper); add `reqwest` with `blocking = false` if not already present.
- No changes to the settings panel UI itself.
- Auth state is stubbed (hardcoded `LoggedOut` or a test email) until `secure-credential-storage` lands.
