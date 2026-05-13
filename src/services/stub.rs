//! In-memory stub implementation of [`LibraryService`] for baseline UI behavior.

use super::{LibraryItem, LibraryService, LibraryServiceError, LibraryServiceErrorKind};

/// Deterministic stub mode for validating view-state transitions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StubMode {
    /// Returns a seeded list of items and item details.
    Seeded,
    /// Returns an empty list to exercise the empty-state path.
    Empty,
    /// Returns a transient network-style failure for list/detail operations.
    NetworkError,
    /// Returns a session-related failure for list/detail operations.
    SessionError,
}

/// Stubbed implementation of [`LibraryService`] used during baseline phase.
#[derive(Clone, Debug)]
pub struct StubLibraryService {
    mode: StubMode,
    items: Vec<LibraryItem>,
}

impl StubLibraryService {
    /// Creates a stub service in the given deterministic mode.
    pub fn new(mode: StubMode) -> Self {
        Self {
            mode,
            items: seeded_items(),
        }
    }

    fn mode_error(&self) -> Option<LibraryServiceError> {
        match self.mode {
            StubMode::NetworkError => Some(LibraryServiceError::new(
                LibraryServiceErrorKind::Network,
                "Unable to load library data in baseline stub mode.",
            )),
            StubMode::SessionError => Some(LibraryServiceError::new(
                LibraryServiceErrorKind::Session,
                "Your session is no longer valid in baseline stub mode.",
            )),
            _ => None,
        }
    }
}

impl LibraryService for StubLibraryService {
    fn list_items(&self) -> Result<Vec<LibraryItem>, LibraryServiceError> {
        if let Some(err) = self.mode_error() {
            return Err(err);
        }

        match self.mode {
            StubMode::Empty => Ok(Vec::new()),
            _ => Ok(self.items.clone()),
        }
    }

    fn get_item(&self, id: u64) -> Result<LibraryItem, LibraryServiceError> {
        if let Some(err) = self.mode_error() {
            return Err(err);
        }

        self.items
            .iter()
            .find(|item| item.id == id)
            .cloned()
            .ok_or_else(|| {
                LibraryServiceError::new(
                    LibraryServiceErrorKind::NotFound,
                    format!("Item with id {id} was not found."),
                )
            })
    }
}

fn seeded_items() -> Vec<LibraryItem> {
    vec![
        LibraryItem {
            id: 170_405_504,
            title: "Sandbox Generator".to_string(),
            publisher: "Tabletop Adventures".to_string(),
            summary: "A toolkit for quickly building sandbox campaign seeds.".to_string(),
        },
        LibraryItem {
            id: 190_222_010,
            title: "Urban Encounters Compendium".to_string(),
            publisher: "Silver Lantern Games".to_string(),
            summary: "Drop-in city encounters with hooks and consequences.".to_string(),
        },
        LibraryItem {
            id: 215_777_901,
            title: "Atlas of the Broken Coast".to_string(),
            publisher: "Northwind Publishing".to_string(),
            summary: "Regional maps and lore for coastal sandbox campaigns.".to_string(),
        },
    ]
}
