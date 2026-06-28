## 1. AuthStateSnapshot — add email field

- [ ] 1.1 In `controllers/settings.rs`, add `pub email: Option<String>` to `AuthStateSnapshot`
- [ ] 1.2 In the `AuthState::LoggedOut` branch of `SettingsController::snapshot()`, set `email: None`
- [ ] 1.3 In the `AuthState::LoggedIn { email, avatar_bytes }` branch, set `email: Some(email.clone())`

## 2. Avatar dropdown — add email label and separator

- [ ] 2.1 In `toolbar_view.rs`, in `render_avatar_button`, before the `dropdown_menu` closure, clone the email: `let menu_email = auth.email.clone().unwrap_or_default();`
- [ ] 2.2 Move `menu_email` into the `dropdown_menu` closure (add it to the capture list alongside `settings`)
- [ ] 2.3 Prepend `menu.item(PopupMenuItem::label(menu_email)).item(PopupMenuItem::separator())` before the existing `menu.item(PopupMenuItem::new("Log Out")...)` chain

## 3. Verification

- [ ] 3.1 Run `cargo check --all-targets` and confirm no compile errors
- [ ] 3.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings
- [ ] 3.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 3.4 Launch the app; open the avatar button menu and confirm the account email appears as a non-interactive label at the top
- [ ] 3.5 Confirm a visual separator appears between the email label and the "Log Out" item
- [ ] 3.6 Confirm "Log Out" still functions correctly
