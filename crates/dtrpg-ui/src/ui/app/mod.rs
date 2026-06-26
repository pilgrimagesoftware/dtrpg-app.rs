use gpui::*;
use gpui_component::{init, Root};
use tracing::warn;

use crate::credentials::{CredentialStore, KeyringCredentialStore, keys};
use crate::services::LibraryService;
use crate::ui::views::root_view::LibraryRootView;
use crate::ui::windows::login::open_login_window;
use crate::util::init::init_globals;

/// Holds the factory closure used to create a [`LibraryService`] on demand.
///
/// Set this global before calling [`setup`] so both the startup routing and the
/// post-login library window opener can create fresh service instances.
pub struct ServiceFactory(pub Box<dyn Fn() -> Box<dyn LibraryService> + Send + Sync + 'static>);

impl Global for ServiceFactory {}

/// Opens a new library window backed by a freshly created service from [`ServiceFactory`].
///
/// # Panics
///
/// Panics if the window cannot be opened or if `ServiceFactory` has not been set.
pub fn open_library_window(cx: &mut App) {
    let service = (cx.global::<ServiceFactory>().0)();
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Libri".into()),
                appears_transparent: true,
                ..Default::default()
            }),
            ..Default::default()
        },
        move |window, cx| {
            let view = cx.new(|cx| LibraryRootView::new(window, cx, service));
            cx.new(|cx| Root::new(view, window, cx).bordered(false))
        },
    )
    .expect("failed to open library window");
}

/// Initializes the GPUI application and routes to the login or library window.
///
/// Checks the platform keyring for a stored API key. Opens the library window when
/// credentials are found; falls back to the login window otherwise.
pub fn setup(cx: &mut App) {
    init(cx);
    cx.update_global::<gpui_component::Theme, _>(|theme, _cx| {
        theme.font_family = "Hoefler Text".into();
    });
    init_globals(cx);

    let api_key_result = KeyringCredentialStore::new(keys::SERVICE, keys::API_KEY).load();
    let has_credentials = match &api_key_result {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            warn!("keyring check failed: {e}");
            false
        }
    };

    if has_credentials {
        open_library_window(cx);
    } else {
        open_login_window(cx);
    }

    cx.activate(true);
}
