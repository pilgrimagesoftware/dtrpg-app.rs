//! `CredentialStore` trait and `KeyringCredentialStore` implementation.

use keyring::Entry;

use super::model::{Credential, CredentialError};

// ── Trait ─────────────────────────────────────────────────────────────────────

/// Platform-agnostic interface for storing, retrieving, and deleting a
/// single credential entry.
///
/// Each `CredentialStore` instance is bound to one `(service, account)` pair.
/// Use separate instances for each credential type (API key, access token,
/// refresh token).
///
/// # Errors
///
/// Methods return [`CredentialError`] when the underlying platform store
/// fails. Callers MUST surface these errors to the user and MUST NOT fall
/// back to plaintext storage.
pub trait CredentialStore: Send + Sync {
    /// Writes `credential.secret` to the platform store.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::Store`] if the write fails, or
    /// [`CredentialError::Unavailable`] if the platform store is not
    /// accessible.
    fn store(&self, credential: &Credential) -> Result<(), CredentialError>;

    /// Reads the credential from the platform store.
    ///
    /// Returns `Ok(None)` when no entry exists yet (first run / after sign-out).
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::Load`] on read failure, or
    /// [`CredentialError::Unavailable`] if the platform store is inaccessible.
    fn load(&self) -> Result<Option<Credential>, CredentialError>;

    /// Removes the credential from the platform store.
    ///
    /// Succeeds silently if no entry exists.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::Delete`] on unexpected failure.
    fn delete(&self) -> Result<(), CredentialError>;
}

// ── KeyringCredentialStore ────────────────────────────────────────────────────

/// [`CredentialStore`] backed by the platform native keyring via the `keyring`
/// crate (macOS Keychain, Windows Credential Manager, Linux Secret Service).
pub struct KeyringCredentialStore {
    service: String,
    account: String,
}

impl KeyringCredentialStore {
    /// Creates a store bound to the given `service` and `account` pair.
    ///
    /// Use the constants in [`crate::credentials::keys`] for both arguments.
    pub fn new(service: impl Into<String>, account: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            account: account.into(),
        }
    }

    fn entry(&self) -> Result<Entry, CredentialError> {
        Entry::new(&self.service, &self.account)
            .map_err(|e| CredentialError::Unavailable(e.to_string()))
    }
}

impl CredentialStore for KeyringCredentialStore {
    fn store(&self, credential: &Credential) -> Result<(), CredentialError> {
        self.entry()?
            .set_password(&credential.secret)
            .map_err(|e| CredentialError::Store {
                account: self.account.clone(),
                reason: e.to_string(),
            })
    }

    fn load(&self) -> Result<Option<Credential>, CredentialError> {
        match self.entry()?.get_password() {
            Ok(secret) => Ok(Some(Credential {
                service: self.service.clone(),
                account: self.account.clone(),
                secret,
            })),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(CredentialError::Load {
                account: self.account.clone(),
                reason: e.to_string(),
            }),
        }
    }

    fn delete(&self) -> Result<(), CredentialError> {
        match self.entry()?.delete_credential() {
            Ok(()) => Ok(()),
            // Already gone — treat as success.
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(CredentialError::Delete {
                account: self.account.clone(),
                reason: e.to_string(),
            }),
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::data::constants::{KEYRING_SERVICE, KEYRING_API_KEY};
    use std::sync::{Arc, Mutex};

    // ── Mock store for unit testing call-site behavior ────────────────────────

    #[derive(Default)]
    struct MockCredentialStore {
        stored: Arc<Mutex<Option<String>>>,
        store_error: bool,
        load_error: bool,
        delete_error: bool,
    }

    impl CredentialStore for MockCredentialStore {
        fn store(&self, credential: &Credential) -> Result<(), CredentialError> {
            if self.store_error {
                return Err(CredentialError::Store {
                    account: "mock".into(),
                    reason: "injected error".into(),
                });
            }
            *self
                .stored
                .lock()
                .map_err(|_| CredentialError::Unavailable("lock poisoned".into()))? =
                Some(credential.secret.clone());
            Ok(())
        }

        fn load(&self) -> Result<Option<Credential>, CredentialError> {
            if self.load_error {
                return Err(CredentialError::Load {
                    account: "mock".into(),
                    reason: "injected error".into(),
                });
            }
            let guard = self
                .stored
                .lock()
                .map_err(|_| CredentialError::Unavailable("lock poisoned".into()))?;
            Ok(guard.as_ref().map(|secret| Credential {
                service: KEYRING_SERVICE.into(),
                account: KEYRING_API_KEY.into(),
                secret: secret.clone(),
            }))
        }

        fn delete(&self) -> Result<(), CredentialError> {
            if self.delete_error {
                return Err(CredentialError::Delete {
                    account: "mock".into(),
                    reason: "injected error".into(),
                });
            }
            *self
                .stored
                .lock()
                .map_err(|_| CredentialError::Unavailable("lock poisoned".into()))? = None;
            Ok(())
        }
    }

    fn make_credential(secret: &str) -> Credential {
        Credential {
            service: KEYRING_SERVICE.into(),
            account: KEYRING_API_KEY.into(),
            secret: secret.into(),
        }
    }

    #[test]
    fn mock_store_then_load_returns_secret() {
        let store = MockCredentialStore::default();
        store
            .store(&make_credential("test-secret"))
            .expect("store should succeed");
        let loaded = store.load().expect("load should succeed");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().secret, "test-secret");
    }

    #[test]
    fn mock_load_before_store_returns_none() {
        let store = MockCredentialStore::default();
        let loaded = store.load().expect("load should succeed");
        assert!(loaded.is_none());
    }

    #[test]
    fn mock_delete_clears_stored_credential() {
        let store = MockCredentialStore::default();
        store
            .store(&make_credential("secret"))
            .expect("store should succeed");
        store.delete().expect("delete should succeed");
        let loaded = store.load().expect("load should succeed");
        assert!(loaded.is_none());
    }

    #[test]
    fn mock_delete_on_empty_store_succeeds() {
        let store = MockCredentialStore::default();
        // delete with nothing stored should not error
        store
            .delete()
            .expect("delete on empty store should succeed");
    }

    #[test]
    fn mock_store_error_is_propagated() {
        let store = MockCredentialStore {
            store_error: true,
            ..Default::default()
        };
        let result = store.store(&make_credential("s"));
        assert!(result.is_err());
        assert!(matches!(result, Err(CredentialError::Store { .. })));
    }

    #[test]
    fn mock_load_error_is_propagated() {
        let store = MockCredentialStore {
            load_error: true,
            ..Default::default()
        };
        let result = store.load();
        assert!(result.is_err());
        assert!(matches!(result, Err(CredentialError::Load { .. })));
    }

    #[test]
    fn credential_debug_redacts_secret() {
        let cred = make_credential("my-super-secret");
        let debug = format!("{cred:?}");
        assert!(!debug.contains("my-super-secret"));
        assert!(debug.contains("[redacted]"));
    }
}
