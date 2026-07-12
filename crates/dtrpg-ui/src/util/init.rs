use gpui::App;

use crate::data::constants::{
    BODY_FONT_OPTIONS, MONO_FONT_OPTIONS, VALUE_FONT_OPTIONS, resolve_font,
};
use crate::data::theme::{Density, LibriTheme, ThemeKey};
use crate::data::ui_prefs::UiPrefs;
use crate::i18n;
use crate::ui::library::cover::CoverCache;

// ── GPUI global initializer
// ───────────────────────────────────────────────────

/// Detects the system locale and activates it, then registers GPUI app-level
/// globals.
///
/// Restores the persisted theme and font selections from [`UiPrefs`] (falling
/// back to defaults for anything missing or unrecognized — e.g. first launch,
/// or a prefs file predating this preference), rather than always starting
/// from [`LibriTheme::default_theme`].
///
/// Must be called before any view renders.
pub fn init_globals(cx: &mut App) {
    i18n::init();
    cx.set_global(initial_theme());
    cx.set_global(CoverCache::new());
}

/// Resolves the persisted theme key and font selections into a [`LibriTheme`],
/// used only at startup — subsequent changes go through
/// `LibraryController::set_theme`/`set_body_font`/`set_value_font`/
/// `set_mono_font`, which preserve whichever selections aren't being changed.
fn initial_theme() -> LibriTheme {
    let prefs = UiPrefs::load();
    let key = prefs.theme_key()
                   .and_then(ThemeKey::from_persisted_key)
                   .unwrap_or_default();
    let body_font = resolve_font(BODY_FONT_OPTIONS, prefs.body_font_id());
    let value_font = resolve_font(VALUE_FONT_OPTIONS, prefs.value_font_id());
    let mono_font = resolve_font(MONO_FONT_OPTIONS, prefs.mono_font_id());
    LibriTheme::new(key, Density::default(), body_font, value_font, mono_font)
}
