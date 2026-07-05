//! Settings controller: owns open/closed state and file-opener overrides.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{Context, Entity, EventEmitter};
use gpui_component::input::InputState;

use crate::credentials::{Credential, CredentialStore, KeyringCredentialStore};
use crate::data::avatar::fetch_avatar_bytes;
use crate::data::constants::{KEYRING_API_KEY, KEYRING_SERVICE};
use crate::data::events::{
    CacheCleared, LogoutRequested, SettingsChanged, SignInSucceeded, StartupAuthBegun,
    StartupAuthFailed,
};
use crate::data::file_openers::{AddOutcome, FileOpenerConfig, FileOpenerEntry};
use crate::data::profile::ProfileConfig;
use crate::data::storage::{StorageConfig, StorageError, validate_writable};
use crate::services::LoginService;

// ── AuthState
// ─────────────────────────────────────────────────────────────────

/// Tracks the current authentication state and cached avatar data.
#[derive(Clone)]
pub enum AuthState {
    /// No user is signed in.
    LoggedOut,
    /// A user is signed in; avatar bytes are fetched asynchronously after
    /// login.
    LoggedIn {
        /// Account email address, if known.
        email:        Option<String>,
        /// Cached Gravatar image bytes, or `None` while the fetch is in flight
        /// or unavailable.
        avatar_bytes: Option<Arc<Vec<u8>>>,
    },
}

/// Snapshot of auth state for a single render pass.
#[derive(Clone)]
pub struct AuthStateSnapshot {
    /// `true` when a user is signed in.
    pub is_logged_in:    bool,
    /// Account email address when signed in.
    pub email:           Option<String>,
    /// First character of the email, uppercased — used as the avatar fallback
    /// initial.
    pub display_initial: Option<char>,
    /// Cached avatar image bytes from Gravatar, or `None`.
    pub avatar_bytes:    Option<Arc<Vec<u8>>>,
    /// Masked API key hint (first 4 + bullets + last 1), set at sign-in time.
    pub api_key_hint:    Option<String>,
}

// ── SettingsController
// ────────────────────────────────────────────────────────

/// Snapshot of settings state needed by the views for a single render pass.
pub struct SettingsSnapshot {
    pub is_open:             bool,
    pub file_openers:        Vec<FileOpenerEntry>,
    /// `true` when credentials are present in the keyring.
    pub is_authenticated:    bool,
    /// Resolved storage root path (override or platform default).
    pub storage_root_path:   PathBuf,
    /// `true` when the configured storage root is unreachable (e.g. unmounted
    /// volume).
    pub storage_unavailable: bool,
    /// `true` when the configured storage root exists on disk.
    pub storage_path_exists: bool,
    /// Current auth state for the toolbar avatar button.
    pub auth:                AuthStateSnapshot,
    /// Current value of the email draft field in the Account tab.
    pub email_draft:         String,
    /// Current value of the password draft field in the Account tab.
    pub password_draft:      String,
    /// `true` while a sign-in request is in flight.
    pub sign_in_in_progress: bool,
    /// Error message from the last failed sign-in attempt, if any.
    pub sign_in_error:       Option<String>,
    /// Shared input state for the email text field in the Account tab.
    pub email_input:         Option<Entity<InputState>>,
    /// Shared input state for the password text field in the Account tab.
    pub password_input:      Option<Entity<InputState>>,
    /// Current draft value of the storage path text field.
    pub storage_path_draft:  String,
    /// Shared input state for the storage path text field in the Storage tab.
    pub storage_path_input:  Option<Entity<InputState>>,
    /// Application path picked via the native file dialog, awaiting an
    /// extension typed inline in the File Openers list. `None` when no add
    /// is in progress.
    pub pending_file_opener: Option<PathBuf>,
}

