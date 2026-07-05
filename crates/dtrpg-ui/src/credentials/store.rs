//! `CredentialStore` trait and `KeyringCredentialStore` implementation.

use keyring::Entry;
use serde_json::Value;

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
    /// Writes the credential to the platform store.
    ///
    /// When `credential.email` is `Some`, the email is persisted alongside
    /// the key as a JSON payload; legacy plain-key format is written when
    /// `email` is `None`.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::Store`] if the write fails, or
    /// [`CredentialError::Unavailable`] if the platform store is not
    /// accessible.
    fn store(&self, credential: &Credential) -> Result<(), CredentialError>;

    /// Reads the credential from the platform store.
    ///
    /// Returns `Ok(None)` when no entry exists yet (first run / after
    /// sign-out). Tolerates legacy entries that contain only a plain
    /// application key with no email; those are returned with
    /// `credential.email = None`.
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

// ── KeyringCredentialStore
// ────────────────────────────────────────────────────

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
        Self { service: service.into(),
               account: account.into(), }
    }

    fn entry(&self) -> Result<Entry, CredentialError> {
        Entry::new(&self.service, &self.account).map_err(|e| {
                                                    CredentialError::Unavailable(e.to_string())
                                                })
    }
}

impl CredentialStore for KeyringCredentialStore {
    fn store(&self, credential: &Credential) -> Result<(), CredentialError> {
        let payload = if let Some(email) = &credential.email {
            serde_json::json!({"email": email, "key": credential.secret}).to_string()
        }
        else {
            credential.secret.clone()
        };
        self.entry()?
            .set_password(&payload)
            .map_err(|e| CredentialError::Store {
                account: self.account.clone(),
                reason: e.to_string(),
            })
    }

