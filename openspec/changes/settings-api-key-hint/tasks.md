## 1. Controller: Store and Mask the Key Hint

- [x] 1.1 In `settings.rs`, add `api_key_hint: Option<String>` field to `SettingsController`; initialize to `None` in `new()`
- [x] 1.2 Add a free function `fn mask_api_key(key: &str) -> String` that returns `format!("{}••••••••{}", &key[..4], &key[key.len()-1..])` when `key.len() > 5`, or `"••••••••".to_string()` otherwise
- [x] 1.3 Update `set_logged_in` signature to `pub fn set_logged_in(&mut self, email: Option<String>, api_key: Option<&str>, cx: &mut Context<Self>)`; inside, set `self.api_key_hint = api_key.map(mask_api_key)`
- [x] 1.4 Update `logout` (and `request_logout` if it clears state) to set `self.api_key_hint = None`
- [x] 1.5 Update the `startup_auth` call to `set_logged_in` to pass `Some(api_key.as_str())` (the key is in scope as `api_key`)
- [x] 1.6 Update the `sign_in` call to `set_logged_in` to pass `Some(api_key.as_str())` (the key is in scope as `api_key`)

## 2. Snapshot: Thread the Hint to the View

- [x] 2.1 Add `api_key_hint: Option<String>` field to `AuthStateSnapshot`; populate it as `None` in the `LoggedOut` arm of `snapshot()`
- [x] 2.2 In the `LoggedIn` arm of `snapshot()`, set `api_key_hint: self.api_key_hint.clone()`
- [x] 2.3 Run `cargo check -p dtrpg-ui` and fix any compilation errors from the signature change

## 3. View: Render the Hint

- [x] 3.1 In `settings_account_view.rs`, in `render_authenticated`, add a third `div()` child inside the identity text column (below the email `div`), conditioned on `auth.api_key_hint.is_some()`, showing the hint string in `text_xs()` and `text_color(colors.text_tertiary)`
- [x] 3.2 Run `cargo check -p dtrpg-ui` and confirm zero errors

## 4. Tests

- [x] 4.1 Add a unit test `mask_api_key_long_key` asserting `mask_api_key("abcdefghij1")` returns `"abcd••••••••1"`
- [x] 4.2 Add a unit test `mask_api_key_short_key` asserting `mask_api_key("abc")` returns `"••••••••"`
- [x] 4.3 Add a unit test `mask_api_key_exactly_five` asserting `mask_api_key("abcde")` returns `"••••••••"`
- [x] 4.4 Run `cargo test -p dtrpg-ui` and confirm all tests pass

## 5. Verification

- [x] 5.1 Build and run the app; sign in with a known API key; open Settings → Account
- [x] 5.2 Confirm the key hint row appears below the email (or "DriveThruRPG Account"), showing the correct masked format
- [x] 5.3 Confirm the hint persists after the Gravatar avatar loads
- [x] 5.4 Log out; confirm no hint is shown on the unauthenticated screen
