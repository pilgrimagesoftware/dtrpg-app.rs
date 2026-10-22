//! Keyring storage for the user's own Anthropic API key, used by the
//! content-summary feature ([`crate::data::summary_settings`]).
//!
//! The key lives in its own service namespace
//! ([`KEYRING_ANTHROPIC_SERVICE`]), apart from the DriveThruRPG credentials.

use super::{Credential, CredentialError, CredentialStore, KeyringCredentialStore};
use crate::data::constants::{
    ANTHROPIC_API_KEY_ENV, KEYRING_ANTHROPIC_API_KEY, KEYRING_ANTHROPIC_SERVICE,
};

fn store() -> KeyringCredentialStore {
    KeyringCredentialStore::new(KEYRING_ANTHROPIC_SERVICE, KEYRING_ANTHROPIC_API_KEY)
}

/// Saves `api_key` as the Anthropic API key, replacing any existing one.
///
/// # Errors
///
/// Returns [`CredentialError`] if the platform keyring rejects the write.
pub fn save_anthropic_api_key(api_key: &str) -> Result<(), CredentialError> {
    store().store(&Credential { service: KEYRING_ANTHROPIC_SERVICE.into(),
                                account: KEYRING_ANTHROPIC_API_KEY.into(),
                                secret:  api_key.into(),
                                email:   None, })
}

/// Loads the saved Anthropic API key, or `None` if none has been saved.
///
/// # Errors
///
/// Returns [`CredentialError`] if the platform keyring cannot be read.
pub fn load_anthropic_api_key() -> Result<Option<String>, CredentialError> {
    Ok(store().load()?.map(|credential| credential.secret))
}

/// Removes the saved Anthropic API key, if there is one.
///
/// # Errors
///
/// Returns [`CredentialError`] if the platform keyring rejects the delete.
pub fn delete_anthropic_api_key() -> Result<(), CredentialError> {
    store().delete()
}

/// Where the Anthropic API key can come from right now.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AnthropicKeyStatus {
    /// The `ANTHROPIC_API_KEY` environment variable is set to something.
    pub env_var: bool,
    /// A key is saved in the keychain.
    pub saved:   bool,
}

/// The key set in the environment, if there is one. An empty value counts as
/// none.
fn key_from_env() -> Option<String> {
    std::env::var(ANTHROPIC_API_KEY_ENV).ok()
                                        .map(|key| key.trim().to_owned())
                                        .filter(|key| !key.is_empty())
}

/// The key to use: the one saved in the keychain if there is one, otherwise
/// the one in the environment, or `None` if there is neither. A key the person
/// saved in Libri wins over the environment variable.
///
/// # Errors
///
/// Returns [`CredentialError`] if the platform keyring cannot be read.
pub fn resolve_anthropic_api_key() -> Result<Option<String>, CredentialError> {
    Ok(load_anthropic_api_key()?.or_else(key_from_env))
}

/// Where a key can come from, for the settings page to say so. A keychain that
/// cannot be read counts as no saved key.
#[must_use]
pub fn anthropic_key_status() -> AnthropicKeyStatus {
    AnthropicKeyStatus { env_var: key_from_env().is_some(),
                         saved:   matches!(load_anthropic_api_key(), Ok(Some(_))), }
}
