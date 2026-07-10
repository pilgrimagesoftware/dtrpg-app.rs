//! Rust SDK-backed implementations of the app's service traits.
//!
//! Organized by domain (`library`, `collections`), each a self-contained
//! gateway/service/error trio. `connection` holds what's genuinely shared
//! between them: both domains are backed by the same underlying
//! [`dtrpg_sdk::LibraryClient`] and go through an identical
//! credential-resolution and connection-setup sequence.

pub mod collections;
mod connection;
pub mod library;

pub use self::collections::RustSdkCollectionsService;
pub use self::library::RustSdkLibraryService;
