//! Ephemeral UI state: panel widths, sidebar section-open flags, the
//! last-active settings tab, the library window's position/size, and which
//! inspectors are popped out into their own windows.
//! Regenerated from defaults if lost — not something a user would expect to
//! find or hand-edit, unlike [`crate::data::ui_preferences`].
//!
//! Backed by `{app_data_dir}/ui_state.toml` (Application Support).

use serde::{Deserialize, Serialize};

use crate::data::paths::app_data_dir;

/// Persisted position and size of the library window, in pixels.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WindowBoundsPref {
    pub x:      f32,
    pub y:      f32,
    pub width:  f32,
    pub height: f32,
}

/// A catalog item's inspector that is popped out into its own window, so the
/// window can be reopened in the same place at the next launch.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InspectorWindowPref {
    /// Id of the catalog item the window shows.
    pub item_id: String,
    /// Where the window was last placed, or `None` if it has not been moved
    /// or resized yet.
    pub bounds:  Option<WindowBoundsPref>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UiStateFile {
    /// Width of the sidebar panel in pixels.
    pub sidebar_width:          Option<f32>,
    /// Width of the inspector pane in pixels.
    pub detail_width:           Option<f32>,
    /// Whether the Collections sidebar section is open (`true`) or collapsed
    /// (`false`).
    pub collections_open:       Option<bool>,
    /// Whether the Publishers sidebar section is open (`true`) or collapsed
    /// (`false`).
    pub publishers_open:        Option<bool>,
    /// Whether the Tags sidebar section is open (`true`) or collapsed
    /// (`false`).
    pub tags_open:              Option<bool>,
    /// Index of the last-active settings page (Account/Storage/File
    /// Openers/Advanced/About), so the settings window reopens on the same
    /// tab it was closed on.
    pub settings_page_ix:       Option<usize>,
    /// Position and size the library window was closed at, so it reopens
    /// there instead of always resetting to the default placement.
    pub library_window_bounds:  Option<WindowBoundsPref>,
    /// Position and size the settings window was last placed at.
    #[serde(default)]
    pub settings_window_bounds: Option<WindowBoundsPref>,
    /// Inspectors that are popped out into their own windows. Kept last
    /// because TOML writes a list of tables after the plain values.
    #[serde(default)]
    pub inspector_windows:      Vec<InspectorWindowPref>,
}

impl UiStateFile {
    /// Records that the inspector for `item_id` is popped out, replacing any
    /// earlier entry for the same item.
    fn upsert_inspector_window(&mut self, item_id: &str, bounds: Option<WindowBoundsPref>) {
        self.remove_inspector_window(item_id);
        self.inspector_windows
            .push(InspectorWindowPref { item_id: item_id.to_owned(),
                                        bounds });
    }

    /// Forgets the popped-out inspector for `item_id`, if there is one.
    fn remove_inspector_window(&mut self, item_id: &str) {
        self.inspector_windows
            .retain(|window| window.item_id != item_id);
    }
}

/// Persists and restores ephemeral UI state.
///
/// Backed by `{app_data_dir}/ui_state.toml`.
pub struct UiState {
    data: UiStateFile,
}

impl UiState {
    /// Load from disk; returns defaults on any error.
    ///
    /// If no file exists yet (first run), writes the defaults to disk
    /// immediately, mirroring [`crate::data::storage::StorageConfig::load`].
    /// A file that exists but fails to parse is left untouched.
    pub fn load() -> Self {
        let path = state_path();
        let file_existed = path.exists();
        let data =
            std::fs::read_to_string(&path).ok()
                                          .and_then(|text| {
                                              toml::from_str::<UiStateFile>(&text).ok()
                                          })
                                          .unwrap_or_default();
        let state = Self { data };
        if !file_existed {
            state.flush();
        }
        state
    }

    /// Width of the sidebar panel, or `None` if not saved.
    pub fn sidebar_width(&self) -> Option<f32> {
        self.data.sidebar_width
    }

    /// Width of the detail panel, or `None` if not saved.
    pub fn detail_width(&self) -> Option<f32> {
        self.data.detail_width
    }

    /// Persist the sidebar width only.
    pub fn save_sidebar_width(&mut self, sidebar: f32) {
        self.data.sidebar_width = Some(sidebar);
        self.flush();
    }

    /// Persist both sidebar and detail widths atomically.
    pub fn save_panel_widths(&mut self, sidebar: f32, detail: f32) {
        self.data.sidebar_width = Some(sidebar);
        self.data.detail_width = Some(detail);
        self.flush();
    }

    /// Whether the Collections sidebar section is open (defaults to `true`).
    pub fn collections_open(&self) -> bool {
        self.data.collections_open.unwrap_or(true)
    }

    /// Whether the Publishers sidebar section is open (defaults to `true`).
    pub fn publishers_open(&self) -> bool {
        self.data.publishers_open.unwrap_or(true)
    }

    /// Whether the Tags sidebar section is open. Defaults to open.
    pub fn tags_open(&self) -> bool {
        self.data.tags_open.unwrap_or(true)
    }

