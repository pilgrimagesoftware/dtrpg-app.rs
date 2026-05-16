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
            .child(render_controls_row(self, cx))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .size_full()
                    .child(render_library_pane(self, cx))
                    .child(render_detail_pane(self)),
            )
            .child(div().text_color(rgb(0x93c5fd)).child(format!(
                "Status: {}",
                self.controller.shell.state().status_message
            )))
    }
}
