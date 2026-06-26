//! TODO

use gpui::App;
use crate::data::theme::LibriTheme;
use crate::ui::library::cover::CoverCache;

// ── GPUI global initializer ───────────────────────────────────────────────────

/// Registers `LibriTheme` and `CoverCache` as GPUI app-level globals.
pub fn init_globals(cx: &mut App) {
    cx.set_global(LibriTheme::default_theme());
    cx.set_global(CoverCache::new());
}
