// //! Service boundaries for library behavior.
// //!
// //! This module defines backend-agnostic service interfaces and entities used by the
// //! Rust frontend. Implementations can be deterministic test stubs or SDK-backed adapters.

// pub mod sdk;

// use core::fmt;

// /// A simplified library item used by list and detail presentation.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct LibraryItem {
//     /// Stable identifier for selection and detail loading.
//     pub id: u64,
//     /// User-facing title shown in list and detail pane.
//     pub title: String,
//     /// Product publisher shown as metadata.
//     pub publisher: String,
//     /// Product type used by alternative grouping modes.
//     pub product_type: String,
//     /// Relative insertion order used by "most recently added" sorting.
//     pub added_order: u32,
//     /// Relative update order used by "most recently updated" sorting.
//     pub updated_order: u32,
//     /// Short summary text shown in detail pane.
//     pub summary: String,
// }

// /// The type of service failure returned by library operations.
// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// pub enum LibraryServiceErrorKind {
//     /// Request failed due to transient connectivity or SDK configuration behavior.
//     Network,
//     /// Request failed due to session/auth state.
//     Session,
//     /// Request referenced a non-existent item.
//     NotFound,
// }

// /// Error returned by library service operations.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct LibraryServiceError {
//     /// The machine-classified failure kind.
//     pub kind: LibraryServiceErrorKind,
//     /// Human-readable baseline error message.
//     pub message: String,
// }

// impl LibraryServiceError {
//     /// Creates a new service error.
//     pub fn new(kind: LibraryServiceErrorKind, message: impl Into<String>) -> Self {
//         Self {
//             kind,
//             message: message.into(),
//         }
//     }
// }

// impl fmt::Display for LibraryServiceError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} ({:?})", self.message, self.kind)
//     }
// }

// impl std::error::Error for LibraryServiceError {}

// /// Service boundary consumed by the baseline library view model.
// pub trait LibraryService {
//     /// Loads the full set of list items for the current filter/query state.
//     fn list_items(&self) -> Result<Vec<LibraryItem>, LibraryServiceError>;

//     /// Loads detail data for a selected item.
//     fn get_item(&self, id: u64) -> Result<LibraryItem, LibraryServiceError>;
// }
