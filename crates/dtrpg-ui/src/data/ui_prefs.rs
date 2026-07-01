//! Lightweight UI preference storage: panel widths and other layout state.

use serde::{Deserialize, Serialize};
use crate::data::constants::APP_NAME;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UiPrefsFile {
    /// Width of the sidebar panel in pixels.
    pub sidebar_width: Option<f32>,
    /// Width of the detail panel in pixels.
    pub detail_width: Option<f32>,
    /// Catalog page size (items per page).
    pub page_size: Option<usize>,
}

/// Persists and restores small UI preferences.
///
/// Backed by `{config_dir}/dtrpg/ui_prefs.toml`.
pub struct UiPrefs {
    data: UiPrefsFile,
}

impl UiPrefs {
    /// Load from disk; returns defaults on any error.
    pub fn load() -> Self {
        let data = prefs_path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|text| toml::from_str::<UiPrefsFile>(&text).ok())
            .unwrap_or_default();
        Self { data }
    }

    /// Width of the sidebar panel, or `None` if not saved.
    pub fn sidebar_width(&self) -> Option<f32> {
        self.data.sidebar_width
    }

    /// Width of the detail panel, or `None` if not saved.
    pub fn detail_width(&self) -> Option<f32> {
        self.data.detail_width
    }

    /// Catalog page size, or `None` if not saved.
    pub fn page_size(&self) -> Option<usize> {
        self.data.page_size
    }

    /// Persist both sidebar and detail widths atomically.
    pub fn save_panel_widths(&mut self, sidebar: f32, detail: f32) {
        self.data.sidebar_width = Some(sidebar);
        self.data.detail_width = Some(detail);
        self.flush();
    }

    /// Persist the catalog page size.
    pub fn save_page_size(&mut self, size: usize) {
        self.data.page_size = Some(size);
        self.flush();
    }

    fn flush(&self) {
        if let Some(path) = prefs_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(text) = toml::to_string(&self.data) {
                let _ = std::fs::write(&path, text);
            }
        }
    }
}

fn prefs_path() -> Option<std::path::PathBuf> {
    Some(dirs::config_dir()?.join(APP_NAME).join("ui_prefs.toml"))
}
