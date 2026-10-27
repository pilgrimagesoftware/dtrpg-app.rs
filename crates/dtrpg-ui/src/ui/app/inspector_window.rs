//! Popped-out inspector window creation.

use std::sync::Arc;

use dtrpg_core::data::ui_state::{UiState, WindowBoundsPref};
use gpui::{App, AppContext as _, Entity, Global, TitlebarOptions, WindowBounds, WindowOptions};
use gpui_kit::component::Root;

use super::window_bounds::{current_window_bounds, visible_window_bounds};
use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::ui::windows::inspector::InspectorWindowView;

/// Set once the application starts shutting down.
///
/// Closing a popped-out inspector forgets it, so it is not reopened at the
/// next launch. Windows also close when the application quits, and those
/// must stay remembered; they check for this marker to tell the two apart.
pub struct ShuttingDown;

impl Global for ShuttingDown {}

/// Opens the inspector for the item `item_id` in its own window, at
/// `saved_bounds` if they are still on a connected display.
///
/// The window is remembered until the user closes it, so it comes back at the
/// next launch.
///
/// # Panics
///
/// Panics if the window cannot be opened.
#[allow(clippy::expect_used)]
pub fn open_inspector_window(item_id: Arc<str>, title: &str,
                             saved_bounds: Option<WindowBoundsPref>,
                             controller: Entity<LibraryController>,
                             settings: Entity<SettingsController>, cx: &mut App)
                             -> gpui::WindowHandle<Root> {
    let window_bounds = saved_bounds.and_then(|saved| visible_window_bounds(saved, cx));
    let (window, _view) =
        gpui_kit::open_window(
                              WindowOptions { titlebar: Some(
            TitlebarOptions { title: Some(title.to_owned().into()),
                              ..Default::default() },
        ),
                                              window_bounds:
                                                  window_bounds.or_else(|| {
                                                      Some(WindowBounds::centered(default_size(),
                                                                                  cx))
                                                  }),
                                              ..Default::default() },
                              cx,
                              move |window, cx| {
                                  remember_window(&item_id, window);

                                  let item_id_for_close = Arc::clone(&item_id);
                                  window.on_window_should_close(cx, move |_window, cx| {
                if !cx.has_global::<ShuttingDown>() {
                    UiState::load().remove_inspector_window(&item_id_for_close);
                }
                true
            });

                                  cx.new(|cx| {
                                        InspectorWindowView::new(item_id, controller, settings,
                                                                 window, cx)
                                    })
                              },
        ).expect("failed to open inspector window");
    // See `open_library_window`'s matching comment: `gpui_kit::open_window`
    // wraps the view in a `Root`, so the window's root entity type is always
    // `Root`.
    window.downcast::<Root>()
          .expect("open_window's root entity is always Root")
}

/// Brings the library window to the front, if it is open.
pub fn show_library_window(cx: &mut App) {
    let Some(window) = cx.try_global::<super::LibraryWindowHandle>()
                         .map(|handle| handle.window)
    else {
        return;
    };
    // The window may have been closed; there is then nothing to bring forward.
    let _ = window.update(cx, |_, window, _| window.activate_window());
}

/// Size of an inspector window that has no saved place: wide enough for the
/// cover beside the details.
fn default_size() -> gpui::Size<gpui::Pixels> {
    gpui::Size { width:  gpui::px(960.),
                 height: gpui::px(640.), }
}

/// Saves that the inspector for `item_id` is popped out, and where its window
/// is now.
pub fn remember_window(item_id: &str, window: &gpui::Window) {
    UiState::load().save_inspector_window(item_id, Some(current_window_bounds(window)));
}
