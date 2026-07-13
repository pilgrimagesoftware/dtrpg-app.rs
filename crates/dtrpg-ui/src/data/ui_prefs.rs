//! Lightweight UI preference storage: panel widths and other layout state.

use serde::{Deserialize, Serialize};

use crate::data::paths::app_preferences_dir;
use crate::util::sort::{CollectionSortMethod, SortDirection};

/// Persisted position and size of the library window, in pixels.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WindowBoundsPref {
    pub x:      f32,
    pub y:      f32,
    pub width:  f32,
    pub height: f32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UiPrefsFile {
    /// Width of the sidebar panel in pixels.
    pub sidebar_width:             Option<f32>,
    /// Width of the detail panel in pixels.
    pub detail_width:              Option<f32>,
    /// Whether the Collections sidebar section is open (`true`) or collapsed
    /// (`false`).
    pub collections_open:          Option<bool>,
    /// Whether the Publishers sidebar section is open (`true`) or collapsed
    /// (`false`).
    pub publishers_open:           Option<bool>,
    /// Index of the last-active settings page (Account/Storage/File
    /// Openers/Advanced/About), so the settings window reopens on the same
    /// tab it was closed on.
    pub settings_page_ix:          Option<usize>,
    /// Position and size the library window was closed at, so it reopens
    /// there instead of always resetting to the default placement.
    pub library_window_bounds:     Option<WindowBoundsPref>,
    /// Persisted key of the active color theme (`ThemeKey`'s variant name,
    /// lowercased). `None` before this preference existed, or if never
    /// changed from the default.
    pub theme_key:                 Option<String>,
    /// Persisted font family name of the active body font. Any font
    /// installed on the user's system is valid, not a curated list.
    pub body_font_name:            Option<String>,
    /// Persisted font family name of the active value font.
    pub value_font_name:           Option<String>,
    /// Persisted font family name of the active label font.
    pub label_font_name:           Option<String>,
    /// Persisted font family name of the active monospace font.
    pub mono_font_name:            Option<String>,
    /// Persisted shared UI text size, in pixels/points.
    pub ui_text_size:              Option<f32>,
    /// Persisted sidebar Collections sort method (`CollectionSortMethod`'s
    /// variant name: `"name"`, `"date_created"`, or `"item_count"`).
    pub collection_sort:           Option<String>,
    /// Persisted sidebar Collections sort direction (`"ascending"` or
    /// `"descending"`).
    pub collection_sort_direction: Option<String>,
}

/// Persists and restores small UI preferences.
///
/// Backed by `{app_preferences_dir}/ui_prefs.toml`.
pub struct UiPrefs {
    data: UiPrefsFile,
}

impl UiPrefs {
    /// Load from disk; returns defaults on any error.
    pub fn load() -> Self {
        let data =
            std::fs::read_to_string(prefs_path()).ok()
                                                 .and_then(|text| {
                                                     toml::from_str::<UiPrefsFile>(&text).ok()
                                                 })
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

    /// Sidebar Collections sort method (defaults to `Name`).
    pub fn collection_sort(&self) -> CollectionSortMethod {
        match self.data.collection_sort.as_deref() {
            Some("date_created") => CollectionSortMethod::DateCreated,
            Some("item_count") => CollectionSortMethod::ItemCount,
            _ => CollectionSortMethod::Name,
        }
    }

    /// Sidebar Collections sort direction (defaults to `Ascending`).
    pub fn collection_sort_direction(&self) -> SortDirection {
        match self.data.collection_sort_direction.as_deref() {
            Some("descending") => SortDirection::Descending,
            _ => SortDirection::Ascending,
        }
    }

    /// Persist the sidebar Collections sort method and direction together.
    pub fn save_collection_sort(&mut self, method: CollectionSortMethod, direction: SortDirection) {
        self.data.collection_sort = Some(match method {
                                             CollectionSortMethod::Name => "name",
                                             CollectionSortMethod::DateCreated => "date_created",
                                             CollectionSortMethod::ItemCount => "item_count",
                                         }.to_string());
        self.data.collection_sort_direction = Some(match direction {
                                                       SortDirection::Ascending => "ascending",
                                                       SortDirection::Descending => "descending",
                                                   }.to_string());
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

    /// Persisted key of the active color theme, or `None` if never saved.
    pub fn theme_key(&self) -> Option<&str> {
        self.data.theme_key.as_deref()
    }

    /// Persist the active color theme's key.
    pub fn save_theme_key(&mut self, key: &str) {
        self.data.theme_key = Some(key.to_string());
        self.flush();
    }

    /// Persisted font family name of the active body font, or `None` if never
    /// saved.
    pub fn body_font_name(&self) -> Option<&str> {
        self.data.body_font_name.as_deref()
    }

    /// Persist the active body font's family name.
    pub fn save_body_font_name(&mut self, name: &str) {
        self.data.body_font_name = Some(name.to_string());
        self.flush();
    }

    /// Persisted font family name of the active value font, or `None` if
    /// never saved.
    pub fn value_font_name(&self) -> Option<&str> {
        self.data.value_font_name.as_deref()
    }

    /// Persist the active value font's family name.
    pub fn save_value_font_name(&mut self, name: &str) {
        self.data.value_font_name = Some(name.to_string());
        self.flush();
    }

    /// Persisted font family name of the active label font, or `None` if
    /// never saved.
    pub fn label_font_name(&self) -> Option<&str> {
        self.data.label_font_name.as_deref()
    }

    /// Persist the active label font's family name.
    pub fn save_label_font_name(&mut self, name: &str) {
        self.data.label_font_name = Some(name.to_string());
        self.flush();
    }

    /// Persisted font family name of the active monospace font, or `None` if
    /// never saved.
    pub fn mono_font_name(&self) -> Option<&str> {
        self.data.mono_font_name.as_deref()
    }

    /// Persist the active monospace font's family name.
    pub fn save_mono_font_name(&mut self, name: &str) {
        self.data.mono_font_name = Some(name.to_string());
        self.flush();
    }

    /// Persisted shared UI text size, or `None` if never saved.
    pub fn ui_text_size(&self) -> Option<f32> {
        self.data.ui_text_size
    }

    /// Persist the shared UI text size.
    pub fn save_ui_text_size(&mut self, size: f32) {
        self.data.ui_text_size = Some(size);
        self.flush();
    }

    fn flush(&self) {
        let path = prefs_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(text) = toml::to_string(&self.data) {
            let _ = std::fs::write(&path, text);
        }
    }
}

fn prefs_path() -> std::path::PathBuf {
    app_preferences_dir().join("ui_prefs.toml")
}
