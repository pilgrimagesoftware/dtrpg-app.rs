## Why

The app currently opens the library window using whatever access token was last stored in the
keyring. JWT access tokens are short-lived; if the app has not been used recently the stored
token is likely expired, causing immediate 401 failures when the library tries to load. The
user has to discover this through an opaque error rather than being prompted to re-authenticate.

Refreshing the token on every startup ensures the session is always valid before the library
window opens, at the cost of one additional network round-trip.

## What Changes

- When a stored API key is found on startup, the app SHALL call the auth endpoint to acquire
  a fresh access token before opening the library window.
- The new access token SHALL be stored in the keyring, replacing the previous one.
- If the auth call fails (network error, invalid key), the app SHALL clear the stored access
  token and fall back to the login window, pre-populating the API key field if possible.
- If no API key is stored, the existing behavior (open login window) is unchanged.

## Capabilities

### New Capabilities

- `silent-startup-reauth`: On startup with a stored API key, the app silently re-authenticates
  and stores the fresh token before opening the library window.

### Modified Capabilities

## Impact

- `dtrpg-ui`: `ui/app/mod.rs` — `setup()` function: add silent-auth step between API key
  check and `open_library_window()`.
- `dtrpg-ui`: `ui/windows/login.rs` — may need a way to pre-populate the API key draft when
  falling back from a failed silent auth.
- No changes to the SDK, `LoginService` trait, or keyring credential model.
