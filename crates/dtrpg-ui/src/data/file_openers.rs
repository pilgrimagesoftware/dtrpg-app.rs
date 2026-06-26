//! File-opener override configuration: persists extension → app-path mappings.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// ── Types ─────────────────────────────────────────────────────────────────────

/// A single extension → application path override.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOpenerEntry {
    /// File extension without a leading dot, lower-cased (e.g. `"pdf"`).
    pub extension: String,
    /// Absolute path to the application used to open files of this type.
    pub app_path: PathBuf,
}

/// Outcome of calling [`FileOpenerConfig::add`].
#[derive(Debug, PartialEq, Eq)]
pub enum AddOutcome {
    /// A new entry was appended.
    Added,
    /// An existing entry for the same extension was replaced.
    Replaced,
}

/// Ordered list of file-opener overrides.
///
/// Serializes as a TOML `[[file_openers]]` array inside the shared app config
/// file (see [`config_path`]).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileOpenerConfig {
    #[serde(default, rename = "file_openers")]
    entries: Vec<FileOpenerEntry>,
}

// ── Wrapper used for TOML round-trips ─────────────────────────────────────────

#[derive(Serialize, Deserialize, Default)]
struct AppConfigFile {
    #[serde(default)]
    file_openers: Vec<FileOpenerEntry>,
    #[serde(default)]
    active_settings_tab: Option<String>,
}

// ── FileOpenerConfig impl ─────────────────────────────────────────────────────

impl FileOpenerConfig {
    // ── Persistence ───────────────────────────────────────────────────────────

    /// Loads the config from disk, returning a default if the file is absent or unparseable.
    pub fn load() -> Self {
        let Some(path) = config_path() else {
            return Self::default();
        };
        let Ok(text) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        let parsed: AppConfigFile = toml::from_str(&text).unwrap_or_default();
        Self { entries: parsed.file_openers }
    }

    /// Loads both the file-opener config and the active tab string from disk.
    pub fn load_with_tab() -> (Self, Option<String>) {
        let Some(path) = config_path() else {
            return (Self::default(), None);
        };
        let Ok(text) = std::fs::read_to_string(&path) else {
            return (Self::default(), None);
        };
        let parsed: AppConfigFile = toml::from_str(&text).unwrap_or_default();
        (Self { entries: parsed.file_openers }, parsed.active_settings_tab)
    }

    /// Saves the config (and optionally an active-tab hint) to disk, creating
    /// parent directories if needed.  Silently ignores I/O errors.
    pub fn save(&self, active_tab_name: Option<&str>) {
        let Some(path) = config_path() else { return };
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let file = AppConfigFile {
            file_openers: self.entries.clone(),
            active_settings_tab: active_tab_name.map(str::to_owned),
        };
        if let Ok(text) = toml::to_string(&file) {
            let _ = std::fs::write(path, text);
        }
    }

    // ── Read ──────────────────────────────────────────────────────────────────

    /// Returns all entries in the configured order.
    pub fn entries(&self) -> &[FileOpenerEntry] {
        &self.entries
    }

    /// Finds the override app path for `extension`.
    ///
    /// The lookup is case-insensitive and tolerates a leading dot in the input.
    pub fn find_override(&self, extension: &str) -> Option<&Path> {
        let normalized = normalize_ext(extension);
        self.entries
            .iter()
            .find(|e| e.extension == normalized)
            .map(|e| e.app_path.as_path())
    }

    // ── Write ─────────────────────────────────────────────────────────────────

    /// Adds `entry`, normalizing the extension.  Replaces an existing entry for
    /// the same extension.  Returns whether the entry was new or a replacement.
    pub fn add(&mut self, entry: FileOpenerEntry) -> AddOutcome {
        let normalized = FileOpenerEntry {
            extension: normalize_ext(&entry.extension),
            app_path: entry.app_path,
        };
        if let Some(existing) = self.entries.iter_mut().find(|e| e.extension == normalized.extension) {
            existing.app_path = normalized.app_path;
            return AddOutcome::Replaced;
        }
        self.entries.push(normalized);
        AddOutcome::Added
    }

    /// Removes the entry whose extension matches `extension` (case-insensitive, dot-tolerant).
    pub fn remove(&mut self, extension: &str) {
        let normalized = normalize_ext(extension);
        self.entries.retain(|e| e.extension != normalized);
    }

    /// Updates the `app_path` of an existing entry.  No-op if the extension is not found.
    pub fn update_app_path(&mut self, extension: &str, new_path: PathBuf) {
        let normalized = normalize_ext(extension);
        if let Some(entry) = self.entries.iter_mut().find(|e| e.extension == normalized) {
            entry.app_path = new_path;
        }
    }

