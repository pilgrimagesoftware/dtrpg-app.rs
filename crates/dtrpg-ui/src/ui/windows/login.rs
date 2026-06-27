//! Login window: presents the API key entry form at startup.

use gpui::{App, AppContext, TitlebarOptions, WindowOptions};
use gpui_component::Root;

use crate::controllers::login::LoginController;
use crate::ui::app::LoginServiceFactory;
use crate::ui::views::login_view::LoginView;

/// Opens the login window centered on screen.
#[allow(clippy::expect_used)]
pub fn open_login_window(cx: &mut App) {
    let login_service = (cx.global::<LoginServiceFactory>().0)();
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Sign In — Libri".into()),
                appears_transparent: true,
                ..Default::default()
            }),
            ..Default::default()
        },
        move |window, cx| {
            let login = cx.new(|_| LoginController::new(login_service));
            let view = cx.new(|cx| LoginView::new(window, cx, login));
            cx.new(|cx| Root::new(view, window, cx).bordered(false))
        },
    )
    .expect("failed to open login window");
}
