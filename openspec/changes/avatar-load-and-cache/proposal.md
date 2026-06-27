## Why

The toolbar avatar never shows a real image in production: `SettingsController::set_logged_in` (which triggers the Gravatar fetch) is called only by a debug stub, never from the actual login or startup flows. Additionally, the DTRPG API does not expose the user's email address, so there is no way to resolve a Gravatar without first collecting the email from the user. And even when bytes are fetched they live only in memory, requiring a new network request every launch.

## What Changes

- The login form gains an optional **Email** field. It is clearly labeled as optional and used only for the avatar; it has no effect on API authentication. The entered value is persisted to a local profile config file (not the keyring).
- On successful login or startup re-auth, the app reads the stored email (if any) and calls `set_logged_in(email, cx)` on the `SettingsController`, triggering the Gravatar fetch.
- `fetch_avatar_bytes` checks a local disk cache (`{cache_dir}/dtrpg/avatar`) before making a network request. If a valid cached file exists it is used directly.
- After a successful network fetch, the bytes are written to the disk cache for future launches.
- The debug stub in `root_view.rs` is removed; the production path now handles avatar loading correctly.

## Capabilities

### New Capabilities

- `avatar-email-profile`: Optional email input at login that is persisted to a local profile config file and used as the Gravatar identity.
- `avatar-disk-cache`: Avatar bytes are cached to disk and served from cache on subsequent launches, falling back to a fresh Gravatar fetch when the cache is missing or stale.

### Modified Capabilities

<!-- none — the existing Gravatar fetch logic in fetch_avatar_bytes and set_logged_in is extended, not replaced -->

## Impact

- `dtrpg-ui/src/ui/views/login_view.rs` — add optional email `InputState`; wire to a new `LoginController::set_email` method
- `dtrpg-ui/src/controllers/login.rs` — add `email_draft: Option<String>` field and `set_email` / `email()` accessors
- `dtrpg-ui/src/data/avatar.rs` — add `load_cached_avatar_bytes()` and `save_cached_avatar_bytes()` using `{cache_dir}/dtrpg/avatar`; update `fetch_avatar_bytes` to check cache first
- `dtrpg-ui/src/data/profile.rs` (new) — `ProfileConfig`: reads/writes `{config_dir}/dtrpg/profile.toml` with `email: Option<String>`
- `dtrpg-ui/src/ui/views/login_view.rs` — on `Succeeded`, forward stored email (from `LoginController::email()`) alongside the library window open
- `dtrpg-ui/src/ui/app/mod.rs` — after successful auth (both login and startup re-auth), read `ProfileConfig::email()` and call `settings.set_logged_in(email, cx)` on the `SettingsController` in `LibraryRootView`
- `dtrpg-ui/src/ui/views/root_view.rs` — remove debug-only `set_logged_in` stub
