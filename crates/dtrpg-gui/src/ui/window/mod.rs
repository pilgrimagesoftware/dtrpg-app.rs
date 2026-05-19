use gpui::*;

pub struct AppWindow;

impl Render for AppWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child("Hello, GPUI")
    }
}
