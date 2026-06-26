
use gpui::*;
use gpui_component::{init, Root};

use crate::ui::views::root_view::LibraryRootView;
use crate::util::init::init_globals;

pub fn setup(cx: &mut App) {
    init(cx);
    // .SystemUIFont resolves to .AppleSystemUIFont via font_name_with_fallbacks,
    // which does not resolve on macOS 26 via NSFontFamilyAttribute. Use the
    // internal CoreText family name directly.
    cx.update_global::<gpui_component::Theme, _>(|theme, _cx| {
        theme.font_family = "Hoefler Text".into();
    });
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
            cx.new(|cx| Root::new(view, window, cx).bordered(false))
        },
    )
    .expect("failed to open window");

    cx.activate(true);
}
