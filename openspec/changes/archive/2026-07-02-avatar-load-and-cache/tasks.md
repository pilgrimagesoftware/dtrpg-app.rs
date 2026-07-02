## 1. Profile config module

- [x] 1.1 Create `dtrpg-ui/src/data/profile.rs` with `ProfileConfig` struct that reads/writes `{config_dir}/dtrpg/profile.toml` containing `email: Option<String>`
- [x] 1.2 Implement `ProfileConfig::load() -> Self`, `ProfileConfig::email(&self) -> Option<&str>`, and `ProfileConfig::save(email: Option<&str>)`
- [x] 1.3 Register `profile` in `dtrpg-ui/src/data/mod.rs`

## 2. Avatar disk cache

- [x] 2.1 Add `fn avatar_cache_path() -> Option<PathBuf>` in `data/avatar.rs` returning `{cache_dir}/dtrpg/avatar`
- [x] 2.2 Add `fn load_cached_avatar() -> Option<Vec<u8>>` that reads the cache file and returns `None` if missing or empty
- [x] 2.3 Add `fn save_cached_avatar(bytes: &[u8])` that creates the parent directory if needed and writes the bytes, logging a warning on failure
- [x] 2.4 Update `fetch_avatar_bytes(email)` to call `load_cached_avatar()` first; return cached bytes if found; after a successful network fetch call `save_cached_avatar(&bytes)` before returning

## 3. LoginController email field

- [x] 3.1 Add `email_draft: String` field to `LoginController`; initialize from `ProfileConfig::load().email().unwrap_or_default()`
- [x] 3.2 Add `pub fn email_draft(&self) -> &str` and `pub fn set_email(&mut self, value: String, cx: &mut Context<Self>)` (emits `LoginStateChanged::Changed`)
- [x] 3.3 In `LoginController::submit`, after storing credentials on success: call `ProfileConfig::save(Some(&self.email_draft.trim()))` if non-empty, else `ProfileConfig::save(None)`

## 4. Login form email field

- [x] 4.1 Add a second `InputState` (non-masked, placeholder "Email (optional, for avatar)") to `LoginView`; pre-populate from `LoginController::email_draft()`
- [x] 4.2 Wire `InputEvent::Change` on the email input to `LoginController::set_email`
- [x] 4.3 Render the email input below the API key input in `LoginView::render`

## 5. Wire set_logged_in in LibraryRootView

- [x] 5.1 In `LibraryRootView::new`, after constructing `settings`, call `ProfileConfig::load()` and if `email()` is `Some`, call `settings.update(cx, |s, cx| s.set_logged_in(email.to_string(), cx))`
- [x] 5.2 Remove the `#[cfg(debug_assertions)]` stub in `root_view.rs` that called `set_logged_in("test@example.com".into(), cx)`

## 6. Verify

- [x] 6.1 Run `cargo test --all-features --workspace` and confirm all tests pass
- [x] 6.2 Manually launch the app, sign in with an API key and email, confirm the avatar appears in the toolbar
- [x] 6.3 Quit and relaunch; confirm the avatar loads from disk cache (no Gravatar network request on the second launch)
- [x] 6.4 Sign in with no email; confirm the initial-letter fallback renders and no network request is made