    /// Persist the Collections section open state.
    pub fn save_collections_open(&mut self, open: bool) {
        self.data.collections_open = Some(open);
        self.flush();
    }

    /// Persist the Publishers section open state.
    pub fn save_publishers_open(&mut self, open: bool) {
        self.data.publishers_open = Some(open);
        self.flush();
    }

    /// Persist whether the Tags sidebar section is open.
    pub fn save_tags_open(&mut self, open: bool) {
        self.data.tags_open = Some(open);
        self.flush();
    }

    /// Index of the last-active settings page, or `None` if never saved.
    pub fn settings_page_ix(&self) -> Option<usize> {
        self.data.settings_page_ix
    }

    /// Persist the active settings page index.
    pub fn save_settings_page_ix(&mut self, ix: usize) {
        self.data.settings_page_ix = Some(ix);
        self.flush();
    }

    /// Position and size the library window was last closed at, or `None` if
    /// never saved.
    pub fn library_window_bounds(&self) -> Option<WindowBoundsPref> {
        self.data.library_window_bounds
    }

    /// Persist the library window's position and size.
    pub fn save_library_window_bounds(&mut self, bounds: WindowBoundsPref) {
        self.data.library_window_bounds = Some(bounds);
        self.flush();
    }

    /// Position and size the settings window was last placed at, or `None` if
    /// never saved.
    pub fn settings_window_bounds(&self) -> Option<WindowBoundsPref> {
        self.data.settings_window_bounds
    }

    /// Persist the settings window's position and size.
    pub fn save_settings_window_bounds(&mut self, bounds: WindowBoundsPref) {
        self.data.settings_window_bounds = Some(bounds);
        self.flush();
    }

    /// Inspectors that were popped out into their own windows, oldest first.
    pub fn inspector_windows(&self) -> Vec<InspectorWindowPref> {
        self.data.inspector_windows.clone()
    }

    /// Records the popped-out inspector for `item_id` and where its window is.
    pub fn save_inspector_window(&mut self, item_id: &str, bounds: Option<WindowBoundsPref>) {
        self.data.upsert_inspector_window(item_id, bounds);
        self.flush();
    }

    /// Forgets the popped-out inspector for `item_id`, so it is not reopened
    /// at the next launch.
    pub fn remove_inspector_window(&mut self, item_id: &str) {
        self.data.remove_inspector_window(item_id);
        self.flush();
    }

    /// Forgets every popped-out inspector.
    pub fn clear_inspector_windows(&mut self) {
        self.data.inspector_windows.clear();
        self.flush();
    }

    fn flush(&self) {
        let path = state_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(text) = toml::to_string(&self.data) {
            let _ = std::fs::write(&path, text);
        }
    }
}

fn state_path() -> std::path::PathBuf {
    app_data_dir().join("ui_state.toml")
}

#[cfg(test)]
mod tests {
    use super::{UiStateFile, WindowBoundsPref};

    fn bounds(x: f32) -> WindowBoundsPref {
        WindowBoundsPref { x,
                           y: 20.0,
                           width: 600.0,
                           height: 400.0 }
    }

    #[test]
    fn upsert_replaces_the_entry_for_the_same_item() {
        let mut file = UiStateFile::default();
        file.upsert_inspector_window("a", None);
        file.upsert_inspector_window("b", Some(bounds(1.0)));
        file.upsert_inspector_window("a", Some(bounds(2.0)));

        let ids: Vec<_> = file.inspector_windows
                              .iter()
                              .map(|w| w.item_id.as_str())
                              .collect();
        assert_eq!(ids, ["b", "a"]);
        assert!(file.inspector_windows[1].bounds.is_some());
    }

    #[test]
    fn remove_forgets_only_the_named_item() {
        let mut file = UiStateFile::default();
        file.upsert_inspector_window("a", None);
        file.upsert_inspector_window("b", None);
        file.remove_inspector_window("a");

        assert_eq!(file.inspector_windows.len(), 1);
        assert_eq!(file.inspector_windows[0].item_id, "b");
    }

    #[test]
    fn popped_out_windows_survive_a_toml_round_trip() {
        let mut file = UiStateFile { library_window_bounds: Some(bounds(5.0)),
                                     ..UiStateFile::default() };
        file.upsert_inspector_window("a", Some(bounds(1.0)));
        file.upsert_inspector_window("b", None);

        let text = toml::to_string(&file).unwrap_or_default();
        let loaded: UiStateFile = toml::from_str(&text).unwrap_or_default();

        assert_eq!(loaded.inspector_windows.len(), 2);
        assert_eq!(loaded.inspector_windows[0].item_id, "a");
        assert!(loaded.inspector_windows[1].bounds.is_none());
    }

    #[test]
    fn a_file_saved_before_pop_out_existed_still_loads() {
        let loaded: UiStateFile = toml::from_str("sidebar_width = 250.0\n").unwrap_or_default();
        assert!(loaded.inspector_windows.is_empty());
        assert_eq!(loaded.sidebar_width, Some(250.0));
    }
}
