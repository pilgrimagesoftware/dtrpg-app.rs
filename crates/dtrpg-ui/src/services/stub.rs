//! Test stubs for the library service.

use crate::data::library::LibraryItem;
use crate::services::{LibraryService, LibraryServiceError, LibraryServiceErrorKind};
use crate::util::stubs::stub_catalog;

/// Controls which canned behavior a [`StubLibraryService`] exhibits.
#[derive(Clone, Copy, Debug)]
pub enum StubMode {
    /// Returns a seeded set of fake library items.
    Seeded,
    /// Returns an empty item list.
    Empty,
    /// Returns a network error.
    NetworkError,
    /// Returns a session error.
    SessionError,
}

/// A deterministic in-memory [`LibraryService`] for unit tests.
pub struct StubLibraryService {
    mode: StubMode,
}

impl StubLibraryService {
    /// Creates a stub that behaves according to `mode`.
    pub fn new(mode: StubMode) -> Self {
        Self { mode }
    }
}

impl LibraryService for StubLibraryService {
    fn list_items(&self) -> Result<Vec<LibraryItem>, LibraryServiceError> {
        match self.mode {
            StubMode::Seeded => Ok(stub_catalog()),
            StubMode::Empty => Ok(vec![]),
            StubMode::NetworkError => Err(LibraryServiceError::new(
                LibraryServiceErrorKind::Network,
                "stub: simulated network error",
            )),
            StubMode::SessionError => Err(LibraryServiceError::new(
                LibraryServiceErrorKind::Session,
                "stub: simulated session error",
            )),
        }
    }

    fn get_item(&self, _id: u64) -> Result<LibraryItem, LibraryServiceError> {
        match self.mode {
            StubMode::Seeded => stub_catalog().into_iter().next().ok_or_else(|| {
                LibraryServiceError::new(LibraryServiceErrorKind::NotFound, "stub: no items")
            }),
            StubMode::Empty => Err(LibraryServiceError::new(
                LibraryServiceErrorKind::NotFound,
                "stub: no items in empty mode",
            )),
            StubMode::NetworkError => Err(LibraryServiceError::new(
                LibraryServiceErrorKind::Network,
                "stub: simulated network error",
            )),
            StubMode::SessionError => Err(LibraryServiceError::new(
                LibraryServiceErrorKind::Session,
                "stub: simulated session error",
            )),
        }
    }
}
