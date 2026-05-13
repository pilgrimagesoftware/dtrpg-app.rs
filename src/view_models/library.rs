//! Library view model for baseline Rust frontend state management.

use crate::services::{LibraryItem, LibraryService, LibraryServiceError, LibraryServiceErrorKind};

/// High-level pane state for shared baseline library behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LibraryPaneState {
    /// List or detail data is being loaded.
    Loading,
    /// List and optional detail content are ready.
    Loaded,
    /// No results are available for the current request.
    Empty,
    /// A recoverable error occurred.
    Error,
}

/// UI-facing library view model used by the app shell.
pub struct LibraryViewModel {
    service: Box<dyn LibraryService>,
    items: Vec<LibraryItem>,
    selected: Option<LibraryItem>,
    pane: LibraryPaneState,
    last_error: Option<LibraryServiceError>,
}

impl LibraryViewModel {
    /// Creates a new view model from a backend-agnostic service implementation.
    pub fn new(service: Box<dyn LibraryService>) -> Self {
        Self {
            service,
            items: Vec::new(),
            selected: None,
            pane: LibraryPaneState::Loading,
            last_error: None,
        }
    }

    /// Returns current high-level pane state.
    pub fn pane_state(&self) -> &LibraryPaneState {
        &self.pane
    }

    /// Returns the currently loaded list items.
    pub fn items(&self) -> &[LibraryItem] {
        &self.items
    }

    /// Returns selected detail item if one is loaded.
    pub fn selected(&self) -> Option<&LibraryItem> {
        self.selected.as_ref()
    }

    /// Returns the most recent recoverable error.
    pub fn last_error(&self) -> Option<&LibraryServiceError> {
        self.last_error.as_ref()
    }

    /// Loads the library list and updates pane state based on deterministic outcomes.
    pub fn load_list(&mut self) {
        self.pane = LibraryPaneState::Loading;
        self.last_error = None;

        match self.service.list_items() {
            Ok(items) if items.is_empty() => {
                self.items = Vec::new();
                self.selected = None;
                self.pane = LibraryPaneState::Empty;
            }
            Ok(items) => {
                self.items = items;
                self.selected = None;
                self.pane = LibraryPaneState::Loaded;
            }
            Err(error) => {
                self.items = Vec::new();
                self.selected = None;
                self.last_error = Some(error);
                self.pane = LibraryPaneState::Error;
            }
        }
    }

    /// Reloads list data using the same baseline behavior as initial load.
    pub fn refresh(&mut self) {
        self.load_list();
    }

    /// Loads detail for the selected item and updates error or loaded behavior.
    pub fn select_item(&mut self, id: u64) {
        self.last_error = None;

        match self.service.get_item(id) {
            Ok(item) => {
                self.selected = Some(item);
                self.pane = if self.items.is_empty() {
                    LibraryPaneState::Loaded
                } else {
                    self.pane.clone()
                };
            }
            Err(error) => {
                self.selected = None;
                self.last_error = Some(error);
                self.pane = match self
                    .last_error
                    .as_ref()
                    .map(|e| e.kind)
                    .unwrap_or(LibraryServiceErrorKind::Network)
                {
                    LibraryServiceErrorKind::NotFound if !self.items.is_empty() => {
                        LibraryPaneState::Loaded
                    }
                    _ => LibraryPaneState::Error,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::stub::{StubLibraryService, StubMode};

    #[test]
    fn load_list_enters_loaded_with_seeded_stub_data() {
        let mut vm = LibraryViewModel::new(Box::new(StubLibraryService::new(StubMode::Seeded)));

        vm.load_list();

        assert_eq!(vm.pane_state(), &LibraryPaneState::Loaded);
        assert!(!vm.items().is_empty());
        assert!(vm.last_error().is_none());
    }

    #[test]
    fn load_list_enters_empty_with_empty_stub_mode() {
        let mut vm = LibraryViewModel::new(Box::new(StubLibraryService::new(StubMode::Empty)));

        vm.load_list();

        assert_eq!(vm.pane_state(), &LibraryPaneState::Empty);
        assert!(vm.items().is_empty());
        assert!(vm.last_error().is_none());
    }

    #[test]
    fn load_list_enters_error_with_network_stub_mode() {
        let mut vm =
            LibraryViewModel::new(Box::new(StubLibraryService::new(StubMode::NetworkError)));

        vm.load_list();

        assert_eq!(vm.pane_state(), &LibraryPaneState::Error);
        assert!(vm.items().is_empty());
        assert!(vm.last_error().is_some());
        assert_eq!(
            vm.last_error().map(|e| e.kind),
            Some(LibraryServiceErrorKind::Network)
        );
    }

    #[test]
    fn select_item_loads_detail_after_list_load() {
        let mut vm = LibraryViewModel::new(Box::new(StubLibraryService::new(StubMode::Seeded)));
        vm.load_list();

        let id = vm.items()[0].id;
        vm.select_item(id);

        assert_eq!(vm.pane_state(), &LibraryPaneState::Loaded);
        assert_eq!(vm.selected().map(|i| i.id), Some(id));
        assert!(vm.last_error().is_none());
    }
}
