use dtrpg_core::data::library_metadata::ensure_cache_metadata_exists;
use dtrpg_core::data::constants::DEFAULT_UI_TEXT_SIZE;
use dtrpg_core::data::paths::cache_dir;
use dtrpg_core::data::ui_preferences::UiPreferences;
use gpui::{App, px};

use crate::data::theme::{Density, FontSelections, LibriTheme};
use crate::data::theme_catalog::ThemeCatalog;
use crate::i18n;
use crate::ui::library::cover::CoverCache;

// ── GPUI global initializer
// ───────────────────────────────────────────────────

/// Detects the system locale and activates it, then registers GPUI app-level
/// globals.
///
/// Restores the persisted theme and font selections from [`UiPreferences`]
/// (falling back to defaults for anything missing, unrecognized, or no longer
/// installed — e.g. first launch, a prefs file predating this preference, or
/// a font that was uninstalled since it was chosen), rather than always
/// starting from [`LibriTheme::default_theme`].
///
/// Must be called before any view renders.
pub fn init_globals(cx: &mut App) {
    i18n::init();
    // Discovers user themes first so `initial_theme` can resolve a persisted
    // user-theme id, and so the render path never reads the themes directory.
    ThemeCatalog::refresh();
    cx.set_global(initial_theme());
    cx.set_global(CoverCache::new());
    crate::util::theme_watcher::start_theme_watcher(cx);
    // `StorageConfig::load`/`UiPreferences::load`/`UiState::load` already
    // write their own defaults to disk on first call; the catalog cache
    // metadata sidecar has no such self-initializing wrapper type (it's a
    // free function keyed by an explicit `root: &Path`), so it needs an
    // explicit boot-time call instead.
    ensure_cache_metadata_exists(&cache_dir());
}

/// Resolves the persisted theme key and font selections into a [`LibriTheme`],
/// used only at startup — subsequent changes go through
/// `LibraryController::set_theme`/`set_ui_text_size`, which preserve
/// whichever selection isn't being changed.
fn initial_theme() -> LibriTheme {
    let prefs = UiPreferences::load();
    let id = prefs.theme_key()
                  .and_then(ThemeCatalog::resolve)
                  .unwrap_or_default();

    // Persisted as a scale multiplier (1.0 = normal), not an absolute pixel
    // size, so it round-trips to exactly what Settings > Appearance displays.
    let text_scale = prefs.text_scale().unwrap_or(1.0);
    let ui_text_size = px(text_scale * DEFAULT_UI_TEXT_SIZE);

    // The families come from the theme; `LibriTheme::new` fills them in.
    let fonts = FontSelections { ui_text_size,
                                 ..FontSelections::default() };
    // A density that is missing or not recognized falls back to the default.
    let density = prefs.density()
                       .and_then(|key| key.parse::<Density>().ok())
                       .unwrap_or_default();
    LibriTheme::new(id, density, fonts)
}
