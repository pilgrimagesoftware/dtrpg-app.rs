//! Saving and restoring window positions and sizes.

use dtrpg_core::data::ui_state::{UiState, WindowBoundsPref};
use gpui::{App, Bounds, Point, Size, Window, WindowBounds, px};
use tracing::debug;

/// Resolves a saved position and size into a `WindowBounds`, or `None` (so
/// the window gets the OS default placement) if the saved bounds no longer
/// touch any connected display.
pub(super) fn visible_window_bounds(saved: WindowBoundsPref, cx: &App) -> Option<WindowBounds> {
    let bounds = Bounds { origin: Point { x: px(saved.x),
                                          y: px(saved.y), },
                          size:   Size { width:  px(saved.width),
                                         height: px(saved.height), }, };
    let display_bounds: Vec<_> = cx.displays().iter().map(|d| d.bounds()).collect();
    let fits_a_display = display_bounds.iter().any(|db| bounds.intersects(db));
    debug!(?saved,
           ?bounds,
           ?display_bounds,
           fits_a_display,
           "restoring window bounds");
    fits_a_display.then_some(WindowBounds::Windowed(bounds))
}

/// Reads `window`'s current position and size in a form that can be saved.
pub(super) fn current_window_bounds(window: &Window) -> WindowBoundsPref {
    let bounds = window.bounds();
    WindowBoundsPref { x:      bounds.origin.x.as_f32(),
                       y:      bounds.origin.y.as_f32(),
                       width:  bounds.size.width.as_f32(),
                       height: bounds.size.height.as_f32(), }
}

/// Resolves the library window's persisted position/size into a
/// `WindowBounds`, or `None` (falling back to the OS default placement) if
/// nothing was ever saved, or the saved bounds no longer intersect any
/// currently connected display.
pub(super) fn restore_library_window_bounds(cx: &App) -> Option<WindowBounds> {
    let saved = UiState::load().library_window_bounds()?;
    visible_window_bounds(saved, cx)
}

/// Saves the library window's current position/size to `UiState`.
///
/// Called both on every live resize/move (via `LibraryRootView`'s
/// `observe_window_bounds` subscription) and once more just before the
/// window closes (mirroring `open_settings_window`'s existing
/// `on_window_should_close` pattern), so the saved bounds stay current even
/// if the close hook doesn't fire for a given quit path.
pub fn save_library_window_bounds(window: &Window) {
    let bounds = current_window_bounds(window);
    debug!(?bounds, "saving library window bounds");
    UiState::load().save_library_window_bounds(bounds);
}

/// Resolves the settings window's saved position and size, or `None` (so the
/// window opens centered at its default size) if nothing was saved or the
/// saved bounds no longer touch a connected display.
pub(super) fn restore_settings_window_bounds(cx: &App) -> Option<WindowBounds> {
    let saved = UiState::load().settings_window_bounds()?;
    visible_window_bounds(saved, cx)
}

/// Saves the settings window's current position and size.
pub fn save_settings_window_bounds(window: &Window) {
    let bounds = current_window_bounds(window);
    debug!(?bounds, "saving settings window bounds");
    UiState::load().save_settings_window_bounds(bounds);
}
