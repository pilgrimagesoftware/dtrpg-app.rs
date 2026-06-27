## 1. Update LoginController to accept an initial API key draft

- [x] 1.1 Change `LoginController::new` signature to `new(service: Box<dyn LoginService>, prefilled_key: Option<String>) -> Self`
- [x] 1.2 Set `api_key_draft` from `prefilled_key.unwrap_or_default()` in the constructor body
- [x] 1.3 Update all existing call sites of `LoginController::new` to pass `None` (login window, tests)

## 2. Update open_login_window to accept a pre-filled key

- [x] 2.1 Change `open_login_window` signature to `open_login_window(prefilled_key: Option<String>, cx: &mut App)`
- [x] 2.2 Pass `prefilled_key` through to `LoginController::new` inside the window closure
- [x] 2.3 Update all existing call sites of `open_login_window` to pass `None` (`setup()`, `root_view.rs` logout handler)

## 3. Add silent re-authentication to setup()

- [x] 3.1 In `setup()` in `ui/app/mod.rs`, after finding a stored API key, build a `LoginService` from `LoginServiceFactory` and call `authenticate(&api_key)`
- [x] 3.2 On success: store the new `access_token` in the keyring (`keys::ACCESS_TOKEN`) and the `refresh_token` in the keyring (`keys::REFRESH_TOKEN`, best-effort); then call `open_library_window(cx)`
- [x] 3.3 On failure: log a warning, delete the stored `access_token` from the keyring, call `open_login_window(Some(api_key), cx)`

## 4. Verify

- [x] 4.1 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 4.2 Manually launch the app with a valid API key in the keyring and confirm the library window opens (no login prompt)
- [ ] 4.3 Manually launch the app with an invalid/expired API key in the keyring and confirm the login window opens with the key pre-filled
