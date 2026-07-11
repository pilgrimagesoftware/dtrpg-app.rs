//! Download location configuration: root path preference and per-item path
//! derivation.
//!
//! This module governs only where downloaded catalog files are stored on disk.
//! It is intentionally decoupled from application cache/metadata (see
//! [`crate::data::paths::cache_dir`]) and from preferences (see
//! [`crate::data::paths::app_preferences_dir`]) — those live in fixed,
//! non-user-facing locations regardless of where the user chooses to store
//! downloads.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::data::paths::{app_preferences_dir, default_download_dir};

// ── StorageError
// ──────────────────────────────────────────────────────────────

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

// ── validate_writable
// ─────────────────────────────────────────────────────────

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

// ── StorageConfig
// ─────────────────────────────────────────────────────────────

/// Default number of concurrent thumbnail/download fetches when no override
/// has been saved.
pub const DEFAULT_MAX_CONCURRENT_DOWNLOADS: usize = 3;

fn default_max_concurrent_downloads() -> usize {
    DEFAULT_MAX_CONCURRENT_DOWNLOADS
}

#[derive(Serialize, Deserialize)]
struct StorageConfigFile {
    root_path:                Option<String>,
    #[serde(default = "default_max_concurrent_downloads")]
    max_concurrent_downloads: usize,
}

/// Manages the root directory where downloaded catalog files are stored, and
/// the shared thumbnail/download concurrency limit.
///
/// Persists the user's chosen overrides in
/// `{app_preferences_dir}/storage.toml`. Falls back to the platform default
/// download directory (e.g. `~/Downloads/dtrpg`) when no root path override is
/// set, and to [`DEFAULT_MAX_CONCURRENT_DOWNLOADS`] when no concurrency
/// override is set.
pub struct StorageConfig {
    override_path:            Option<PathBuf>,
    max_concurrent_downloads: usize,
}

impl StorageConfig {
    /// Loads the storage config from disk. Returns a default-path config on any
    /// error.
    pub fn load() -> Self {
        let file = config_path().and_then(|p| std::fs::read_to_string(p).ok())
                                .and_then(|text| toml::from_str::<StorageConfigFile>(&text).ok());
        let override_path = file.as_ref()
                                .and_then(|cfg| cfg.root_path.clone())
                                .map(PathBuf::from);
        let max_concurrent_downloads = file.map_or(DEFAULT_MAX_CONCURRENT_DOWNLOADS, |cfg| {
                                               cfg.max_concurrent_downloads
                                           });
        Self { override_path,
               max_concurrent_downloads }
    }

    /// Returns the resolved download root (saved override, or platform
    /// default).
    pub fn root_path(&self) -> PathBuf {
        self.override_path
            .clone()
            .unwrap_or_else(default_download_dir)
    }

    /// Returns `true` when the download root has been verified accessible on
    /// disk.
    pub fn is_accessible(&self) -> bool {
        self.root_path().try_exists().unwrap_or(false)
    }

    /// Returns `true` when no user override is set — `root_path()` resolves to
    /// the platform default download directory.
    #[must_use]
    pub fn is_default(&self) -> bool {
        self.override_path.is_none()
    }

    /// Creates the resolved root directory (and any missing parents) if it does
    /// not already exist.
    ///
    /// Only meaningful to call unconditionally for the platform default path —
    /// for a user-chosen override, a missing directory more likely means an
    /// unmounted volume than a fresh install, so callers should not blindly
    /// recreate it there.
    ///
    /// # Errors
    ///
    /// Returns the underlying I/O error if directory creation fails (e.g.
    /// permissions).
    pub fn ensure_root_exists(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.root_path())
    }

    /// Derives the directory an item's files are stored in, grouped by
    /// publisher.
    ///
    /// Maps to `{root}/items/{sanitized publisher}/` — see [`publisher_dir`].
    pub fn path_for_publisher(&self, publisher: &str) -> PathBuf {
        publisher_dir(&self.root_path(), publisher)
    }

    /// Saves `path` as the new storage root override and updates the in-memory
    /// state.
    ///
    /// Creates parent directories as needed. Silently ignores I/O errors during
    /// save (the path is still applied in memory).
    pub fn set_root_path(&mut self, path: PathBuf) {
        self.override_path = Some(path);
        self.save();
    }

    /// Removes the root path override and reverts to the platform default on
    /// next `root_path()` call. The concurrency limit is unaffected.
    pub fn clear_override(&mut self) {
        self.override_path = None;
        self.save();
    }

    /// Returns the configured maximum number of concurrent thumbnail/download
    /// fetches.
    #[must_use]
    pub fn max_concurrent_downloads(&self) -> usize {
        self.max_concurrent_downloads
    }

    /// Saves `n` as the new shared thumbnail/download concurrency limit and
    /// updates the in-memory state.
    pub fn set_max_concurrent_downloads(&mut self, n: usize) {
        self.max_concurrent_downloads = n;
        self.save();
    }

    /// Writes the current in-memory state to
    /// `{app_preferences_dir}/storage.toml`. Silently ignores I/O errors
    /// (the state remains applied in memory).
    fn save(&self) {
        let cfg = StorageConfigFile { root_path:
                                          self.override_path
                                              .as_ref()
                                              .map(|p| p.to_string_lossy().into_owned()),
                                      max_concurrent_downloads: self.max_concurrent_downloads, };
        let Some(config_file) = config_path()
        else {
            return;
        };
        if let Some(parent) = config_file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(text) = toml::to_string(&cfg) {
            let _ = std::fs::write(&config_file, text);
        }
    }
}

