//! User profile config: optional email address persisted for avatar lookup.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const APP_NAME: &str = "dtrpg";

#[derive(Serialize, Deserialize, Default)]
struct ProfileConfigFile {
    email: Option<String>,
}

/// Reads and writes `{config_dir}/dtrpg/profile.toml`.
///
/// Contains user-preference data (email for Gravatar) that is not a credential
/// and therefore belongs in the config directory rather than the keyring.
pub struct ProfileConfig {
    email: Option<String>,
}

impl ProfileConfig {
    fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join(APP_NAME).join("profile.toml"))
    }

    /// Loads the profile from disk, returning defaults on any error.
    pub fn load() -> Self {
        let email = Self::path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| toml::from_str::<ProfileConfigFile>(&s).ok())
            .and_then(|f| f.email)
            .filter(|s| !s.trim().is_empty());

        Self { email }
    }

    /// Returns the stored email address, if any.
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// Saves `email` to disk.
    ///
    /// Passing `None` clears the stored value. Write failures are logged as warnings.
    pub fn save(email: Option<&str>) {
        let Some(path) = Self::path() else {
            tracing::warn!("profile: config_dir unavailable, skipping save");
            return;
        };
        if let Some(parent) = path.parent()
            && let Err(e) = std::fs::create_dir_all(parent)
        {
            tracing::warn!("profile: failed to create config dir: {e}");
            return;
        }
        let file = ProfileConfigFile {
            email: email.filter(|s| !s.trim().is_empty()).map(str::to_owned),
        };
        match toml::to_string_pretty(&file) {
            Ok(contents) => {
                if let Err(e) = std::fs::write(&path, contents) {
                    tracing::warn!("profile: failed to write {}: {e}", path.display());
                }
            }
            Err(e) => tracing::warn!("profile: serialization failed: {e}"),
        }
    }
}
