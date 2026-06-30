//! Application shell modules for the Rust desktop frontend.

use gpui::*;

use dtrpg_ui::ui::app::{LoginServiceFactory, ServiceFactory, setup};

/// Boots the GPUI application with a keyring-backed library service.
pub fn run() {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .with_quit_mode(QuitMode::LastWindowClosed)
        .run(|cx| {
            cx.set_global(ServiceFactory(Box::new(|tokens| {
                match tokens {
                    Some(t) => Box::new(crate::services::sdk::RustSdkLibraryService::from_keyring_with_tokens(t)),
                    None => Box::new(crate::services::sdk::RustSdkLibraryService::unauthenticated()),
                }
            })));
            cx.set_global(LoginServiceFactory(Box::new(
                crate::services::login::build_login_service,
            )));
            setup(cx);
        });
}
