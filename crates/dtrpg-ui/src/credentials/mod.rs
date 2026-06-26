//! Secure credential storage abstraction over platform-native keyring.
//!
//! Use [`KeyringCredentialStore`] to store, retrieve, and delete DriveThruRPG
//! credentials. All entries are namespaced under [`keys::SERVICE`].
//!
//! # Example
//!
//! ```rust,no_run
//! use dtrpg_ui::credentials::{KeyringCredentialStore, CredentialStore, Credential, keys};
//!
//! let store = KeyringCredentialStore::new(keys::SERVICE, keys::API_KEY);
//! let credential = Credential {
//!     service: keys::SERVICE.into(),
//!     account: keys::API_KEY.into(),
//!     secret: "my-api-key".into(),
//! };
//! store.store(&credential).expect("failed to store credential");
//! let loaded = store.load().expect("failed to load credential");
//! store.delete().expect("failed to delete credential");
//! ```

pub mod keys;
mod model;
mod store;

pub use model::{Credential, CredentialError};
pub use store::{CredentialStore, KeyringCredentialStore};