/// Owns all mutable settings state: panel visibility, file-opener overrides,
/// catalog storage configuration, and sign-in form state.
pub struct SettingsController {
    is_open:             bool,
    file_openers:        FileOpenerConfig,
    is_authenticated:    bool,
    auth_state:          AuthState,
    storage:             StorageConfig,
    storage_unavailable: bool,
    storage_path_exists: bool,
    login_service:       Arc<dyn LoginService>,
    email_draft:         String,
    password_draft:      String,
    sign_in_in_progress: bool,
    sign_in_error:       Option<String>,
    /// Input state for the email text field; set by the root view after
    /// creation.
    email_input:         Option<Entity<InputState>>,
    /// Input state for the password text field; set by the root view after
    /// creation.
    password_input:      Option<Entity<InputState>>,
    /// Draft value of the storage path text field.
    storage_path_draft:  String,
    /// Input state for the storage path text field; set by the root view after
    /// creation.
    storage_path_input:  Option<Entity<InputState>>,
    /// Masked API key hint computed at sign-in time (first 4 + bullets + last
    /// 1).
    api_key_hint:        Option<String>,
    /// Application path picked via the native file dialog, awaiting an
    /// extension typed inline in the File Openers list. `None` when no add
    /// is in progress.
    pending_file_opener: Option<PathBuf>,
}

impl SettingsController {
    /// Creates a controller, loading the file-opener list from disk.
    ///
    /// Checks the platform keyring to determine initial auth state, and
    /// verifies the configured storage root is accessible. Spawns a
    /// background check for path existence.
    ///
    /// When no storage override is configured (first launch, or after a reset),
    /// the platform default download directory is created automatically if
    /// it does not yet exist, rather than surfacing a "storage folder does
    /// not exist" warning for a location the app itself owns. A user-chosen
    /// override path that is missing is left untouched and still surfaces
    /// the warning — that more likely indicates an unmounted volume than a
    /// fresh install.
    pub fn new(login_service: Box<dyn LoginService>, cx: &mut Context<Self>) -> Self {
        let file_openers = FileOpenerConfig::load();
        let loaded_credential = KeyringCredentialStore::new(KEYRING_SERVICE, KEYRING_API_KEY)
            .load()
            .ok()
            .flatten();
        let is_authenticated = loaded_credential.is_some();
        // Pre-fill email from the stored credential; fall back to ProfileConfig
        // for users whose email was saved there before this field moved to the
        // keyring entry.
        let email_draft = loaded_credential.as_ref()
                                           .and_then(|c| c.email.as_deref())
                                           .map(str::to_owned)
                                           .or_else(|| {
                                               ProfileConfig::load()
                                                   .email()
                                                   .map(str::to_owned)
                                           })
                                           .unwrap_or_default();

        let storage = StorageConfig::load();
        if storage.is_default()
           && !storage.is_accessible()
           && let Err(e) = storage.ensure_root_exists()
        {
            tracing::warn!(
                error = %e,
                path = %storage.root_path().display(),
                "failed to create default download directory"
            );
        }
        let storage_unavailable = !storage.is_accessible();
        if storage_unavailable {
            tracing::warn!(
                path = %storage.root_path().display(),
                "configured storage root is not accessible"
            );
        }
        let initial_path = storage.root_path();
        let storage_path_draft = initial_path.to_string_lossy().into_owned();

        let mut ctrl = Self { is_open: false,
                              file_openers,
                              is_authenticated,
                              auth_state: AuthState::LoggedOut,
                              storage_path_exists: true,
                              storage,
                              storage_unavailable,
                              login_service: Arc::from(login_service),
                              email_draft,
                              password_draft: String::new(),
                              sign_in_in_progress: false,
                              sign_in_error: None,
                              email_input: None,
                              password_input: None,
                              storage_path_draft,
                              storage_path_input: None,
                              api_key_hint: None,
                              pending_file_opener: None };
        ctrl.check_storage_path_exists(initial_path, cx);
        ctrl
    }

    /// Attaches the email input state entity created by the root view.
    pub fn set_email_input(&mut self, input: Entity<InputState>) {
        self.email_input = Some(input);
    }

