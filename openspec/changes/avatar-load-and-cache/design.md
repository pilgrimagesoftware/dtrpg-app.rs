## Context

The DTRPG API authenticates via API key (`POST /auth_key?applicationKey=<key>`) and returns a JWT. The API does not expose the authenticated user's email address — it is private customer data not surfaced through any available endpoint. Without an email, the existing Gravatar URL computation (`gravatar_url(email)`) cannot produce a valid URL.

`SettingsController::set_logged_in(email, cx)` is the existing hook that triggers the avatar fetch. It is currently called only by a `#[cfg(debug_assertions)]` stub in `root_view.rs`. The production login and startup re-auth paths do not call it at all.

The `dirs` crate is already used for resolving `config_dir` and `data_dir`. `reqwest::blocking` is already used by `fetch_avatar_bytes`.

## Goals / Non-Goals

**Goals:**
- Collect the user's email once (at login, optional) and persist it locally.
- Trigger avatar loading in both the first-time login and startup re-auth paths.
- Cache the fetched Gravatar bytes to disk so subsequent launches skip the network round-trip.
- Remove the debug-only stub and replace it with the real production path.

**Non-Goals:**
- Fetching email from the DTRPG API (it is not available).
- Avatar cache expiry / background refresh (a future concern).
- Supporting avatar sources other than Gravatar.
- Storing the email in the keyring (it is not a credential).

## Decisions

### Email collected at login as an optional field

The login form gains a second, non-masked, optional `InputState` for the user's email. It is labeled clearly as optional and for avatar use only. On `Succeeded`, the email is forwarded through the `open_library_window` call.

**Why**: The email is the only practical input for Gravatar. The DTRPG API does not provide it. Making the field optional keeps friction low — users who skip it simply keep the initial-letter fallback.

**Alternative**: Derive a pseudo-avatar from the customer ID using a generated image service. Rejected because the existing Gravatar infrastructure already works; adding another image service is unnecessary complexity.

**Alternative**: Ask for email in Settings rather than at login. Rejected because the avatar would not appear until after the first trip to Settings, which is a worse first impression.

### Email persisted to `{config_dir}/dtrpg/profile.toml`

`data::profile::ProfileConfig` reads and writes a TOML file with a single optional `email` field. The `dirs::config_dir()` path is already resolved by the app.

**Why**: The email is user preference, not a credential. The keyring is for secrets only (`api-key`, `access-token`, `refresh-token`). A plain TOML file is consistent with `StorageConfig`.

### Avatar bytes cached at `{cache_dir}/dtrpg/avatar`

`fetch_avatar_bytes` is extended to check `dirs::cache_dir().join("dtrpg/avatar")` before making the Gravatar HTTP request. If the file exists and is non-empty, its bytes are returned directly. After a successful network fetch, the bytes are written to this path.

**Why**: `cache_dir` is the platform-conventional location for non-critical data that can be regenerated (macOS: `~/Library/Caches`). An untyped raw bytes file (no extension) is sufficient since the image format is embedded in the bytes and GPUI handles format detection.

**Alternative**: Cache in `data_dir` alongside library downloads. Rejected — the avatar is not user-generated content; it belongs in the system cache.

### `open_library_window` does not change signature

`open_library_window(cx)` creates `LibraryRootView` and therefore `SettingsController`. Rather than plumbing the email through the window-open call, the library root view reads `ProfileConfig::email()` directly during its own `new()` call and immediately calls `settings.set_logged_in(email, cx)` if an email is present.

**Why**: `open_library_window` is already used from two call sites (login `Succeeded` handler and `setup()`). Keeping its signature unchanged avoids touching both and keeps `app/mod.rs` clean.

### Debug stub removal

The `#[cfg(debug_assertions)]` stub in `root_view.rs` that calls `set_logged_in("test@example.com".into(), cx)` is removed. Developers can set an email via the login form or by writing `{config_dir}/dtrpg/profile.toml` manually.

## Risks / Trade-offs

- [Risk] User enters an email with no Gravatar account → Mitigation: `d=404` in the Gravatar URL returns HTTP 404; `fetch_avatar_bytes` returns `None` and the initial-letter fallback is shown. No change needed.
- [Risk] Disk cache contains stale or corrupted bytes → Mitigation: if `gpui` fails to render the cached bytes as an image, the fallback initial letter renders; a future cache-expiry pass can refresh the file.
- [Risk] `cache_dir` is unavailable (unusual on macOS/Linux/Windows) → Mitigation: read/write failures are logged as warnings; the avatar falls back to in-memory behavior as before.
- [Risk] Email stored in plaintext on disk → Mitigation: acceptable — the email is not a credential. Users who consider it sensitive can skip the field; the fallback avatar still renders.

## Open Questions

- Should the email field be pre-populated at login if `ProfileConfig` already has an email stored? → Yes, pre-populate it so the user can verify/change it without re-entering from scratch.
