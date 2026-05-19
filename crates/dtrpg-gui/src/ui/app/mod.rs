
use gpui::*;
use gpui_component::*;
use crate::ui::windows::app::AppWindow;

pub fn setup(cx: &mut App) {
    init(cx);

    cx.open_window(WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: Some("Experimenting".into()),
            appears_transparent: true,
            ..Default::default()
        }),
        ..Default::default()
    }, |window, cx| {
        let view = cx.new(|_| AppWindow);

        cx.new(|cx| {
            Root::new(view, window, cx)
                .bg(cx.theme().background)
        })
    })
    .expect("failed to open window");

    cx.activate(true);
}