// ── Helpers
// ───────────────────────────────────────────────────────────────────

fn config_path() -> Option<PathBuf> {
    Some(app_preferences_dir().join("storage.toml"))
}

/// Derives the directory a downloaded item's files live in:
/// `{root}/items/{sanitized publisher}/`, grouping every item from the same
/// publisher together under a common `items/` directory.
///
/// Sanitizes `publisher` first so a name containing a path separator can
/// never escape `root` or be (mis)treated as an absolute path component —
/// see [`sanitize_path_component`]. Every call site that needs an item's
/// on-disk directory (downloads, "Open", "Reveal in Finder") must go through
/// this function rather than reimplementing the join inline.
pub fn publisher_dir(root: &Path, publisher: &str) -> PathBuf {
    root.join("items").join(sanitize_path_component(publisher))
}

/// Strips a leading path separator, replaces any remaining path separators,
/// and converts spaces to underscores, so the result can never be (or
/// contain) an absolute path component when joined onto another path and
/// reads as a single filesystem-friendly token.
fn sanitize_path_component(value: &str) -> String {
    value.trim_start_matches('/').replace(['/', ' '], "_")
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn default_path_is_non_empty() {
        let cfg = StorageConfig { override_path:            None,
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        let path = cfg.root_path();
        assert!(path.components().count() > 0);
        assert!(path.ends_with("dtrpg"));
    }

    #[test]
    fn override_path_is_returned_when_set() {
        let custom = PathBuf::from("/tmp/custom-storage");
        let cfg = StorageConfig { override_path:            Some(custom.clone()),
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        assert_eq!(cfg.root_path(), custom);
    }

    #[test]
    fn path_for_publisher_is_under_root() {
        let cfg = StorageConfig { override_path:            Some(PathBuf::from("/tmp/dtrpg")),
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        let publisher_path = cfg.path_for_publisher("Chaosium");
        assert_eq!(publisher_path, Path::new("/tmp/dtrpg/items/Chaosium"));
    }

    #[test]
    fn path_for_publisher_sanitizes_a_name_containing_a_path_separator() {
        // A publisher name is untrusted display text from the API; if it ever
        // contained a path separator, `PathBuf::join` would otherwise let it
        // escape `root` (or discard it entirely if the name looked absolute).
        let cfg = StorageConfig { override_path:            Some(PathBuf::from("/tmp/dtrpg")),
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        let publisher_path = cfg.path_for_publisher("/Evil/Publisher");
        assert_eq!(publisher_path, Path::new("/tmp/dtrpg/items/Evil_Publisher"));
    }

    #[test]
    fn path_for_publisher_converts_spaces_to_underscores() {
        let cfg = StorageConfig { override_path:            Some(PathBuf::from("/tmp/dtrpg")),
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        let publisher_path = cfg.path_for_publisher("The Forge Studios");
        assert_eq!(publisher_path,
                   Path::new("/tmp/dtrpg/items/The_Forge_Studios"));
    }

    #[test]
    fn publisher_dir_never_escapes_root_for_an_absolute_looking_name() {
        let dir = publisher_dir(Path::new("/tmp/dtrpg"), "/Evil/Publisher");
        assert!(dir.starts_with("/tmp/dtrpg"));
    }

    #[test]
    fn validate_writable_succeeds_on_temp_dir() {
        let dir = std::env::temp_dir();
        assert!(validate_writable(&dir).is_ok());
    }

    #[test]
    fn validate_writable_fails_on_missing_path() {
        let missing = PathBuf::from("/nonexistent/surely/missing/path");
        assert!(matches!(validate_writable(&missing),
                         Err(StorageError::PathDoesNotExist(_))));
    }

    #[test]
    fn is_default_true_without_override() {
        let cfg = StorageConfig { override_path:            None,
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        assert!(cfg.is_default());
    }

    #[test]
    fn is_default_false_with_override() {
        let cfg = StorageConfig { override_path:            Some(PathBuf::from("/tmp/custom-storage")),
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        assert!(!cfg.is_default());
    }

    #[test]
    fn ensure_root_exists_creates_missing_directory() {
        let root =
            std::env::temp_dir().join(format!("dtrpg-test-ensure-root-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let cfg = StorageConfig { override_path:            Some(root.clone()),
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        assert!(!cfg.is_accessible());
        cfg.ensure_root_exists().unwrap();
        assert!(cfg.is_accessible());
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn ensure_root_exists_is_idempotent_on_existing_directory() {
        let dir = std::env::temp_dir();
        let cfg = StorageConfig { override_path:            Some(dir),
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        assert!(cfg.ensure_root_exists().is_ok());
        assert!(cfg.ensure_root_exists().is_ok());
    }

    #[test]
    fn max_concurrent_downloads_defaults_to_three() {
        let cfg = StorageConfig { override_path:            None,
                                  max_concurrent_downloads: DEFAULT_MAX_CONCURRENT_DOWNLOADS, };
        assert_eq!(cfg.max_concurrent_downloads(), 3);
    }

    #[test]
    fn missing_max_concurrent_downloads_field_deserializes_to_default() {
        let file: StorageConfigFile = toml::from_str("").unwrap();
        assert_eq!(file.max_concurrent_downloads,
                   DEFAULT_MAX_CONCURRENT_DOWNLOADS);
    }
}
