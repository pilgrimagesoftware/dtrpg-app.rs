use gpui::*;
use gpui_component::{button::*, *};

pub struct AppWindow;

impl Render for AppWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0x010101))
            .child("Hello, World!")
            .child(Button::new("ok")
                .primary()
                .text_color(rgb(0xEFEFEF))
                .bg(rgb(0x1A1A1A))
                .label("Click Me")
                .on_click(|_, _, _| println!("Button clicked!")))
    }
}
