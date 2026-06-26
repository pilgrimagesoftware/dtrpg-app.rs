//! Controller events emitted when state changes.

use gpui::EventEmitter;
use crate::controllers::library::LibraryController;

// ── LibraryChanged ────────────────────────────────────────────────────────────

/// Emitted when any library state changes that should trigger a re-render.
pub struct LibraryChanged;

impl EventEmitter<LibraryChanged> for LibraryController {}

// ── SettingsChanged ───────────────────────────────────────────────────────────

/// Emitted when any settings state changes that should trigger a re-render.
pub struct SettingsChanged;

// ── LogoutRequested ───────────────────────────────────────────────────────────

/// Emitted by `SettingsController` when the user requests to log out.
pub struct LogoutRequested;

// ── LoginStateChanged ─────────────────────────────────────────────────────────

/// Emitted by `LoginController` when auth state transitions.
pub enum LoginStateChanged {
    /// The draft API key or in-progress/error state changed; views should re-render.
    Changed,
    /// Login succeeded; the caller should open the library window.
    Succeeded,
    /// Logout completed; the caller should open the login window.
    LoggedOut,
}
