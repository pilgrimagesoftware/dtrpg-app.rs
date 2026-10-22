//! The switches that decide which tools an agent may call.
//!
//! Backed by `{app_preferences_dir}/mcp.toml`. Access is off until the user
//! turns it on, and then read-only until they widen it.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::{McpScope, McpTool};
use crate::data::paths::app_preferences_dir;

/// Why a call was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Denial {
    /// Agent access is switched off.
    Off,
    /// The tool's scope is switched off.
    ScopeOff(McpScope),
    /// The tool itself is switched off.
    ToolOff,
}

impl Denial {
    /// What to tell the agent, in plain words.
    #[must_use]
    pub fn message(self) -> String {
        match self {
            Self::Off => "agent access is switched off in Libri's settings".to_owned(),
            Self::ScopeOff(scope) => {
                format!("the {scope} scope is switched off in Libri's settings")
            }
            Self::ToolOff => "this tool is switched off in Libri's settings".to_owned(),
        }
    }
}

/// The settings as stored.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct McpSettingsFile {
    /// Whether agents may use Libri at all. Off by default.
    pub enabled:            bool,
    /// Whether the read scope is on.
    pub read:               bool,
    /// Whether the change scope is on. Off until the user widens access.
    pub change:             bool,
    /// Whether the download and spend scope is on. Off until the user widens
    /// access.
    pub download_and_spend: bool,
    /// Names of tools switched off even though their scope is on.
    pub disabled_tools:     BTreeSet<String>,
}

impl Default for McpSettingsFile {
    fn default() -> Self {
        Self { enabled:            false,
               read:               true,
               change:             false,
               download_and_spend: false,
               disabled_tools:     BTreeSet::new(), }
    }
}

/// Persists and restores the agent access settings.
pub struct McpSettings {
    /// Where the settings are saved, or `None` if they are never written.
    path: Option<PathBuf>,
    data: McpSettingsFile,
}

fn settings_path() -> PathBuf {
    app_preferences_dir().join("mcp.toml")
}

impl McpSettings {
    /// Loads the settings from disk; the defaults (access off) on any error.
    /// A missing or unreadable file therefore never widens access.
    #[must_use]
    pub fn load() -> Self {
        Self::load_from(&settings_path())
    }

    /// Loads the settings from `path`, as [`Self::load`] does.
    #[must_use]
    pub fn load_from(path: &Path) -> Self {
        let data = std::fs::read_to_string(path).ok()
                                                .and_then(|text| toml::from_str(&text).ok())
                                                .unwrap_or_default();
        Self { path: Some(path.to_owned()),
               data }
    }

    /// The defaults, never written to disk, for tests and previews.
    #[must_use]
    pub fn in_memory() -> Self {
        Self { path: None,
               data: McpSettingsFile::default(), }
    }

    /// Settings holding `data`, never written to disk. For working out what
    /// settings that are already in hand allow.
    #[must_use]
    pub const fn from_file(data: McpSettingsFile) -> Self {
        Self { path: None, data }
    }

    /// The settings as they are now.
    #[must_use]
    pub fn snapshot(&self) -> McpSettingsFile {
        self.data.clone()
    }

    /// Whether `scope` is switched on.
    #[must_use]
    pub const fn scope_enabled(&self, scope: McpScope) -> bool {
        match scope {
            McpScope::Read => self.data.read,
            McpScope::Change => self.data.change,
            McpScope::DownloadAndSpend => self.data.download_and_spend,
        }
    }

    /// Whether `tool` is switched on by itself, whatever its scope.
    #[must_use]
    pub fn tool_enabled(&self, tool: McpTool) -> bool {
        !self.data.disabled_tools.contains(tool.name())
    }

    /// Whether a call of `tool` is allowed, or why not. Access that is off
    /// refuses everything; then the tool's scope, then the tool, are checked.
    ///
    /// # Errors
    ///
    /// Returns the [`Denial`] when the call is refused.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::data::mcp_access::{Denial, McpSettings, McpTool};
    ///
    /// let settings = McpSettings::in_memory();
    /// assert_eq!(settings.check(McpTool::ListItems), Err(Denial::Off));
    /// ```
    pub fn check(&self, tool: McpTool) -> Result<(), Denial> {
        if !self.data.enabled {
            return Err(Denial::Off);
        }
        if !self.scope_enabled(tool.scope()) {
            return Err(Denial::ScopeOff(tool.scope()));
        }
        if !self.tool_enabled(tool) {
            return Err(Denial::ToolOff);
        }
        Ok(())
    }