    /// Attaches the password input state entity created by the root view.
    pub fn set_password_input(&mut self, input: Entity<InputState>) {
        self.password_input = Some(input);
    }

    /// Attaches the storage path input state entity created by the root view.
    pub fn set_storage_path_input(&mut self, input: Entity<InputState>) {
        self.storage_path_input = Some(input);
    }

    /// Updates the storage path draft field.
    pub fn set_storage_path_draft(&mut self, value: String, cx: &mut Context<Self>) {
        self.storage_path_draft = value;
        cx.emit(SettingsChanged);
    }
}

impl SettingsController {
    /// Emits `LogoutRequested` so the library root view can coordinate the
    /// logout flow.
    pub fn request_logout(&mut self, cx: &mut Context<Self>) {
        cx.emit(LogoutRequested);
    }

    // ── Auth state ────────────────────────────────────────────────────────────

    /// Marks the user as signed in.
    ///
    /// When `email` is `Some`, spawns a background task to fetch the Gravatar
    /// avatar. Emits [`SettingsChanged`] immediately and again once avatar
    /// bytes arrive (if applicable).
    pub fn set_logged_in(&mut self, email: Option<String>, api_key: Option<&str>,
                         cx: &mut Context<Self>) {
        self.is_authenticated = true;
        self.auth_state = AuthState::LoggedIn { email:        email.clone(),
                                                avatar_bytes: None, };
        self.api_key_hint = api_key.map(mask_api_key);
        cx.emit(SettingsChanged);

        if let Some(addr) = email {
            cx.spawn(async move |this, async_cx| {
                  let bytes = async_cx.background_executor()
                                      .spawn(async move { fetch_avatar_bytes(addr) })
                                      .await;
                  this.update(async_cx, |ctrl, cx| ctrl.set_avatar_bytes(bytes, cx))
                      .ok();
              })
              .detach();
        }
    }

    /// Stores fetched avatar bytes and re-renders.
    ///
    /// No-op if the user is not currently signed in.
    pub fn set_avatar_bytes(&mut self, bytes: Option<Vec<u8>>, cx: &mut Context<Self>) {
        if let AuthState::LoggedIn { avatar_bytes, .. } = &mut self.auth_state {
            *avatar_bytes = bytes.map(Arc::new);
            cx.emit(SettingsChanged);
        }
    }

    /// Clears the auth state and triggers the full logout flow.
    ///
    /// Emits [`SettingsChanged`] to update the UI, then [`LogoutRequested`] to
    /// prompt the root view to delete credentials and open the login window.
    pub fn logout(&mut self, cx: &mut Context<Self>) {
        self.is_authenticated = false;
        self.auth_state = AuthState::LoggedOut;
        self.api_key_hint = None;
        cx.emit(SettingsChanged);
        cx.emit(LogoutRequested);
    }

    // ── Storage ───────────────────────────────────────────────────────────────

    /// Returns the resolved storage root path.
    pub fn storage_root_path(&self) -> PathBuf {
        self.storage.root_path()
    }

    /// Validates `path` for writability and saves it as the new storage root.
    ///
    /// Emits [`SettingsChanged`] on success. Returns the validation error on
    /// failure so the caller can surface it to the user.
    ///
    /// # Errors
    ///
    /// Returns a [`StorageError`] if `path` is missing, unwritable, or on an
    /// unavailable volume.
    pub fn apply_storage_path(&mut self, path: PathBuf, cx: &mut Context<Self>)
                              -> Result<(), StorageError> {
        validate_writable(&path)?;
        self.storage.set_root_path(path.clone());
        self.storage_unavailable = false;
        self.check_storage_path_exists(path, cx);
        cx.emit(SettingsChanged);
        Ok(())
    }

    /// Spawns a background task to check whether `path` exists on disk.
    ///
    /// Writes the result back to `storage_path_exists` and notifies the UI.
    fn check_storage_path_exists(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        cx.spawn(async move |this, async_cx| {
              let exists = async_cx.background_executor()
                                   .spawn(async move { path.exists() })
                                   .await;
              this.update(async_cx, |ctrl, cx| {
                      ctrl.storage_path_exists = exists;
                      cx.notify();
                  })
                  .ok();
          })
          .detach();
    }

