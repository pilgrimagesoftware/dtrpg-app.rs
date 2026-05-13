//! Baseline app shell state and command routing.

use crate::services::{LibraryItem, LibraryServiceErrorKind};
use crate::view_models::library::{LibraryPaneState, LibraryViewModel};

/// Session presentation state used by the shell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SessionPresentationState {
    /// User is signed out.
    SignedOut,
    /// Session is being restored.
    Restoring,
    /// User has an active authenticated session.
    SignedIn,
    /// Session recovery is required before continuing.
    NeedsRecovery,
}

/// Top-level shell state for baseline UI flow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppShellState {
    /// Session presentation state for top-level shell routing.
    pub session: SessionPresentationState,
    /// Current library pane state.
    pub library: LibraryPaneState,
    /// Selected library item id if a detail pane is active.
    pub selected_item_id: Option<u64>,
    /// Status message shown in shell-level status area.
    pub status_message: String,
}

/// App-level commands used for baseline flow transitions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppCommand {
    /// Load list data for the first time.
    LoadLibrary,
    /// Reload list data while preserving shell flow.
    RefreshLibrary,
    /// Select an item and load detail.
    SelectLibraryItem(u64),
    /// Clear current selection.
    ClearSelection,
}

/// Root shell object coordinating command routing and view model updates.
pub struct AppShell {
    state: AppShellState,
    library_vm: LibraryViewModel,
}

impl AppShell {
    /// Creates a new shell from initial state and a library view model.
    pub fn new(initial_state: AppShellState, library_vm: LibraryViewModel) -> Self {
        Self {
            state: initial_state,
            library_vm,
        }
    }

    /// Returns immutable shell state for rendering.
    pub fn state(&self) -> &AppShellState {
        &self.state
    }

    /// Returns first item id if any items are loaded.
    pub fn first_item_id(&self) -> Option<u64> {
        self.library_vm.items().first().map(|item| item.id)
    }

    /// Returns loaded library items for list presentation.
    pub fn items(&self) -> &[LibraryItem] {
        self.library_vm.items()
    }

    /// Returns the selected detail item if one is active.
    pub fn selected_item(&self) -> Option<&LibraryItem> {
        self.library_vm.selected()
    }

    /// Routes a shell command and synchronizes shell state from view model changes.
    pub fn dispatch(&mut self, command: AppCommand) {
        match command {
            AppCommand::LoadLibrary => {
                self.library_vm.load_list();
                self.sync_from_library("Library loaded in baseline stub mode.");
            }
            AppCommand::RefreshLibrary => {
                self.library_vm.refresh();
                self.sync_from_library("Library refreshed in baseline stub mode.");
            }
            AppCommand::SelectLibraryItem(id) => {
                self.library_vm.select_item(id);
                self.state.selected_item_id = self.library_vm.selected().map(|item| item.id);
                self.sync_from_library("Library item selected in baseline stub mode.");
            }
            AppCommand::ClearSelection => {
                self.state.selected_item_id = None;
                self.state.status_message = "Selection cleared.".to_string();
            }
        }
    }

    fn sync_from_library(&mut self, loaded_message: &str) {
        self.state.library = self.library_vm.pane_state().clone();

        match self.library_vm.pane_state() {
            LibraryPaneState::Loading => {
                self.state.status_message = "Loading your library…".to_string();
            }
            LibraryPaneState::Loaded => {
                self.state.status_message = loaded_message.to_string();
            }
            LibraryPaneState::Empty => {
                self.state.status_message = "No library items found in baseline stub mode.".to_string();
            }
            LibraryPaneState::Error => {
                self.state.status_message = self
                    .library_vm
                    .last_error()
                    .map(|e| match e.kind {
                        LibraryServiceErrorKind::Network => {
                            "Library failed to load. Retry is available.".to_string()
                        }
                        LibraryServiceErrorKind::Session => {
                            self.state.session = SessionPresentationState::NeedsRecovery;
                            "Session recovery required before loading library.".to_string()
                        }
                        LibraryServiceErrorKind::NotFound => {
                            "Selected item is no longer available.".to_string()
                        }
                    })
                    .unwrap_or_else(|| "Unknown library error.".to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::stub::{StubLibraryService, StubMode};

    fn make_shell(mode: StubMode) -> AppShell {
        let library_vm = LibraryViewModel::new(Box::new(StubLibraryService::new(mode)));

        AppShell::new(
            AppShellState {
                session: SessionPresentationState::SignedIn,
                library: LibraryPaneState::Loading,
                selected_item_id: None,
                status_message: "Loading your library…".to_string(),
            },
            library_vm,
        )
    }

    #[test]
    fn load_command_sets_loaded_state_for_seeded_mode() {
        let mut shell = make_shell(StubMode::Seeded);

        shell.dispatch(AppCommand::LoadLibrary);

        assert_eq!(shell.state().library, LibraryPaneState::Loaded);
        assert_eq!(
            shell.state().status_message,
            "Library loaded in baseline stub mode."
        );
    }

    #[test]
    fn load_command_sets_empty_state_for_empty_mode() {
        let mut shell = make_shell(StubMode::Empty);

        shell.dispatch(AppCommand::LoadLibrary);

        assert_eq!(shell.state().library, LibraryPaneState::Empty);
        assert_eq!(
            shell.state().status_message,
            "No library items found in baseline stub mode."
        );
    }

    #[test]
    fn load_command_sets_recovery_state_for_session_error_mode() {
        let mut shell = make_shell(StubMode::SessionError);

        shell.dispatch(AppCommand::LoadLibrary);

        assert_eq!(shell.state().library, LibraryPaneState::Error);
        assert_eq!(shell.state().session, SessionPresentationState::NeedsRecovery);
        assert_eq!(
            shell.state().status_message,
            "Session recovery required before loading library."
        );
    }
}
