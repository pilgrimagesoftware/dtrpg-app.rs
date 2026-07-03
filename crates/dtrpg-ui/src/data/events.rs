//! Controller events emitted when state changes.

use crate::controllers::activity::ActivityController;
use crate::controllers::auth_state::AuthStateController;
use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::controllers::tabs::TabsController;
use crate::services::LoginTokens;
use gpui::EventEmitter;

// ── LibraryChanged ────────────────────────────────────────────────────────────

/// Emitted when any library state changes that should trigger a re-render.
pub struct LibraryChanged;

impl EventEmitter<LibraryChanged> for LibraryController {}

// ── ActivityChanged ───────────────────────────────────────────────────────────

/// Emitted when the activity item list changes (item added, completed, or errored).
pub struct ActivityChanged;

impl EventEmitter<ActivityChanged> for ActivityController {}

// ── DownloadComplete ──────────────────────────────────────────────────────────

/// Emitted when a download activity item transitions to `Complete`.
pub struct DownloadComplete {
    /// Display label of the completed item.
    pub title: std::sync::Arc<str>,
}

impl EventEmitter<DownloadComplete> for ActivityController {}

// ── DownloadError ─────────────────────────────────────────────────────────────

/// Emitted when a download activity item transitions to `Error`.
pub struct DownloadError {
    /// Display label of the failed item.
    pub title: std::sync::Arc<str>,
    /// Error message.
    pub message: String,
}

impl EventEmitter<DownloadError> for ActivityController {}

// ── SettingsChanged ───────────────────────────────────────────────────────────

/// Emitted when any settings state changes that should trigger a re-render.
pub struct SettingsChanged;

// ── LogoutRequested ───────────────────────────────────────────────────────────

/// Emitted by `SettingsController` when the user requests to log out.
pub struct LogoutRequested;

// ── AuthStateChanged ──────────────────────────────────────────────────────────

/// Emitted when authentication state or the active notice list changes.
pub struct AuthStateChanged;

impl EventEmitter<AuthStateChanged> for AuthStateController {}

// ── SignInSucceeded ───────────────────────────────────────────────────────────

/// Emitted by `SettingsController` when the user successfully signs in from the Account tab.
///
/// The receiver should update `AuthStateController` and replace the `LibraryService`.
pub struct SignInSucceeded(pub LoginTokens);

impl EventEmitter<SignInSucceeded> for SettingsController {}

// ── StartupAuthBegun ──────────────────────────────────────────────────────────

/// Emitted by `SettingsController` when a background startup re-authentication begins.
///
/// The receiver should set `AuthStateController` to the auth-pending state so the
/// banner shows "Signing in..." instead of "Not signed in".
pub struct StartupAuthBegun;

impl EventEmitter<StartupAuthBegun> for SettingsController {}

// ── CollectionCreateFailed ────────────────────────────────────────────────────

/// Emitted by `LibraryController` when a background collection create call fails.
///
/// The receiver should push an error `Notification` to the window.
pub struct CollectionCreateFailed {
    /// Human-readable error message.
    pub message: String,
}

impl EventEmitter<CollectionCreateFailed> for LibraryController {}

// ── StartupAuthFailed ─────────────────────────────────────────────────────────

/// Emitted by `SettingsController` when background startup re-authentication fails.
///
/// The receiver should clear the auth-pending state so the banner transitions to
/// the normal "Not signed in" notice.
pub struct StartupAuthFailed;

impl EventEmitter<StartupAuthFailed> for SettingsController {}

// ── CacheCleared ───────────────────────────────────────────────────────────────

/// Emitted by `SettingsController` after the on-disk app cache has been deleted.
///
/// The receiver should clear `LibraryController`'s in-memory catalog and collections
/// and force a fresh live fetch, so cleared content disappears from the UI immediately
/// instead of lingering until the next unrelated reload.
pub struct CacheCleared;

impl EventEmitter<CacheCleared> for SettingsController {}

// ── TabsChanged ───────────────────────────────────────────────────────────────

/// Emitted when the open tab list or the active tab changes.
pub struct TabsChanged;

impl EventEmitter<TabsChanged> for TabsController {}
