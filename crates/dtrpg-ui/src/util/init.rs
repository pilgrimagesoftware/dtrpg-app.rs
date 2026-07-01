use crate::data::theme::LibriTheme;
use crate::i18n;
use crate::ui::library::cover::CoverCache;
use gpui::App;

// ── GPUI global initializer ───────────────────────────────────────────────────

/// Detects the system locale and activates it, then registers GPUI app-level globals.
///
/// Must be called before any view renders.
pub fn init_globals(cx: &mut App) {
    i18n::init();
    cx.set_global(LibriTheme::default_theme());
    cx.set_global(CoverCache::new());
}
