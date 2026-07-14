## Why

Users who have stored an API key have no visual confirmation of which key is in use; the settings Account section shows only "Signed in" with no key reference. Showing a masked hint (first 4 + last 1 characters) gives enough context to identify the key without exposing sensitive material.

## What Changes

- The authenticated Account section gains a read-only API key hint row below the email/label text, showing the key in the form `abcd••••••••••••1`.
- The hint is derived from the stored API key at the point authentication succeeds and is carried in the settings snapshot; no extra keyring read is needed at render time.

## Capabilities

### New Capabilities

- `api-key-hint-display`: Show a masked API key hint in the authenticated account view (first 4 chars + ellipsis + last 1 char).

### Modified Capabilities

## Impact

- **`dtrpg-ui/src/controllers/settings.rs`**: Store the masked key hint when `set_logged_in` is called; include it in `AuthStateSnapshot` and `SettingsSnapshot`.
- **`dtrpg-ui/src/ui/views/settings_account_view.rs`**: Render the hint as a tertiary-colored label beneath the email in the identity section.
- No API, SDK, credential storage, or dependency changes.
