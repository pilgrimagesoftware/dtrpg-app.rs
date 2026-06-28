## 1. AuthStateSnapshot — add email field

- [x] 1.1 In `controllers/settings.rs`, add `pub email: Option<String>` to `AuthStateSnapshot`
- [x] 1.2 In the `AuthState::LoggedOut` branch of `SettingsController::snapshot()`, set `email: None`
- [x] 1.3 In the `AuthState::LoggedIn { email, avatar_bytes }` branch, set `email: Some(email.clone())`

## 2. Avatar dropdown — add email label and separator

- [x] 2.1 In `toolbar_view.rs`, in `render_avatar_button`, clone email before the closure: `let menu_email = auth.email.clone().unwrap_or_default();`
- [x] 2.2 `menu_email` captured via `move` into the `dropdown_menu` closure
- [x] 2.3 Prepend `menu.item(PopupMenuItem::label(menu_email.clone())).item(PopupMenuItem::separator())` before the Log Out item

## 3. Verification

- [x] 3.1 Run `cargo check --all-targets` — no compile errors
- [x] 3.2 No new clippy warnings introduced
- [x] 3.3 Run `cargo test --all-features --workspace` — all tests pass
- [ ] 3.4 Launch the app; open the avatar button menu and confirm the account email appears as a non-interactive label at the top
- [ ] 3.5 Confirm a visual separator appears between the email label and the "Log Out" item
- [ ] 3.6 Confirm "Log Out" still functions correctly
