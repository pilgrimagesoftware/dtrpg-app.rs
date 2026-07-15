## Why

The Account tab in the settings panel shows only "Signed in to DriveThruRPG" — no email address, no avatar. Users have no way to confirm which account they are signed in with, and the avatar bytes fetched from Gravatar are never actually rendered anywhere in the UI.

## What Changes

- `AuthStateSnapshot` gains an `email: Option<String>` field populated when the user is signed in, so the settings view can display it without accessing the controller directly.
- `render_account_section` accepts `&AuthStateSnapshot` instead of just `is_authenticated: bool`, giving it the email, avatar bytes, and display initial.
- The authenticated branch of `render_account_section` displays:
  - A circular avatar — the Gravatar image if bytes are available, otherwise the first-letter initial on a colored background.
  - The account email address below the avatar.
  - The existing divider and logout button, unchanged.
- `render_settings_panel` and `render_active_section` are updated to pass `&AuthStateSnapshot` down to the account section instead of the bare `is_authenticated` bool.
- No changes to `SettingsController` business logic, fetch behavior, or credential storage.

## Capabilities

### New Capabilities

- `settings-account-user-info`: The account section of the settings panel displays the signed-in user's avatar image (or initial fallback) and email address.

### Modified Capabilities

<!-- none -->

## Impact

- `dtrpg-ui/src/controllers/settings.rs` — add `email: Option<String>` to `AuthStateSnapshot`; populate in `snapshot()`
- `dtrpg-ui/src/ui/views/settings_account_view.rs` — update `render_account_section` signature and authenticated render branch
- `dtrpg-ui/src/ui/views/settings_view.rs` — thread `&AuthStateSnapshot` through `render_settings_panel` and `render_active_section`; remove the now-redundant `is_authenticated` parameter
- No changes to service layer, SDK, or credential storage
