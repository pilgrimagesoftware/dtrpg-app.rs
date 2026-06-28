## Context

The app currently routes to a standalone login window when no valid session exists. `setup()` in `ui/app/mod.rs` either opens the library window (with `LoginTokens`) or the login window (with no tokens). The library window requires `LoginTokens` to construct the `LibraryService`. `open_library_window` takes `LoginTokens` by value; the `ServiceFactory` global is `Fn(LoginTokens) -> Box<dyn LibraryService>`.

The login UI lives in `ui/windows/login.rs` + `ui/views/login_view.rs` + `controllers/login.rs`. When login succeeds, the login window closes and the library window opens. This means there is always exactly one window visible: login or library, never both.

Auth-adjacent infrastructure already in place that this change builds on:
- `AuthState` enum (`Unauthenticated` / `Authenticated` / `SessionExpired`) drives the notification banner via `AuthStateController`.
- `SettingsController` has an Account tab that is currently a placeholder.
- The notification banner (`NoticeAction::OpenSettings(SettingsTab::Account)`) already points unauthenticated users toward settings.

## Goals / Non-Goals

**Goals:**
- Library window opens unconditionally at startup.
- Sign-in is an action the user takes from the existing notification banner or avatar menu, not a forced gate.
- API key entry lives in the Account tab of the settings panel, not a separate window.
- Auth state (authenticated / not) is always visible in the UI.
- `LoginController` and the standalone login window/view are removed.

**Non-Goals:**
- Local content caching/persistence (offline mode returns an empty catalog; no local DB is introduced here).
- Async background authentication at startup (re-auth remains synchronous but failure no longer blocks the window).
- Token refresh near expiry (tracked separately).

## Decisions

### 1. `open_library_window` accepts `Option<LoginTokens>`

`ServiceFactory` changes to `Fn(Option<LoginTokens>) -> Box<dyn LibraryService>`. When `None`, the SDK gateway returns an offline/empty service (existing `UnavailableSdkGateway` can be reused with a benign "not signed in" error message). When `Some(tokens)`, the real SDK gateway is constructed as today.

**Alternative considered**: Async startup auth (window opens first, auth result arrives later via event). Rejected for now — adds significant complexity; the synchronous blocking startup auth is fast enough in practice.

**Alternative considered**: Require `LoginTokens` on window open but allow re-authentication inside the window. Rejected — this still hides the window until initial auth succeeds.

### 2. `setup()` always opens the library window

`setup()` attempts silent re-auth if an API key exists. Success → `open_library_window(Some(tokens), cx)`. Failure or no key → `open_library_window(None, cx)`. `open_login_window` call sites are eliminated.

### 3. Sign-in moves to the Settings Account tab

`SettingsController` gains sign-in fields and logic (API key draft, in-progress state, error message). `settings_view.rs` renders sign-in controls in the Account tab when `auth_state.is_logged_in == false`. On successful sign-in, `SettingsController` emits `SettingsChanged` and triggers `AuthStateController` to transition to `Authenticated`.

**Alternative considered**: In-app sheet/modal over the library window. More effort to build; the Settings panel already exists and is already linked from the banner. Deferred.

### 4. `LoginController`, `LoginView`, and `open_login_window` are removed

Their functionality is absorbed by `SettingsController` (auth logic) and the settings Account tab view. `LoginStateChanged` event enum is removed; auth-outcome events go through `SettingsChanged` and the `AuthStateController`.

### 5. `AuthStateController` initial state comes from startup auth result

`root_view.rs` uses the startup auth result to set the initial `AuthState` passed to `AuthStateController::new`. Previously this was read from an env var override (debug builds only) or defaulted to `Authenticated`. Now `Unauthenticated` is the explicit result when startup auth fails or no key is present.

## Risks / Trade-offs

- **Startup latency visible**: if re-auth is slow, the window opens but the catalog shows a loading/empty state longer than before. → Acceptable; the window is at least present and responsive.
- **UnavailableSdkGateway as offline service**: The error messages shown when no tokens are present need to be "not signed in" not "network error". → The `UnavailableSdkGateway` error string should be updated, or a dedicated `UnauthenticatedGateway` added.
- **Settings Account tab UX debt**: Moving sign-in to a settings tab is not the most discoverable UX. → The notification banner and avatar menu both drive users there. An in-app sheet can replace this later without a spec change.
- **`LoginTokens` event removal**: Any test stubs that emit `LoginStateChanged::Succeeded(tokens)` need updating. → Tests are currently in `controllers/login.rs` and are removed with `LoginController`.

## Open Questions

- Should the Account tab show a password-manager-style "save API key" toggle in the future, or is keychain storage always implicit?  (No decision needed now — keychain is always used.)
- When sign-in from settings fails, should the banner update its action text to "Try again"? (Deferred; current error display in the Account tab is sufficient.)
