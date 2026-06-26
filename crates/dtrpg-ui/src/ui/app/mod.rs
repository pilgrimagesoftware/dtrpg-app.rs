
use gpui::*;
use gpui_component::init;

use crate::ui::views::root_view::LibraryRootView;
use crate::util::init::init_globals;

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
        |window, cx| cx.new(|cx| LibraryRootView::new(window, cx)),
    )
    .expect("failed to open window");

    cx.activate(true);
}
