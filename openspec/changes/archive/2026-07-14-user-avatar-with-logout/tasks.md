## 1. Dependencies

- [x] 1.1 Add `md5 = "0.7"` to `[workspace.dependencies]` in `dtrpg-app/rust/Cargo.toml` and `md5 = { workspace = true }` to `dtrpg-ui/Cargo.toml`
- [x] 1.2 Confirm `reqwest` is in the workspace (check `Cargo.toml`); if absent, add `reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }` to workspace deps and `reqwest = { workspace = true }` to `dtrpg-ui/Cargo.toml`
- [x] 1.3 Run `cargo check -p dtrpg-ui` to confirm the dependency additions compile

## 2. Auth State and Avatar Data Model

- [x] 2.1 Create `dtrpg-ui/src/data/avatar.rs` with `gravatar_url(email: &str) -> String` (MD5-hashes the trimmed, lowercased email; returns `https://www.gravatar.com/avatar/<hash>?d=404&s=64`) and `fetch_avatar_bytes(email: String) -> Option<Vec<u8>>` (async; GET with 5-second timeout; returns `None` on non-200 or error)
- [x] 2.2 Register `pub mod avatar;` in `data/mod.rs`
- [x] 2.3 Add `AuthState` enum to `controllers/settings.rs`:
  ```
  pub enum AuthState {
      LoggedOut,
      LoggedIn { email: String, avatar_bytes: Option<Arc<Vec<u8>>> },
  }
  ```
- [x] 2.4 Add `auth_state: AuthState` field to `SettingsController`; initialize to `AuthState::LoggedOut` in `new()`
- [x] 2.5 Add `pub fn set_logged_in(&mut self, email: String, cx: &mut Context<Self>)` — sets `auth_state` to `LoggedIn` with `avatar_bytes: None`, emits `SettingsChanged`, then spawns background avatar fetch and on completion calls `ctrl.set_avatar_bytes(bytes, cx)`
- [x] 2.6 Add `pub fn set_avatar_bytes(&mut self, bytes: Option<Vec<u8>>, cx: &mut Context<Self>)` — if `LoggedIn`, wraps bytes in `Arc` and stores; emits `SettingsChanged`
- [x] 2.7 Add `pub fn logout(&mut self, cx: &mut Context<Self>)` — sets `auth_state` to `LoggedOut`; emits `SettingsChanged` and `LogoutRequested`
- [x] 2.8 Add `AuthStateSnapshot { is_logged_in: bool, display_initial: Option<char>, avatar_bytes: Option<Arc<Vec<u8>>> }` struct; populate it in `SettingsController::snapshot()` from `auth_state`
- [x] 2.9 Add `auth: AuthStateSnapshot` to `SettingsSnapshot`
- [x] 2.10 Run `cargo check -p dtrpg-ui` and confirm zero errors

## 3. Avatar Button

Superseded by `shared-main-window-structure`: the account button was relocated from
the toolbar to the title bar (`title_bar_view.rs`), so the function name and call
site below differ from the original plan. The functional requirements are met.

- [x] 3.1 `render_account_button(auth: &AuthStateSnapshot, settings: Entity<SettingsController>, colors: &ColorTokens, cx: &App) -> AnyElement` in `title_bar_view.rs` (renamed/relocated from the planned `render_avatar_button` in `toolbar_view.rs`)
- [x] 3.2 Unauthenticated state: 28x28 circle with `surface_alt` background and a person emoji, dropdown offers "Sign In" (no bare no-op — resolves to the settings/login flow)
- [x] 3.3 Logged-in state: `Button` showing `display_initial` as a fallback label
- [x] 3.4 Avatar image bytes are rendered when present, via `img(ImageSource::Image(...))` with `rounded_full()` — not deferred to future work as originally planned
- [x] 3.5 `dropdown_menu()` on the logged-in button includes an email/account label item and a `title_bar.sign_out` item calling `settings.update(cx, |ctrl, cx| ctrl.logout(cx))`
- [x] 3.6 Inserted via `render_title_bar()` -> `render_account_button()`, called from `root_view.rs` in place of the toolbar controls row
- [x] 3.7 `render_title_bar(auth: &AuthStateSnapshot, ...)` in `title_bar_view.rs`; `root_view.rs` passes `&settings_snap.auth`
- [x] 3.8 `cargo check -p dtrpg-ui` passes with zero errors

## 4. Stub and Verification

- [x] 4.1 In `root_view.rs`, added stub `settings.update(cx, |ctrl, cx| ctrl.set_logged_in("test@example.com".into(), cx))` after construction, gated behind `#[cfg(debug_assertions)]`
- [x] 4.2 Build and run the app; confirm the avatar button appears to the right of the gear button
- [x] 4.3 Confirm the initials fallback renders immediately (before the fetch completes) and the Gravatar image replaces it once the background fetch returns
- [x] 4.4 Click the avatar; confirm the logout popover appears with a "Log Out" item
- [x] 4.5 Click "Log Out"; confirm the avatar reverts to the generic person icon and the popover closes
- [x] 4.6 Confirm clicking the generic (logged-out) avatar does not open a popover
- [x] 4.7 Remove or gate the stub `set_logged_in` call behind a `#[cfg(debug_assertions)]` block so it doesn't ship in release builds