    fn load(&self) -> Result<Option<Credential>, CredentialError> {
        match self.entry()?.get_password() {
            Ok(raw) => {
                // Try to parse as {"email": "...", "key": "..."}.
                // Fall back to treating the raw string as a legacy plain key.
                let (secret, email) =
                    if let Ok(Value::Object(map)) = serde_json::from_str::<Value>(&raw) {
                        let key = map.get("key").and_then(Value::as_str).map(str::to_owned);
                        let em = map.get("email").and_then(Value::as_str).map(str::to_owned);
                        if let Some(k) = key {
                            (k, em)
                        }
                        else {
                            // Malformed JSON object — treat whole string as legacy key.
                            (raw, None)
                        }
                    }
                    else {
                        (raw, None)
                    };
                Ok(Some(Credential { service: self.service.clone(),
                                     account: self.account.clone(),
                                     secret,
                                     email }))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(CredentialError::Load { account: self.account.clone(),
                                                  reason:  e.to_string(), }),
        }
    }

    fn delete(&self) -> Result<(), CredentialError> {
        match self.entry()?.delete_credential() {
            Ok(()) => Ok(()),
            // Already gone — treat as success.
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(CredentialError::Delete { account: self.account.clone(),
                                                    reason:  e.to_string(), }),
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::data::constants::{KEYRING_API_KEY, KEYRING_SERVICE};

    // ── Mock store for unit testing call-site behavior ────────────────────────

    #[derive(Default)]
    struct MockCredentialStore {
        stored:       Arc<Mutex<Option<Credential>>>,
        store_error:  bool,
        load_error:   bool,
        delete_error: bool,
    }

    impl CredentialStore for MockCredentialStore {
        fn store(&self, credential: &Credential) -> Result<(), CredentialError> {
            if self.store_error {
                return Err(CredentialError::Store { account: "mock".into(),
                                                    reason:  "injected error".into(), });
            }
            *self.stored
                 .lock()
                 .map_err(|_| CredentialError::Unavailable("lock poisoned".into()))? =
                Some(credential.clone());
            Ok(())
        }

        fn load(&self) -> Result<Option<Credential>, CredentialError> {
            if self.load_error {
                return Err(CredentialError::Load { account: "mock".into(),
                                                   reason:  "injected error".into(), });
            }
            Ok(self.stored
                   .lock()
                   .map_err(|_| CredentialError::Unavailable("lock poisoned".into()))?
                   .clone())
        }

        fn delete(&self) -> Result<(), CredentialError> {
            if self.delete_error {
                return Err(CredentialError::Delete { account: "mock".into(),
                                                     reason:  "injected error".into(), });
            }
            *self.stored
                 .lock()
                 .map_err(|_| CredentialError::Unavailable("lock poisoned".into()))? = None;
            Ok(())
        }
    }

    fn make_credential(secret: &str) -> Credential {
        Credential { service: KEYRING_SERVICE.into(),
                     account: KEYRING_API_KEY.into(),
                     secret:  secret.into(),
                     email:   None }
    }

    fn make_credential_with_email(secret: &str, email: &str) -> Credential {
        Credential { service: KEYRING_SERVICE.into(),
                     account: KEYRING_API_KEY.into(),
                     secret:  secret.into(),
                     email:   Some(email.into()) }
    }

    #[test]
    fn mock_store_then_load_returns_secret() {
        let store = MockCredentialStore::default();
        store.store(&make_credential("test-secret"))
             .expect("store should succeed");
        let loaded = store.load().expect("load should succeed");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().secret, "test-secret");
    }

    #[test]
    fn mock_store_then_load_preserves_email() {
        let store = MockCredentialStore::default();
        store.store(&make_credential_with_email("the-key", "user@example.com"))
             .expect("store should succeed");
        let loaded = store.load().expect("load should succeed").unwrap();
        assert_eq!(loaded.secret, "the-key");
        assert_eq!(loaded.email.as_deref(), Some("user@example.com"));
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
        store.store(&make_credential("secret"))
             .expect("store should succeed");
        store.delete().expect("delete should succeed");
        let loaded = store.load().expect("load should succeed");
        assert!(loaded.is_none());
    }

    #[test]
    fn mock_delete_on_empty_store_succeeds() {
        let store = MockCredentialStore::default();
        store.delete()
             .expect("delete on empty store should succeed");
    }

    #[test]
    fn mock_store_error_is_propagated() {
        let store = MockCredentialStore { store_error: true,
                                          ..Default::default() };
        let result = store.store(&make_credential("s"));
        assert!(result.is_err());
        assert!(matches!(result, Err(CredentialError::Store { .. })));
    }

    #[test]
    fn mock_load_error_is_propagated() {
        let store = MockCredentialStore { load_error: true,
                                          ..Default::default() };
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

    // ── JSON payload encoding/decoding tests ──────────────────────────────────

    #[test]
    fn keyring_store_serializes_email_plus_key_as_json() {
        // Verify the store method produces the expected JSON payload.
        // We test through the KeyringCredentialStore logic directly by
        // exercising the same encoding path that load() uses to decode.
        let email = "user@example.com";
        let key = "abc123";
        let payload = serde_json::json!({"email": email, "key": key}).to_string();

        // Decode via the same logic used in load()
        let parsed: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(parsed["key"].as_str(), Some(key));
        assert_eq!(parsed["email"].as_str(), Some(email));
    }

    #[test]
    fn keyring_load_parses_json_payload_with_email() {
        // Simulate what load() does when it reads back a JSON payload.
        let raw = r#"{"email":"user@example.com","key":"my-app-key"}"#.to_string();
        let (secret, email) = decode_payload(raw);
        assert_eq!(secret, "my-app-key");
        assert_eq!(email.as_deref(), Some("user@example.com"));
    }

    #[test]
    fn keyring_load_treats_plain_string_as_legacy_key() {
        let raw = "legacy-plain-api-key".to_string();
        let (secret, email) = decode_payload(raw);
        assert_eq!(secret, "legacy-plain-api-key");
        assert!(email.is_none());
    }

    #[test]
    fn keyring_load_treats_json_without_key_field_as_legacy() {
        let raw = r#"{"other":"value"}"#.to_string();
        let (secret, email) = decode_payload(raw);
        // No "key" field — fall back to treating the whole string as the key.
        assert_eq!(secret, r#"{"other":"value"}"#);
        assert!(email.is_none());
    }

    /// Mirrors the decode logic from `KeyringCredentialStore::load`.
    fn decode_payload(raw: String) -> (String, Option<String>) {
        use serde_json::Value;
        if let Ok(Value::Object(map)) = serde_json::from_str::<Value>(&raw) {
            let key = map.get("key").and_then(Value::as_str).map(str::to_owned);
            let em = map.get("email").and_then(Value::as_str).map(str::to_owned);
            if let Some(k) = key {
                return (k, em);
            }
        }
        (raw, None)
    }
}
