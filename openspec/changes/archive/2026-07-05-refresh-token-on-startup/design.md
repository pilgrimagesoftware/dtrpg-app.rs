## Context

`setup()` in `ui/app/mod.rs` currently routes to the library or login window based solely on
whether an API key is present in the keyring. If an API key exists, the library window opens
and `RustSdkLibraryService::from_keyring()` reads the stored access token — which may be
expired.

The `LoginService` trait already encapsulates the `POST /auth_key` call and token storage
logic. `SdkLoginService::authenticate()` is synchronous (uses `block_on` internally), so it
can be called directly inside the GPUI app setup closure without spawning an additional task.

## Goals / Non-Goals

**Goals:**
- Guarantee the access token is fresh on every startup when an API key is present.
- Fall back gracefully to the login window on auth failure, pre-filling the key.

**Non-Goals:**
- Background token refresh while the app is running (separate concern).
- Changing the `LoginService` trait signature.
- Adding a loading screen for the silent auth (it is fast enough to be invisible).

## Decisions

### Decision: Silent auth runs synchronously inside `setup()`

`setup()` runs in the GPUI app closure before any windows are opened. Calling a blocking
`LoginService::authenticate()` here is safe — no UI is visible yet, so there is no freeze
perceived by the user. A background task would add complexity for no UX benefit.

**Alternatives considered:**
- Show a brief "Signing in…" splash window → unnecessary complexity; auth is typically
  sub-second on a normal connection.
- Reuse `LoginController` and its background spawn → `LoginController` is designed for the
  interactive form flow and emits UI events; adapting it for silent use would couple concerns.

### Decision: Use `LoginServiceFactory` already set as a GPUI global

`setup()` can call `(cx.global::<LoginServiceFactory>().0)()` to build the login service,
consistent with how `open_login_window` obtains it. No new globals or parameters needed.

### Decision: Token storage on success mirrors the login flow exactly

Store the new `access_token` and `refresh_token` in the keyring using the same keys
(`keys::ACCESS_TOKEN`, `keys::REFRESH_TOKEN`) as `LoginController::submit()`. This keeps a
single source of truth for how credentials are persisted.

### Decision: Add `prefilled_key: Option<String>` parameter to `open_login_window`

On auth failure, the login window opens with the API key field pre-filled. The cleanest way
to pass the key is as a parameter to `open_login_window`, which passes it to
`LoginController`. Add a `set_api_key_draft(key)` method (or pass via constructor) so the
draft is populated before the view renders.

**Alternatives considered:**
- Another GPUI global to carry the pre-fill key → adds state that lives longer than needed.
- Modifying `LoginController::new` to accept `Option<String>` → simpler than a setter, chosen.

## Risks / Trade-offs

- [Silent auth adds ~200–500 ms to cold startup on a good connection] → Acceptable; without
  it the app opens and immediately shows an auth error in the activity panel.
- [Silent auth fails on first offline launch with a previously valid key] → Falls back to
  login window with key pre-filled. User sees login screen on offline start.
  → Mitigation: acceptable trade-off; the stored token would be useless anyway.

## Migration Plan

1. Update `LoginController::new` to accept `Option<String>` as initial draft key.
2. Update `open_login_window` to accept `prefilled_key: Option<String>` and pass it through.
3. Update `setup()` to perform silent auth and call the updated `open_login_window` on failure.
4. Update call site in `root_view.rs` (logout path) to pass `None`.
