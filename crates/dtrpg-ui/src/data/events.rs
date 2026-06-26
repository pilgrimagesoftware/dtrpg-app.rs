//! Libri library controller: owns all mutable UI state for the library feature.

use gpui::{EventEmitter};
use crate::controllers::library::LibraryController;

// ── LibraryChanged event ──────────────────────────────────────────────────────

/// Emitted when any state changes that should trigger a re-render.
pub struct LibraryChanged;

impl EventEmitter<LibraryChanged> for LibraryController {}
