//! Service trait and error types for collections access.

use crate::data::collection::CollectionEntry;

/// The type of failure returned by collections operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollectionsServiceErrorKind {
    /// Request failed due to transient connectivity or SDK configuration.
    Network,
    /// Request failed due to session or authentication state.
    Session,
}

/// Error returned by collections service operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionsServiceError {
    /// The machine-classified failure kind.
    pub kind: CollectionsServiceErrorKind,
    /// Human-readable baseline error message.
    pub message: String,
}

impl CollectionsServiceError {
    /// Creates a new service error.
    pub fn new(kind: CollectionsServiceErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CollectionsServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", self.message, self.kind)
    }
}

impl std::error::Error for CollectionsServiceError {}

/// Service boundary consumed by the collections sidebar.
///
/// Implementations may be SDK-backed HTTP adapters or deterministic test stubs.
pub trait CollectionsService: Send + Sync + 'static {
    /// Loads the full list of the user's named product lists.
    ///
    /// # Errors
    ///
    /// Returns a [`CollectionsServiceError`] if the request fails or the session is invalid.
    fn list_collections(&self) -> Result<Vec<CollectionEntry>, CollectionsServiceError>;

    /// Creates a new product list with the given name.
    ///
    /// Returns the newly created [`CollectionEntry`] with an empty `member_ids` slice.
    ///
    /// # Errors
    ///
    /// Returns a [`CollectionsServiceError`] if the request fails or the session is invalid.
    fn create_collection(
        &self,
        name: &str,
    ) -> Result<CollectionEntry, CollectionsServiceError>;

    /// Deletes the product list with the given id.
    ///
    /// # Errors
    ///
    /// Returns a [`CollectionsServiceError`] if the request fails or the session is invalid.
    fn delete_collection(&self, id: u64) -> Result<(), CollectionsServiceError>;
}

#[cfg(test)]
pub mod stub {
    use std::sync::Arc;

    use super::*;

    /// Controls which canned behavior a [`CollectionsStubService`] exhibits.
    pub enum CollectionsStubMode {
        /// Returns a small seeded list of collections with member IDs.
        Seeded,
        /// Returns an empty collection list.
        Empty,
        /// Returns a session error.
        Error,
    }

    /// A deterministic in-memory [`CollectionsService`] for unit tests.
    pub struct CollectionsStubService {
        mode: CollectionsStubMode,
    }

    impl CollectionsStubService {
        /// Creates a stub that behaves according to `mode`.
        pub fn new(mode: CollectionsStubMode) -> Self {
            Self { mode }
        }
    }

    impl CollectionsService for CollectionsStubService {
        fn list_collections(&self) -> Result<Vec<CollectionEntry>, CollectionsServiceError> {
            match self.mode {
                CollectionsStubMode::Seeded => Ok(vec![CollectionEntry {
                    id: 1,
                    name: Arc::from("Favorites"),
                    member_ids: Arc::from([42u64, 99u64]),
                }]),
                CollectionsStubMode::Empty => Ok(vec![]),
                CollectionsStubMode::Error => Err(CollectionsServiceError::new(
                    CollectionsServiceErrorKind::Session,
                    "stub: simulated session error",
                )),
            }
        }

        fn create_collection(
            &self,
            name: &str,
        ) -> Result<CollectionEntry, CollectionsServiceError> {
            match self.mode {
                CollectionsStubMode::Seeded | CollectionsStubMode::Empty => {
                    Ok(CollectionEntry {
                        id: 1,
                        name: Arc::from(name),
                        member_ids: Arc::from(&[][..]),
                    })
                }
                CollectionsStubMode::Error => Err(CollectionsServiceError::new(
                    CollectionsServiceErrorKind::Session,
                    "stub: simulated session error",
                )),
            }
        }

        fn delete_collection(&self, _id: u64) -> Result<(), CollectionsServiceError> {
            match self.mode {
                CollectionsStubMode::Seeded | CollectionsStubMode::Empty => Ok(()),
                CollectionsStubMode::Error => Err(CollectionsServiceError::new(
                    CollectionsServiceErrorKind::Session,
                    "stub: simulated session error",
                )),
            }
        }
    }
}