    /// Persists whether agents may use Libri at all.
    pub fn save_enabled(&mut self, enabled: bool) {
        self.data.enabled = enabled;
        self.flush();
    }

    /// Persists whether `scope` is on.
    pub fn save_scope(&mut self, scope: McpScope, enabled: bool) {
        match scope {
            McpScope::Read => self.data.read = enabled,
            McpScope::Change => self.data.change = enabled,
            McpScope::DownloadAndSpend => self.data.download_and_spend = enabled,
        }
        self.flush();
    }

    /// Persists whether `tool` is on.
    pub fn save_tool(&mut self, tool: McpTool, enabled: bool) {
        if enabled {
            self.data.disabled_tools.remove(tool.name());
        }
        else {
            self.data.disabled_tools.insert(tool.name().to_owned());
        }
        self.flush();
    }

    /// Writes the settings to disk, creating the directory if needed. A
    /// failure is logged and otherwise ignored.
    fn flush(&self) {
        let Some(path) = &self.path
        else {
            return;
        };
        let result = toml::to_string_pretty(&self.data).map_err(|error| error.to_string())
                                                       .and_then(|text| {
                                                           if let Some(dir) = path.parent() {
                                                               std::fs::create_dir_all(dir)
                                                                   .map_err(|e| e.to_string())?;
                                                           }
                                                           std::fs::write(path, text)
                                                               .map_err(|e| e.to_string())
                                                       });
        if let Err(error) = result {
            tracing::warn!(%error, path = %path.display(), "could not save the agent settings");
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    fn in_temp_dir() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("prefs").join("mcp.toml");
        (dir, path)
    }

    #[test]
    fn access_is_off_by_default_and_a_missing_file_writes_nothing() {
        let (_dir, path) = in_temp_dir();

        let settings = McpSettings::load_from(&path);

        assert_eq!(settings.check(McpTool::ListItems), Err(Denial::Off));
        assert!(!path.exists());
    }

    #[test]
    fn switching_access_on_allows_reading_only() {
        let (_dir, path) = in_temp_dir();
        let mut settings = McpSettings::load_from(&path);

        settings.save_enabled(true);

        assert_eq!(settings.check(McpTool::ListItems), Ok(()));
        assert_eq!(settings.check(McpTool::SetTags),
                   Err(Denial::ScopeOff(McpScope::Change)));
        assert_eq!(settings.check(McpTool::DownloadItem),
                   Err(Denial::ScopeOff(McpScope::DownloadAndSpend)));
    }

    #[test]
    fn a_tool_can_be_switched_off_inside_a_scope_that_is_on() {
        let (_dir, path) = in_temp_dir();
        let mut settings = McpSettings::load_from(&path);
        settings.save_enabled(true);

        settings.save_tool(McpTool::GetSummary, false);

        assert_eq!(settings.check(McpTool::GetSummary), Err(Denial::ToolOff));
        assert_eq!(settings.check(McpTool::GetItem), Ok(()));
        settings.save_tool(McpTool::GetSummary, true);
        assert_eq!(settings.check(McpTool::GetSummary), Ok(()));
    }

    #[test]
    fn the_choices_are_there_after_loading_again() {
        let (_dir, path) = in_temp_dir();
        let mut settings = McpSettings::load_from(&path);
        settings.save_enabled(true);
        settings.save_scope(McpScope::Change, true);
        settings.save_tool(McpTool::SetTagColor, false);

        let reloaded = McpSettings::load_from(&path);

        assert_eq!(reloaded.check(McpTool::SetTags), Ok(()));
        assert_eq!(reloaded.check(McpTool::SetTagColor), Err(Denial::ToolOff));
    }

    #[test]
    fn a_file_that_cannot_be_read_never_widens_access() {
        let (_dir, path) = in_temp_dir();
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, "enabled = \"yes please\"").unwrap();

        let settings = McpSettings::load_from(&path);

        assert_eq!(settings.check(McpTool::ListItems), Err(Denial::Off));
    }
}
