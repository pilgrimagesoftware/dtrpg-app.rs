//! Service trait and error types for library data access.

use crate::data::library::LibraryItem;

/// The type of service failure returned by library operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibraryServiceErrorKind {
    /// Request failed due to transient connectivity or SDK configuration.
    Network,
    /// Request failed due to session or authentication state.
    Session,
    /// Request referenced a non-existent item.
    NotFound,
}

/// Error returned by library service operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryServiceError {
    /// The machine-classified failure kind.
    pub kind: LibraryServiceErrorKind,
    /// Human-readable baseline error message.
    pub message: String,
}

impl LibraryServiceError {
    /// Creates a new service error.
    pub fn new(kind: LibraryServiceErrorKind, message: impl Into<String>) -> Self {
        Self { kind, message: message.into() }
    }
}

impl std::fmt::Display for LibraryServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", self.message, self.kind)
    }
}

impl std::error::Error for LibraryServiceError {}

/// Service boundary consumed by the library view model.
///
/// Implementations may be SDK-backed HTTP adapters or deterministic test stubs.
/// The service is responsible for fetching and mapping all data; callers receive
/// fully-formed [`LibraryItem`] values ready for presentation.
pub trait LibraryService: Send + Sync + 'static {
    /// Loads the full library item list.
    ///
    /// # Errors
    ///
    /// Returns a [`LibraryServiceError`] if the request fails or the session is invalid.
    fn list_items(&self) -> Result<Vec<LibraryItem>, LibraryServiceError>;

    /// Loads detail data for a selected item by its numeric API identifier.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryServiceError`] with kind [`LibraryServiceErrorKind::NotFound`]
    /// if the id does not match any item, or a network/session error if the request fails.
    fn get_item(&self, id: u64) -> Result<LibraryItem, LibraryServiceError>;
}

#[cfg(test)]
pub mod stub;
