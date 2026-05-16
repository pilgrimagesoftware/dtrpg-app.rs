//! GPUI root view and window bootstrap for the library feature.

use gpui::{
    App, Application, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

use crate::ui::library::controller::library_controller::LibraryController;

use super::controls_view::render_controls_row;
use super::detail_pane_view::render_detail_pane;
use super::library_pane_view::render_library_pane;

/// Launches the GPUI desktop window.
pub fn launch() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.0), px(760.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| LibraryRootView::new()),
        )
        .expect("failed to open desktop window");
    });
}

/// Root library feature view.
pub(crate) struct LibraryRootView {
    /// Interaction/state controller for shell behavior.
    pub(crate) controller: LibraryController,
}

impl LibraryRootView {
    /// Creates a new root view with SDK-backed library state.
    fn new() -> Self {
        Self {
            controller: LibraryController::new(),
        }
    }
}

impl Render for LibraryRootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x0b1220))
            .text_color(rgb(0xf8fafc))
            .p_3()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .items_center()
                    .child("DriveThruRPG Library")
                    .child(
                        div()
                            .id("account-menu")
                            .px_2()
                            .py_1()
                            .bg(rgb(0x1f2937))
                            .rounded_sm()
                            .cursor_pointer()
                            .child(self.controller.account_summary())
                            .on_click(
                                cx.listener(|this, _, _, _| this.controller.toggle_account_menu()),
                            ),
                    )
                    .child(
                        div()
                            .id("refresh-library")
                            .px_2()
                            .py_1()
                            .bg(rgb(0x1d4ed8))
                            .rounded_sm()
                            .cursor_pointer()
                            .child("Refresh")
                            .on_click(cx.listener(|this, _, _, _| this.controller.refresh())),
                    ),
            )
            .when(self.controller.account.menu_open, |view| {
                view.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .p_2()
                        .bg(rgb(0x111827))
                        .border_1()
                        .border_color(rgb(0x374151))
                        .rounded_sm()
                        .child(self.controller.account.display_name.clone())
                        .child(self.controller.account.connection_status.clone())
                        .child(
                            div()
                                .id("set-access-token")
                                .px_2()
                                .py_1()
                                .bg(rgb(0x1d4ed8))
                                .rounded_sm()
                                .cursor_pointer()
                                .child("Set access token")
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.controller.mark_token_set_action()
                                })),
                        )
                        .child(
                            div()
                                .id("reset-access-token")
                                .px_2()
                                .py_1()
                                .bg(rgb(0x334155))
                                .rounded_sm()
                                .cursor_pointer()
                                .child("Reset access token")
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.controller.mark_token_reset_action()
                                })),
                        )
                        .child(
                            div()
                                .id("open-settings")
                                .px_2()
                                .py_1()
                                .bg(rgb(0x334155))
                                .rounded_sm()
                                .cursor_pointer()
                                .child("Application settings")
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.controller.open_settings_action()
                                })),
                        ),
                )
            })
            .child(render_controls_row(self, cx))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .size_full()
                    .child(render_library_pane(self, cx))
                    .child(render_detail_pane(self)),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_color(rgb(0x93c5fd))
                    .child(format!("View: {}", self.controller.view_summary()))
                    .child(format!(
                        "{} | {}",
                        self.controller.sync_status_summary(),
                        self.controller.sync_status_detail()
                    ))
                    .child(format!(
                        "Status: {}",
                        self.controller.shell.state().status_message
                    )),
            )
    }
}
