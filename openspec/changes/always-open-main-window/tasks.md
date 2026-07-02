## 1. ServiceFactory — accept optional tokens

- [x] 1.1 Change `ServiceFactory` in `ui/app/mod.rs` from `Fn(LoginTokens)` to `Fn(Option<LoginTokens>)`
- [x] 1.2 Update `open_library_window` signature to `tokens: Option<LoginTokens>`; pass it to the factory
- [x] 1.3 Update the factory closure in `dtrpg-core/src/app/mod.rs` to accept `Option<LoginTokens>` and call `from_keyring_with_tokens(tokens)` when `Some`, or create a `UnavailableSdkGateway` variant when `None`
- [x] 1.4 Add a named constructor `RustSdkLibraryService::unauthenticated()` in `sdk.rs` that wraps `UnavailableSdkGateway` with a "not signed in" message (kind: `Session`)
- [x] 1.5 Update `LibraryRootView::new` — remove the `set_logged_in(None, cx)` unconditional call; it is now driven by the startup auth result via `AuthStateController`
- [x] 1.6 Run `cargo check --all-targets` — no errors

## 2. Startup routing — always open main window

- [x] 2.1 In `setup()` in `ui/app/mod.rs`: on successful re-auth, call `open_library_window(Some(tokens), cx)` with `AuthState::Authenticated`
- [x] 2.2 In `setup()`: on failed re-auth (bad key), call `open_library_window(None, cx)` with `AuthState::Unauthenticated` — remove the `open_login_window` call
- [x] 2.3 In `setup()`: when no API key is found, call `open_library_window(None, cx)` with `AuthState::Unauthenticated` — remove the `open_login_window` call
- [x] 2.4 Thread the `AuthState` into `open_library_window` (add `auth_state: AuthState` parameter) so `root_view.rs` can initialize `AuthStateController` with the correct state
- [x] 2.5 In `LibraryRootView::new`: initialize `AuthStateController` from the passed `auth_state` instead of the current env-var/debug branch
- [x] 2.6 In `LibraryRootView::new`: call `settings.set_logged_in(None, cx)` only when `auth_state == AuthState::Authenticated`
- [x] 2.7 Run `cargo check --all-targets` — no errors

## 3. Remove login window and LoginController

- [x] 3.1 Delete `crates/dtrpg-ui/src/ui/windows/login.rs`
- [x] 3.2 Delete `crates/dtrpg-ui/src/ui/views/login_view.rs`
- [x] 3.3 Delete `crates/dtrpg-ui/src/controllers/login.rs`
- [x] 3.4 Remove `pub mod login` from `controllers/mod.rs`; remove `login_view` from `ui/views/mod.rs`; remove `login` from `ui/windows/mod.rs`
- [x] 3.5 Remove `use crate::ui::windows::login::open_login_window` import from `root_view.rs` and `app/mod.rs`
- [x] 3.6 Remove `LoginStateChanged` from `data/events.rs`; remove `LoginServiceFactory` global from `ui/app/mod.rs` and `dtrpg-core/src/app/mod.rs` (login service is still used by settings, so keep the trait — just remove the global factory)
- [x] 3.7 Run `cargo check --all-targets` — no errors

## 4. Sign-in in Settings Account tab

- [x] 4.1 Add `api_key_draft: String`, `sign_in_in_progress: bool`, `sign_in_error: Option<String>` fields to `SettingsController`
- [x] 4.2 Add `LoginService` (Arc) to `SettingsController`; update `SettingsController::new` to accept `Box<dyn LoginService>` and wire through from `LibraryRootView::new`
- [x] 4.3 Add `SettingsController::set_api_key_draft(value: String, cx)`, `sign_in(cx)`, and expose them via snapshot (`api_key_draft`, `sign_in_in_progress`, `sign_in_error`)
- [x] 4.4 `sign_in(cx)` calls `login_service.authenticate(&api_key_draft)` on a background executor; on success: stores API key to keyring, calls `self.set_logged_in(None, cx)`, emits `SettingsChanged`, and fires a new `SignInSucceeded(LoginTokens)` event so `root_view` can update `AuthStateController` and the `LibraryService`
- [x] 4.5 Add `SignInSucceeded(LoginTokens)` to `data/events.rs`; emit from `SettingsController`; subscribe in `root_view.rs` to transition `AuthStateController` to `Authenticated`
- [x] 4.6 In `settings_view.rs` Account tab: when `!settings_snap.auth.is_logged_in`, render an API key input and Sign In button; when `sign_in_in_progress`, disable input and show loading; when `sign_in_error.is_some()`, show error text
- [x] 4.7 Run `cargo check --all-targets` — no errors

## 5. Avatar menu — sign-in action when unauthenticated

- [x] 5.1 In `toolbar_view.rs` `render_avatar_button`: when `!auth.is_logged_in`, the existing ghost button gains an `on_click` that opens settings to the Account tab (instead of just a tooltip)
- [x] 5.2 Make the ghost button open a dropdown (matching the authenticated case) with a single "Sign In…" item that triggers `settings.update(cx, |ctrl, cx| ctrl.open_to(SettingsTab::Account, cx))`
- [x] 5.3 Add `SettingsController::open_to(tab: SettingsTab, cx)` that sets `is_open = true`, `active_tab = tab`, and emits `SettingsChanged`
- [x] 5.4 Run `cargo check --all-targets` — no errors

## 6. Wiring the service refresh on sign-in

- [x] 6.1 In `root_view.rs`, subscribe to `SignInSucceeded(tokens)` from `settings` entity; call a new `LibraryController::replace_service(service, cx)` with a freshly constructed service built from the new tokens
- [x] 6.2 Add `LibraryController::replace_service(service: Box<dyn LibraryService>, cx)` that replaces the inner service and triggers a reload
- [x] 6.3 Run `cargo check --all-targets` — no errors

## 7. Verification

- [x] 7.1 Run `cargo check --all-targets` — no compile errors
- [x] 7.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings
- [x] 7.3 Run `cargo test --all-features --workspace` — all tests pass
- [x] 7.4 Launch with no stored API key: main window opens; banner shows; catalog is empty (not an error)
- [x] 7.5 From Settings > Account: enter a valid API key and sign in; banner clears; catalog loads; avatar shows signed-in state
- [x] 7.6 Click avatar button while signed out: dropdown appears with "Sign In…" item; clicking opens Settings > Account
- [x] 7.7 Launch with a valid stored API key: window opens authenticated; banner absent; catalog loads normally
- [x] 7.8 Launch with a stored API key that fails re-auth: window opens unauthenticated; banner shown; no login window appears