    /// Opens the OS file manager at the configured storage root, creating the
    /// directory first if it does not exist.
    pub fn reveal_storage_location(&self) {
        let path = self.storage.root_path();
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }
        if let Err(e) = crate::util::reveal::reveal_in_file_manager(&path) {
            tracing::warn!("reveal_in_file_manager failed: {e}");
        }
    }

    /// Deletes all regenerable app cache data (catalog/collections metadata
    /// cache and the cached avatar image) from disk, then emits
    /// [`CacheCleared`] so the library view drops its in-memory catalog and
    /// re-fetches live.
    ///
    /// Does not touch downloaded content, credentials, or preferences.
    pub fn clear_cache(&self, cx: &mut Context<Self>) {
        let dir = crate::data::paths::app_cache_dir();
        if let Err(e) = std::fs::remove_dir_all(&dir)
           && e.kind() != std::io::ErrorKind::NotFound
        {
            tracing::warn!("clear cache: failed to remove {}: {e}", dir.display());
        }
        cx.emit(CacheCleared);
    }

    // ── Panel visibility ──────────────────────────────────────────────────────

    /// Returns `true` when the settings panel is visible.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    // ── Sign-in ───────────────────────────────────────────────────────────────

    /// Returns the current email draft value.
    pub fn email_draft(&self) -> &str {
        &self.email_draft
    }

    /// Updates the email draft field and clears any prior sign-in error.
    pub fn set_email_draft(&mut self, value: String, cx: &mut Context<Self>) {
        self.email_draft = value;
        self.sign_in_error = None;
        cx.emit(SettingsChanged);
    }

    /// Updates the password draft field and clears any prior sign-in error.
    pub fn set_password_draft(&mut self, value: String, cx: &mut Context<Self>) {
        self.password_draft = value;
        self.sign_in_error = None;
        cx.emit(SettingsChanged);
    }

    /// Authenticates silently at startup using a stored API key.
    ///
    /// Emits [`StartupAuthBegun`] immediately, then runs authentication on a
    /// background thread. On success, emits [`SignInSucceeded`] so the root
    /// view can replace the library service. On failure, emits
    /// [`StartupAuthFailed`] and logs a warning.
    pub fn startup_auth(&mut self, api_key: String, cx: &mut Context<Self>) {
        let email = {
            let trimmed = self.email_draft.trim();
            if trimmed.is_empty() {
                None
            }
            else {
                Some(trimmed.to_owned())
            }
        };
        let svc = self.login_service.clone();

        cx.emit(StartupAuthBegun);

        cx.spawn(async move |this, async_cx| {
              let api_key_hint = api_key.clone();
              let result = async_cx.background_executor()
                                   .spawn(async move { svc.authenticate(&api_key) })
                                   .await;

              this.update(async_cx, |ctrl, cx| match result {
                      Ok(tokens) => {
                          ctrl.set_logged_in(email, Some(api_key_hint.as_str()), cx);
                          cx.emit(SignInSucceeded(tokens));
                      }
                      Err(e) => {
                          tracing::warn!("startup re-authentication failed: {e}");
                          cx.emit(StartupAuthFailed);
                      }
                  })
                  .ok();
          })
          .detach();
    }

    /// Signs in using the current `email_draft` and `password_draft`.
    ///
    /// First exchanges the email/password pair for an application key via
    /// the SDK credential exchange, then exchanges that key for session tokens
    /// via the existing authentication call. On success, stores the credential
    /// to the keyring and emits [`SignInSucceeded`]. On failure at either step,
    /// sets `sign_in_error` with a message identifying which step failed.
    pub fn sign_in(&mut self, cx: &mut Context<Self>) {
        let email = self.email_draft.trim().to_owned();
        let password = self.password_draft.trim().to_owned();
        if self.sign_in_in_progress || email.is_empty() || password.is_empty() {
            return;
        }
        self.sign_in_in_progress = true;
        self.sign_in_error = None;
        cx.emit(SettingsChanged);

        let svc = self.login_service.clone();

        cx.spawn(async move |this, async_cx| {
              // Step 1: exchange email + password for application key.
              let credential_result = async_cx.background_executor()
                                              .spawn({
                                                  let email = email.clone();
                                                  let password = password.clone();
                                                  async move {
                                                      svc.login_with_credentials(&email, &password)
                                                         .map(|key| (svc, key))
                                                  }
                                              })
                                              .await;

              let (svc, api_key) = match credential_result {
                  Ok(pair) => pair,
                  Err(e) => {
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.sign_in_in_progress = false;
                              ctrl.sign_in_error = Some(e.0);
                              cx.emit(SettingsChanged);
                          })
                          .ok();
                      return;
                  }
              };

              // Step 2: exchange application key for session tokens.
              let auth_result = async_cx.background_executor()
                                        .spawn({
                                            let key = api_key.clone();
                                            async move { svc.authenticate(&key).map(|t| (key, t)) }
                                        })
                                        .await;

              this.update(async_cx, |ctrl, cx| {
                      ctrl.sign_in_in_progress = false;
                      match auth_result {
                          Ok((key, tokens)) => {
                              let store =
                                  KeyringCredentialStore::new(KEYRING_SERVICE, KEYRING_API_KEY);
                              if let Err(e) =
                                  store.store(&Credential { service: KEYRING_SERVICE.into(),
                                                            account: KEYRING_API_KEY.into(),
                                                            secret:  key.clone(),
                                                            email:   Some(email.clone()), })
                              {
                                  tracing::warn!(
                                      "failed to save credential to keyring: {e}"
                                  );
                              }
                              ProfileConfig::save(Some(&email));
                              ctrl.password_draft.clear();
                              ctrl.set_logged_in(Some(email), Some(key.as_str()), cx);
                              cx.emit(SignInSucceeded(tokens));
                          }
                          Err(e) => {
                              ctrl.sign_in_error = Some(format!(
                                  "Session setup failed after sign-in: {}",
                                  e.0
                              ));
                              cx.emit(SettingsChanged);
                          }
                      }
                  })
                  .ok();
          })
          .detach();
    }

    // ── Panel visibility ──────────────────────────────────────────────────────

    /// Opens the settings panel.
    pub fn open(&mut self, cx: &mut Context<Self>) {
        if !self.is_open {
            self.is_open = true;
            cx.emit(SettingsChanged);
        }
    }

    /// Closes the settings panel.
    pub fn close(&mut self, cx: &mut Context<Self>) {
        if self.is_open {
            self.is_open = false;
            cx.emit(SettingsChanged);
        }
    }

    /// Toggles the settings panel open/closed.
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.is_open = !self.is_open;
        cx.emit(SettingsChanged);
    }

    // ── File-opener overrides ─────────────────────────────────────────────────

    /// Returns a shared reference to the file-opener config.
    pub fn file_openers(&self) -> &FileOpenerConfig {
        &self.file_openers
    }

    /// Adds or replaces a file-opener entry and persists the change.
    pub fn add_file_opener(&mut self, entry: FileOpenerEntry, cx: &mut Context<Self>)
                           -> AddOutcome {
        let outcome = self.file_openers.add(entry);
        self.file_openers.save();
        cx.emit(SettingsChanged);
        outcome
    }

    /// Removes the file-opener entry for `extension` and persists the change.
    pub fn remove_file_opener(&mut self, extension: &str, cx: &mut Context<Self>) {
        self.file_openers.remove(extension);
        self.file_openers.save();
        cx.emit(SettingsChanged);
    }

    /// Begins an in-place "add file opener" flow after the user has picked an
    /// application via the native file dialog.
    ///
    /// The extension is entered inline in the File Openers list rather than in
    /// a separate modal; the list renders a pending row for `app_path`
    /// until the flow is committed or cancelled.
    pub fn begin_add_file_opener(&mut self, app_path: PathBuf, cx: &mut Context<Self>) {
        self.pending_file_opener = Some(app_path);
        cx.emit(SettingsChanged);
    }

    /// Cancels an in-progress "add file opener" flow without persisting
    /// anything.
    pub fn cancel_pending_file_opener(&mut self, cx: &mut Context<Self>) {
        if self.pending_file_opener.take().is_some() {
            cx.emit(SettingsChanged);
        }
    }

    /// Commits the in-progress "add file opener" flow using `extension`.
    ///
    /// No-op if no add is in progress or `extension` is empty after trimming
    /// (the caller should
    /// [`cancel_pending_file_opener`](Self::cancel_pending_file_opener)
    /// explicitly in that case, e.g. on blur).
    pub fn commit_pending_file_opener(&mut self, extension: &str, cx: &mut Context<Self>) {
        let trimmed = extension.trim();
        if trimmed.is_empty() {
            return;
        }
        let Some(app_path) = self.pending_file_opener.take()
        else {
            return;
        };
        self.add_file_opener(FileOpenerEntry { extension: trimmed.to_string(),
                                               app_path },
                             cx);
    }

    // ── Snapshot ──────────────────────────────────────────────────────────────

    /// Returns all data needed by the views for one render pass.
    pub fn snapshot(&self) -> SettingsSnapshot {
        let auth = match &self.auth_state {
            AuthState::LoggedOut => AuthStateSnapshot { is_logged_in:    false,
                                                        email:           None,
                                                        display_initial: None,
                                                        avatar_bytes:    None,
                                                        api_key_hint:    None, },
            AuthState::LoggedIn { email,
                                  avatar_bytes, } => {
                AuthStateSnapshot { is_logged_in:    true,
                                    display_initial: email.as_deref()
                                                          .and_then(|s| s.trim().chars().next())
                                                          .map(|c| c.to_ascii_uppercase()),
                                    email:           email.clone(),
                                    avatar_bytes:    avatar_bytes.clone(),
                                    api_key_hint:    self.api_key_hint.clone(), }
            }
        };

        SettingsSnapshot { is_open: self.is_open,
                           file_openers: self.file_openers.entries().to_vec(),
                           is_authenticated: self.is_authenticated,
                           storage_root_path: self.storage.root_path(),
                           storage_unavailable: self.storage_unavailable,
                           storage_path_exists: self.storage_path_exists,
                           auth,
                           email_draft: self.email_draft.clone(),
                           password_draft: self.password_draft.clone(),
                           sign_in_in_progress: self.sign_in_in_progress,
                           sign_in_error: self.sign_in_error.clone(),
                           email_input: self.email_input.clone(),
                           password_input: self.password_input.clone(),
                           storage_path_draft: self.storage_path_draft.clone(),
                           storage_path_input: self.storage_path_input.clone(),
                           pending_file_opener: self.pending_file_opener.clone() }
    }
}

impl EventEmitter<SettingsChanged> for SettingsController {}
impl EventEmitter<LogoutRequested> for SettingsController {}

// ── Helpers
// ───────────────────────────────────────────────────────────────────

/// Returns a masked representation of an API key: first 4 chars + bullets +
/// last 1 char. For keys 5 characters or shorter, returns only bullets.
fn mask_api_key(key: &str) -> String {
    if key.len() > 5 {
        format!("{}••••••••{}", &key[..4], &key[key.len() - 1..])
    }
    else {
        "••••••••".to_string()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_api_key_long_key() {
        assert_eq!(mask_api_key("abcdefghij1"), "abcd••••••••1");
    }

    #[test]
    fn mask_api_key_short_key() {
        assert_eq!(mask_api_key("abc"), "••••••••");
    }

    #[test]
    fn mask_api_key_exactly_five() {
        assert_eq!(mask_api_key("abcde"), "••••••••");
    }
}
