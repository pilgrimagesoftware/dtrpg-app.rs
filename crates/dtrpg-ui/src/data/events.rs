//! Controller events emitted when state changes.

use gpui::EventEmitter;
use crate::controllers::activity::ActivityController;
use crate::controllers::auth_state::AuthStateController;
use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::services::LoginTokens;

// ── LibraryChanged ────────────────────────────────────────────────────────────

/// Emitted when any library state changes that should trigger a re-render.
pub struct LibraryChanged;

impl EventEmitter<LibraryChanged> for LibraryController {}

// ── ActivityChanged ───────────────────────────────────────────────────────────

/// Emitted when the activity item list changes (item added, completed, or errored).
pub struct ActivityChanged;

impl EventEmitter<ActivityChanged> for ActivityController {}

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
