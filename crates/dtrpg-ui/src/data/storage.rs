//! Storage configuration: root path preference and per-item path derivation.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const APP_NAME: &str = "dtrpg";

// ── StorageError ──────────────────────────────────────────────────────────────

/// Errors that can occur when validating or applying a storage path.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// The specified path does not exist on disk.
    #[error("path does not exist: {0}")]
    PathDoesNotExist(PathBuf),
    /// The application cannot write to the specified path.
    #[error("path is not writable: {0}")]
    NotWritable(PathBuf),
    /// The volume containing the path appears to be unavailable.
    #[error("volume may be unavailable: {0}")]
    VolumeUnavailable(PathBuf),
}

// ── validate_writable ─────────────────────────────────────────────────────────

/// Validates `path` for writability by attempting a probe write.
///
/// Creates a temporary file inside `path` and immediately removes it.
/// This catches permission issues, read-only mounts, and full volumes.
///
/// # Errors
///
/// Returns [`StorageError::PathDoesNotExist`] if the directory is absent,
/// [`StorageError::VolumeUnavailable`] if the path cannot be stat'd, or
/// [`StorageError::NotWritable`] if the probe write fails.
pub fn validate_writable(path: &Path) -> Result<(), StorageError> {
    match path.try_exists() {
        Ok(true) => {}
        Ok(false) => return Err(StorageError::PathDoesNotExist(path.to_path_buf())),
        Err(_) => return Err(StorageError::VolumeUnavailable(path.to_path_buf())),
    }
    let probe = path.join(".dtrpg_write_probe");
    std::fs::write(&probe, b"probe").map_err(|_| StorageError::NotWritable(path.to_path_buf()))?;
    let _ = std::fs::remove_file(&probe);
    Ok(())
}

// ── StorageConfig ─────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Default)]
struct StorageConfigFile {
    root_path: Option<String>,
}

/// Manages the root directory where catalog data (downloads, metadata cache) is stored.
///
/// Persists the user's chosen override in `{config_dir}/dtrpg/storage.toml`.
/// Falls back to `{data_dir}/dtrpg/` when no override is set.
pub struct StorageConfig {
    override_path: Option<PathBuf>,
}

impl StorageConfig {
    /// Loads the storage config from disk. Returns a default-path config on any error.
    pub fn load() -> Self {
        let override_path = config_path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|text| toml::from_str::<StorageConfigFile>(&text).ok())
            .and_then(|cfg| cfg.root_path)
            .map(PathBuf::from);
        Self { override_path }
    }

    /// Returns the resolved storage root (saved override, or platform default).
    pub fn root_path(&self) -> PathBuf {
        self.override_path
            .clone()
            .unwrap_or_else(default_storage_path)
    }

    /// Returns `true` when the storage root has been verified accessible on disk.
    pub fn is_accessible(&self) -> bool {
        self.root_path().try_exists().unwrap_or(false)
    }

    /// Returns the directory where catalog metadata (e.g. the catalog cache) is stored.
    ///
    /// Maps to `{root}/metadata/`.
    pub fn metadata_path(&self) -> PathBuf {
        self.root_path().join("metadata")
    }

    /// Derives a stable per-item subdirectory under the downloads directory.
    ///
    /// Maps to `{root}/items/{item_id}/`.
    pub fn path_for_item(&self, item_id: &str) -> PathBuf {
        self.root_path().join("items").join(item_id)
    }

    /// Saves `path` as the new storage root override and updates the in-memory state.
    ///
    /// Creates parent directories as needed. Silently ignores I/O errors during save
    /// (the path is still applied in memory).
    pub fn set_root_path(&mut self, path: PathBuf) {
        let cfg = StorageConfigFile {
            root_path: Some(path.to_string_lossy().into_owned()),
        };
        if let Some(config_file) = config_path() {
            if let Some(parent) = config_file.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(text) = toml::to_string(&cfg) {
                let _ = std::fs::write(&config_file, text);
            }
        }
        self.override_path = Some(path);
    }

    /// Removes the override and reverts to the platform default on next `root_path()` call.
    pub fn clear_override(&mut self) {
        self.override_path = None;
        if let Some(path) = config_path() {
            let _ = std::fs::remove_file(path);
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn default_storage_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(APP_NAME)
}

fn config_path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join(APP_NAME).join("storage.toml"))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn default_path_is_non_empty() {
        let cfg = StorageConfig {
            override_path: None,
        };
        let path = cfg.root_path();
        assert!(path.components().count() > 0);
        assert!(path.ends_with("dtrpg"));
    }

    #[test]
    fn override_path_is_returned_when_set() {
        let custom = PathBuf::from("/tmp/custom-storage");
        let cfg = StorageConfig {
            override_path: Some(custom.clone()),
        };
        assert_eq!(cfg.root_path(), custom);
    }

    #[test]
    fn metadata_path_is_under_root() {
        let cfg = StorageConfig {
            override_path: Some(PathBuf::from("/tmp/dtrpg")),
        };
        assert_eq!(cfg.metadata_path(), Path::new("/tmp/dtrpg/metadata"));
    }

    #[test]
    fn path_for_item_is_under_root() {
        let cfg = StorageConfig {
            override_path: Some(PathBuf::from("/tmp/dtrpg")),
        };
        let item_path = cfg.path_for_item("b42");
        assert_eq!(item_path, Path::new("/tmp/dtrpg/items/b42"));
    }

    #[test]
    fn validate_writable_succeeds_on_temp_dir() {
        let dir = std::env::temp_dir();
        assert!(validate_writable(&dir).is_ok());
    }

    #[test]
    fn validate_writable_fails_on_missing_path() {
        let missing = PathBuf::from("/nonexistent/surely/missing/path");
        assert!(matches!(
            validate_writable(&missing),
            Err(StorageError::PathDoesNotExist(_))
        ));
    }
}
