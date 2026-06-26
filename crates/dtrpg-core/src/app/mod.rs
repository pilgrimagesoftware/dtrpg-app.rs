//! Application shell modules for the Rust desktop frontend.

use gpui::*;

use dtrpg_ui::ui::*;

/// Boots the GPUI application with an SDK-backed library service.
pub fn run() {
    let service = crate::services::sdk::RustSdkLibraryService::from_environment();
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .with_quit_mode(QuitMode::LastWindowClosed)
        .run(move |cx| app::setup(cx, Box::new(service)));
}
