## 1. Extend AuthStateSnapshot with email

- [x] 1.1 Add `pub email: Option<String>` field to `AuthStateSnapshot` in `controllers/settings.rs`
- [x] 1.2 In `SettingsController::snapshot()`, populate `email` from `AuthState::LoggedIn { email, .. }` as `Some(email.clone())` and `None` for `AuthState::LoggedOut`

## 2. Update settings panel call sites

- [x] 2.1 In `settings_view.rs`, remove the `is_authenticated: bool` parameter from `render_settings_panel` and `render_active_section`; add `auth: &AuthStateSnapshot` in its place (import `AuthStateSnapshot` if not already imported)
- [x] 2.2 In `render_active_section`, pass `auth` instead of `is_authenticated` to `render_account_section`
- [x] 2.3 Update the call to `render_settings_panel` in `root_view.rs` (or wherever it is called from the snapshot): remove `snapshot.is_authenticated`, add `&snapshot.auth`

## 3. Rewrite render_account_section signature

- [x] 3.1 In `settings_account_view.rs`, change the `render_account_section` signature from `(is_authenticated: bool, entity: Entity<SettingsController>, colors: &ColorTokens)` to `(auth: &AuthStateSnapshot, entity: Entity<SettingsController>, colors: &ColorTokens)` and add the necessary `use` import for `AuthStateSnapshot`
- [x] 3.2 Update the branch condition from `if is_authenticated` to `if auth.is_logged_in`; pass `auth` into `render_authenticated`

## 4. Implement the avatar + email identity row

- [x] 4.1 Update `render_authenticated` to accept `auth: &AuthStateSnapshot` in addition to (or replacing use of) entity and colors
- [x] 4.2 Implement `fn render_avatar_circle(auth: &AuthStateSnapshot, colors: &ColorTokens) -> AnyElement` that:
  - if `auth.avatar_bytes` is `Some(bytes)`: attempt to construct `gpui::ImageData` from the bytes; if successful render an `img(ImageSource::Data(...))` element sized 56×56 px inside a `div().size(px(56.0)).rounded_full().overflow_hidden()`
  - if bytes are absent or decoding fails: render a `div().size(px(56.0)).rounded_full().bg(colors.accent_soft).flex().items_center().justify_center()` containing the `display_initial` character (or "?" if `None`) as `text_xl` in `colors.accent`
- [x] 4.3 Check the GPUI source at `.cargo/git/checkouts/zed-*/crates/gpui/src/image.rs` (or `image_cache.rs`) to confirm the exact API for constructing `ImageData` from raw bytes; adapt 4.2 if the API differs (e.g., `ImageData::from_encoded_bytes`, `gpui::ImageSource::Bytes`, etc.)
- [x] 4.4 In `render_authenticated`, replace the existing "Account" / "Signed in to DriveThruRPG" identity row with a new horizontal flex row containing:
  - `render_avatar_circle(auth, colors)` on the left
  - A vertical flex column on the right with: a semibold "Account" label (text_sm, text_primary) and, if `auth.email.is_some()`, the email string (text_sm, text_secondary)

## 5. Verify

- [x] 5.1 Run `cargo check --all-targets` and confirm no compile errors
- [x] 5.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any warnings
- [x] 5.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 5.4 Manually launch the app in debug mode (with the `set_logged_in` stub active) and open Settings → Account; confirm the avatar circle with an initial letter and the email address are visible
- [ ] 5.5 If `avatar-load-and-cache` is also applied: confirm that after a successful Gravatar fetch the image renders in place of the initial letter
- [ ] 5.6 Sign out and reopen Settings → Account; confirm the avatar circle and email are absent and the "not signed in" prompt is shown
