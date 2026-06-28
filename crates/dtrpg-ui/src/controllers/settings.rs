//! Settings controller: owns open/closed state, active tab, and file-opener overrides.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{Context, EventEmitter};

use crate::credentials::{CredentialStore, KeyringCredentialStore, keys};
use crate::data::avatar::fetch_avatar_bytes;
use crate::data::events::{LogoutRequested, SettingsChanged};
use crate::data::file_openers::{AddOutcome, FileOpenerConfig, FileOpenerEntry};
use crate::data::storage::{StorageConfig, StorageError, validate_writable};

// ── AuthState ─────────────────────────────────────────────────────────────────

/// Tracks the current authentication state and cached avatar data.
#[derive(Clone)]
pub enum AuthState {
    /// No user is signed in.
    LoggedOut,
    /// A user is signed in; avatar bytes are fetched asynchronously after login.
    LoggedIn {
        /// Account email address, if known.
        email: Option<String>,
        /// Cached Gravatar image bytes, or `None` while the fetch is in flight or unavailable.
        avatar_bytes: Option<Arc<Vec<u8>>>,
    },
}

/// Snapshot of auth state for a single render pass.
pub struct AuthStateSnapshot {
    /// `true` when a user is signed in.
    pub is_logged_in: bool,
    /// Account email address when signed in.
    pub email: Option<String>,
    /// First character of the email, uppercased — used as the avatar fallback initial.
    pub display_initial: Option<char>,
    /// Cached avatar image bytes from Gravatar, or `None`.
    pub avatar_bytes: Option<Arc<Vec<u8>>>,
}

// ── SettingsTab ───────────────────────────────────────────────────────────────

/// The three tabs available in the settings panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    Account,
    Storage,
    FileOpeners,
}

impl SettingsTab {
    /// Human-readable label used in the tab strip.
    pub fn label(self) -> &'static str {
        match self {
            Self::Account => "Account",
            Self::Storage => "Storage",
            Self::FileOpeners => "File Openers",
        }
    }

    /// Parses a persisted tab name back to a `SettingsTab`.
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "Account" => Some(Self::Account),
            "Storage" => Some(Self::Storage),
            "FileOpeners" => Some(Self::FileOpeners),
            _ => None,
        }
    }

    /// The string written to disk when persisting the active tab.
    fn to_name(self) -> &'static str {
        match self {
            Self::Account => "Account",
            Self::Storage => "Storage",
            Self::FileOpeners => "FileOpeners",
        }
    }
}

// ── SettingsController ────────────────────────────────────────────────────────

/// Snapshot of settings state needed by the views for a single render pass.
pub struct SettingsSnapshot {
    pub is_open: bool,
    pub active_tab: SettingsTab,
    pub file_openers: Vec<FileOpenerEntry>,
    /// `true` when an API key is present in the keyring.
    pub is_authenticated: bool,
    /// Resolved storage root path (override or platform default).
    pub storage_root_path: PathBuf,
    /// `true` when the configured storage root is unreachable (e.g. unmounted volume).
    pub storage_unavailable: bool,
    /// Current auth state for the toolbar avatar button.
    pub auth: AuthStateSnapshot,
}

/// Owns all mutable settings state: panel visibility, active tab, file-opener overrides,
/// and catalog storage configuration.
pub struct SettingsController {
    is_open: bool,
    active_tab: SettingsTab,
    file_openers: FileOpenerConfig,
    is_authenticated: bool,
    auth_state: AuthState,
    storage: StorageConfig,
    storage_unavailable: bool,
}

impl SettingsController {
    /// Creates a controller, restoring the last-active tab and file-opener list from disk.
    ///
    /// Checks the platform keyring to determine initial auth state, and verifies the
    /// configured storage root is accessible.
    pub fn new() -> Self {
        let (file_openers, tab_name) = FileOpenerConfig::load_with_tab();
        let active_tab = tab_name
            .as_deref()
            .and_then(SettingsTab::from_name)
            .unwrap_or_default();
        let is_authenticated = KeyringCredentialStore::new(keys::SERVICE, keys::API_KEY)
            .load()
            .ok()
            .flatten()
            .is_some();
        let storage = StorageConfig::load();
        let storage_unavailable = !storage.is_accessible();
        if storage_unavailable {
            tracing::warn!(
                path = %storage.root_path().display(),
                "configured storage root is not accessible"
            );
        }
        Self {
            is_open: false,
            active_tab,
            file_openers,
            is_authenticated,
            auth_state: AuthState::LoggedOut,
            storage,
            storage_unavailable,
        }
    }
}

