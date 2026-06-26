## 1. Dependencies

- [ ] 1.1 Add `md5 = "0.7"` to `[workspace.dependencies]` in `dtrpg-app/rust/Cargo.toml` and `md5 = { workspace = true }` to `dtrpg-ui/Cargo.toml`
- [ ] 1.2 Confirm `reqwest` is in the workspace (check `Cargo.toml`); if absent, add `reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }` to workspace deps and `reqwest = { workspace = true }` to `dtrpg-ui/Cargo.toml`
- [ ] 1.3 Run `cargo check -p dtrpg-ui` to confirm the dependency additions compile

## 2. Auth State and Avatar Data Model

- [ ] 2.1 Create `dtrpg-ui/src/data/avatar.rs` with `gravatar_url(email: &str) -> String` (MD5-hashes the trimmed, lowercased email; returns `https://www.gravatar.com/avatar/<hash>?d=404&s=64`) and `fetch_avatar_bytes(email: &str) -> Option<Vec<u8>>` (async; GET with 5-second timeout; returns `None` on non-200 or error)
- [ ] 2.2 Register `pub mod avatar;` in `data/mod.rs`
- [ ] 2.3 Add `AuthState` enum to `controllers/settings.rs`:
  ```
  pub enum AuthState {
      LoggedOut,
      LoggedIn { email: String, avatar_bytes: Option<Arc<Vec<u8>>> },
  }
  ```
- [ ] 2.4 Add `auth_state: AuthState` field to `SettingsController`; initialize to `AuthState::LoggedOut` in `new()`
- [ ] 2.5 Add `pub fn set_logged_in(&mut self, email: String, cx: &mut Context<Self>)` — sets `auth_state` to `LoggedIn` with `avatar_bytes: None`, emits `SettingsChanged`, then spawns `cx.background_executor().spawn(fetch_avatar_bytes(email.clone()))` and on completion calls `ctrl.set_avatar_bytes(bytes, cx)`
- [ ] 2.6 Add `pub fn set_avatar_bytes(&mut self, bytes: Option<Vec<u8>>, cx: &mut Context<Self>)` — if `LoggedIn`, wraps bytes in `Arc` and stores; emits `SettingsChanged`
- [ ] 2.7 Add `pub fn logout(&mut self, cx: &mut Context<Self>)` — sets `auth_state` to `LoggedOut`; emits `SettingsChanged`
- [ ] 2.8 Add `AuthStateSnapshot { is_logged_in: bool, display_initial: Option<char>, avatar_bytes: Option<Arc<Vec<u8>>> }` struct; populate it in `SettingsController::snapshot()` from `auth_state`
- [ ] 2.9 Add `auth: AuthStateSnapshot` to `SettingsSnapshot`
- [ ] 2.10 Run `cargo check -p dtrpg-ui` and confirm zero errors

## 3. Toolbar Avatar Button

- [ ] 3.1 In `toolbar_view.rs`, add `render_avatar_button(auth: &AuthStateSnapshot, settings: Entity<SettingsController>, colors: &ColorTokens) -> impl IntoElement` function
- [ ] 3.2 Implement the unauthenticated state: a 32×32 circle with `surface_alt` background and `👤` text centered in it; `on_click` is a no-op (no handler registered so the click does nothing)
- [ ] 3.3 Implement the logged-in + no avatar state: same circle with `accent` background and the `display_initial` char as white text
- [ ] 3.4 Implement the logged-in + avatar bytes state: render a gpui `img()` element (or `ImageData`-backed image) clipped to a circle via `.rounded_full()` and sized to 32×32
- [ ] 3.5 When `is_logged_in`, attach a `DropdownMenu` (or `Popover`) to the avatar div using gpui-component's `PopupMenuItem` pattern; add a single "Log Out" item whose `on_click` calls `settings.update(cx, |ctrl, cx| ctrl.logout(cx))`
- [ ] 3.6 Insert `render_avatar_button(&snap.auth, settings.clone(), colors)` into the controls row in `render_toolbar()`, after `render_settings_button()`; thread `auth: &AuthStateSnapshot` through from the caller or capture from the snapshot
- [ ] 3.7 Update `render_toolbar()` signature to accept the auth snapshot (or read it from the settings snapshot already passed)
- [ ] 3.8 Run `cargo check -p dtrpg-ui` and confirm zero errors

## 4. Stub and Verification

- [ ] 4.1 In `root_view.rs` (or wherever `SettingsController::new()` is called), temporarily call `settings.update(cx, |ctrl, cx| ctrl.set_logged_in("test@example.com".into(), cx))` after construction so the avatar renders with a real Gravatar fetch during development
- [ ] 4.2 Build and run the app; confirm the avatar button appears to the right of the gear button
- [ ] 4.3 Confirm the initials fallback renders immediately (before the fetch completes) and the Gravatar image replaces it once the background fetch returns
- [ ] 4.4 Click the avatar; confirm the logout popover appears with a "Log Out" item
- [ ] 4.5 Click "Log Out"; confirm the avatar reverts to the generic person icon and the popover closes
- [ ] 4.6 Confirm clicking the generic (logged-out) avatar does not open a popover
- [ ] 4.7 Remove or gate the stub `set_logged_in` call behind a `#[cfg(debug_assertions)]` block so it doesn't ship in release builds
