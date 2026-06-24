//! Application shell modules for the Rust desktop frontend.

use gpui::*;

use dtrpg_gui::ui::*;

pub fn run() {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .with_quit_mode(QuitMode::LastWindowClosed)
        .run(app::setup);
}