impl Default for SettingsController {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsController {
    /// Emits `LogoutRequested` so the library root view can coordinate the logout flow.
    pub fn request_logout(&mut self, cx: &mut Context<Self>) {
        cx.emit(LogoutRequested);
    }

    // ── Auth state ────────────────────────────────────────────────────────────

    /// Marks the user as signed in.
    ///
    /// When `email` is `Some`, spawns a background task to fetch the Gravatar avatar.
    /// Emits [`SettingsChanged`] immediately and again once avatar bytes arrive (if applicable).
    pub fn set_logged_in(&mut self, email: Option<String>, cx: &mut Context<Self>) {
        self.is_authenticated = true;
        self.auth_state = AuthState::LoggedIn { email: email.clone(), avatar_bytes: None };
        cx.emit(SettingsChanged);

        if let Some(addr) = email {
            cx.spawn(async move |this, async_cx| {
                let bytes = async_cx
                    .background_executor()
                    .spawn(async move { fetch_avatar_bytes(addr) })
                    .await;
                this.update(async_cx, |ctrl, cx| ctrl.set_avatar_bytes(bytes, cx)).ok();
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
    /// Emits [`SettingsChanged`] on success. Returns the validation error on failure
    /// so the caller can surface it to the user.
    ///
    /// # Errors
    ///
    /// Returns a [`StorageError`] if `path` is missing, unwritable, or on an
    /// unavailable volume.
    pub fn apply_storage_path(
        &mut self,
        path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Result<(), StorageError> {
        validate_writable(&path)?;
        self.storage.set_root_path(path);
        self.storage_unavailable = false;
        cx.emit(SettingsChanged);
        Ok(())
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

    // ── Panel visibility ──────────────────────────────────────────────────────

    /// Returns `true` when the settings panel is visible.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

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

    // ── Tab navigation ────────────────────────────────────────────────────────

    /// Returns the currently active settings tab.
    pub fn active_tab(&self) -> SettingsTab {
        self.active_tab
    }

    /// Sets the active tab and persists it to disk.
    pub fn set_tab(&mut self, tab: SettingsTab, cx: &mut Context<Self>) {
        self.active_tab = tab;
        self.file_openers.save(Some(tab.to_name()));
        cx.emit(SettingsChanged);
    }

    // ── File-opener overrides ─────────────────────────────────────────────────

    /// Returns a shared reference to the file-opener config.
    pub fn file_openers(&self) -> &FileOpenerConfig {
        &self.file_openers
    }

    /// Adds or replaces a file-opener entry and persists the change.
    pub fn add_file_opener(&mut self, entry: FileOpenerEntry, cx: &mut Context<Self>) -> AddOutcome {
        let outcome = self.file_openers.add(entry);
        self.file_openers.save(Some(self.active_tab.to_name()));
        cx.emit(SettingsChanged);
        outcome
    }

    /// Removes the file-opener entry for `extension` and persists the change.
    pub fn remove_file_opener(&mut self, extension: &str, cx: &mut Context<Self>) {
        self.file_openers.remove(extension);
        self.file_openers.save(Some(self.active_tab.to_name()));
        cx.emit(SettingsChanged);
    }

    // ── Snapshot ──────────────────────────────────────────────────────────────

    /// Returns all data needed by the views for one render pass.
    pub fn snapshot(&self) -> SettingsSnapshot {
        let auth = match &self.auth_state {
            AuthState::LoggedOut => AuthStateSnapshot {
                is_logged_in: false,
                email: None,
                display_initial: None,
                avatar_bytes: None,
            },
            AuthState::LoggedIn { email, avatar_bytes } => AuthStateSnapshot {
                is_logged_in: true,
                display_initial: email
                    .as_deref()
                    .and_then(|s| s.trim().chars().next())
                    .map(|c| c.to_ascii_uppercase()),
                email: email.clone(),
                avatar_bytes: avatar_bytes.clone(),
            },
        };

        SettingsSnapshot {
            is_open: self.is_open,
            active_tab: self.active_tab,
            file_openers: self.file_openers.entries().to_vec(),
            is_authenticated: self.is_authenticated,
            storage_root_path: self.storage.root_path(),
            storage_unavailable: self.storage_unavailable,
            auth,
        }
    }
}

impl EventEmitter<SettingsChanged> for SettingsController {}
impl EventEmitter<LogoutRequested> for SettingsController {}
