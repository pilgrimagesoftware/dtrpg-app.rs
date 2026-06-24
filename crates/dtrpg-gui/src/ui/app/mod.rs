
use gpui::*;
use gpui_component::*;

use crate::ui::library::{root_view::LibraryRootView, state::init_globals};

pub fn setup(cx: &mut App) {
    init(cx);
    init_globals(cx);

    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Libri".into()),
                appears_transparent: true,
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let view = cx.new(|cx| LibraryRootView::new(window, cx));

            cx.new(|cx| Root::new(view, window, cx).bg(cx.theme().background))
        },
    )
    .expect("failed to open window");

    cx.activate(true);
}