    // ── Validation ────────────────────────────────────────────────────────────

    /// Returns entries whose `app_path` does not exist on disk.
    pub fn validate_all(&self) -> Vec<&FileOpenerEntry> {
        self.entries.iter().filter(|e| !e.app_path.exists()).collect()
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Strips a leading dot and lower-cases the extension string.
fn normalize_ext(ext: &str) -> String {
    ext.trim_start_matches('.').to_lowercase()
}

/// Returns the path to `~/.config/dtrpg/app_config.toml` (macOS/Linux) or
/// `%APPDATA%\dtrpg\app_config.toml` (Windows).
fn config_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let base = std::env::var("APPDATA").ok().map(PathBuf::from)?;
    #[cfg(not(target_os = "windows"))]
    let base = std::env::var("HOME").ok().map(|h| PathBuf::from(h).join(".config"))?;
    Some(base.join("dtrpg").join("app_config.toml"))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(ext: &str, path: &str) -> FileOpenerEntry {
        FileOpenerEntry { extension: ext.to_owned(), app_path: PathBuf::from(path) }
    }

    fn config_with(entries: Vec<FileOpenerEntry>) -> FileOpenerConfig {
        FileOpenerConfig { entries }
    }

    // ── find_override ─────────────────────────────────────────────────────────

    #[test]
    fn find_override_exact_match() {
        let cfg = config_with(vec![entry("pdf", "/Applications/Preview.app")]);
        assert_eq!(cfg.find_override("pdf"), Some(Path::new("/Applications/Preview.app")));
    }

    #[test]
    fn find_override_case_insensitive() {
        let cfg = config_with(vec![entry("pdf", "/Applications/Preview.app")]);
        assert_eq!(cfg.find_override("PDF"), Some(Path::new("/Applications/Preview.app")));
        assert_eq!(cfg.find_override("Pdf"), Some(Path::new("/Applications/Preview.app")));
    }

    #[test]
    fn find_override_dot_prefixed_input() {
        let cfg = config_with(vec![entry("pdf", "/Applications/Preview.app")]);
        assert_eq!(cfg.find_override(".pdf"), Some(Path::new("/Applications/Preview.app")));
        assert_eq!(cfg.find_override(".PDF"), Some(Path::new("/Applications/Preview.app")));
    }

    #[test]
    fn find_override_no_match() {
        let cfg = config_with(vec![entry("pdf", "/Applications/Preview.app")]);
        assert_eq!(cfg.find_override("epub"), None);
        assert_eq!(cfg.find_override(""), None);
    }

    // ── add ───────────────────────────────────────────────────────────────────

    #[test]
    fn add_new_entry() {
        let mut cfg = FileOpenerConfig::default();
        let result = cfg.add(entry("pdf", "/Applications/Preview.app"));
        assert_eq!(result, AddOutcome::Added);
        assert_eq!(cfg.entries().len(), 1);
        assert_eq!(cfg.entries()[0].extension, "pdf");
    }

    #[test]
    fn add_normalizes_extension() {
        let mut cfg = FileOpenerConfig::default();
        cfg.add(entry(".PDF", "/Applications/Preview.app"));
        assert_eq!(cfg.entries()[0].extension, "pdf");
    }

    #[test]
    fn add_duplicate_replaces() {
        let mut cfg = FileOpenerConfig::default();
        cfg.add(entry("pdf", "/Applications/Preview.app"));
        let result = cfg.add(entry("pdf", "/Applications/Acrobat.app"));
        assert_eq!(result, AddOutcome::Replaced);
        assert_eq!(cfg.entries().len(), 1);
        assert_eq!(cfg.entries()[0].app_path, PathBuf::from("/Applications/Acrobat.app"));
    }

    // ── validate_all ──────────────────────────────────────────────────────────

    #[test]
    fn validate_all_flags_missing_paths() {
        let cfg = config_with(vec![
            entry("pdf", "/nonexistent/Preview.app"),
            entry("epub", "/nonexistent/Calibre.app"),
        ]);
        let stale = cfg.validate_all();
        assert_eq!(stale.len(), 2);
    }

    #[test]
    fn validate_all_accepts_existing_paths() {
        let tmp = std::env::temp_dir();
        let cfg = config_with(vec![
            entry("pdf", tmp.to_str().unwrap_or("/tmp")),
        ]);
        let stale = cfg.validate_all();
        assert_eq!(stale.len(), 0);
    }
}
