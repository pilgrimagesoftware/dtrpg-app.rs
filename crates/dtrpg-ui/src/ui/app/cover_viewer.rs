//! Cover-viewer window creation.

use dtrpg_core::data::library::LibraryItem;
use gpui::{
    App, AppContext as _, Entity, Pixels, Size, TitlebarOptions, WindowBounds, WindowHandle,
    WindowOptions, px,
};
use gpui_kit::component::Root;

use crate::controllers::library::LibraryController;
use crate::ui::views::cover_viewer::CoverViewerView;

/// The smallest the viewer can be resized to.
const MIN_SIZE: Size<Pixels> = Size { width:  px(240.),
                                      height: px(320.), };

/// Opens a window that shows `item`'s cover as large as the window allows.
///
/// The window follows `controller`, so the full-size image appears as soon as
/// it has been fetched.
///
/// # Panics
///
/// Panics if the window cannot be opened.
#[allow(clippy::expect_used)]
pub fn open_cover_viewer_window(item: LibraryItem, controller: Entity<LibraryController>,
                                cx: &mut App)
                                -> WindowHandle<Root> {
    let window_size = Size { width:  px(640.),
                             height: px(900.), };
    let title = item.title.to_string();
    let (window, _view) =
        gpui_kit::open_window(WindowOptions { titlebar:
                                                  Some(TitlebarOptions { title:
                                                                             Some(title.into()),
                                                                         ..Default::default() }),
                                              window_bounds:
                                                  Some(WindowBounds::centered(window_size, cx)),
                                              window_min_size: Some(MIN_SIZE),
                                              ..Default::default() },
                              cx,
                              move |window, cx| {
                                  cx.new(|cx| CoverViewerView::new(item, controller, window, cx))
                              }).expect("failed to open cover viewer window");
    // See `open_library_window`'s matching comment: `gpui_kit::open_window`
    // wraps the view in a `Root`, so the window's root entity type is always
    // `Root`.
    window.downcast::<Root>()
          .expect("open_window's root entity is always Root")
}
