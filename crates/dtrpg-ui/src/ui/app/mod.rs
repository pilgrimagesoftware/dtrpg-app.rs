use gpui::*;
use gpui_component::{init, Root};
use tracing::warn;

use crate::credentials::{Credential, CredentialStore, KeyringCredentialStore, keys};
use crate::services::{LibraryService, LoginService};
use crate::ui::views::root_view::LibraryRootView;
use crate::ui::windows::login::open_login_window;
use crate::util::init::init_globals;

/// Holds the factory closure used to create a [`LibraryService`] on demand.
///
/// Set this global before calling [`setup`] so both the startup routing and the
/// post-login library window opener can create fresh service instances.
pub struct ServiceFactory(pub Box<dyn Fn() -> Box<dyn LibraryService> + Send + Sync + 'static>);

impl Global for ServiceFactory {}

/// Holds the factory closure used to create a [`LoginService`] on demand.
///
/// Set this global before calling [`setup`] so the login window can obtain a service
/// instance to exchange the user's API key for session tokens.
pub struct LoginServiceFactory(pub Box<dyn Fn() -> Box<dyn LoginService> + Send + Sync + 'static>);

impl Global for LoginServiceFactory {}

/// Opens a new library window backed by a freshly created service from [`ServiceFactory`].
///
/// # Panics
///
/// Panics if the window cannot be opened or if `ServiceFactory` has not been set.
#[allow(clippy::expect_used)]
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

    let api_key = match KeyringCredentialStore::new(keys::SERVICE, keys::API_KEY).load() {
        Ok(Some(cred)) => Some(cred.secret),
        Ok(None) => None,
        Err(e) => {
            warn!("keyring check failed: {e}");
            None
        }
    };

    match api_key {
        Some(key) => {
            // Always re-authenticate on startup so the access token is guaranteed fresh.
            let login_service = (cx.global::<LoginServiceFactory>().0)();
            match login_service.authenticate(&key) {
                Ok(tokens) => {
                    let store = KeyringCredentialStore::new(keys::SERVICE, keys::ACCESS_TOKEN);
                    if let Err(e) = store.store(&Credential {
                        service: keys::SERVICE.into(),
                        account: keys::ACCESS_TOKEN.into(),
                        secret: tokens.access_token,
                    }) {
                        warn!("failed to store new access token: {e}");
                    }
                    let store = KeyringCredentialStore::new(keys::SERVICE, keys::REFRESH_TOKEN);
                    if let Err(e) = store.store(&Credential {
                        service: keys::SERVICE.into(),
                        account: keys::REFRESH_TOKEN.into(),
                        secret: tokens.refresh_token,
                    }) {
                        warn!("failed to store refresh token: {e}");
                    }
                    open_library_window(cx);
                }
                Err(e) => {
                    warn!("silent re-authentication failed: {e}");
                    if let Err(del_err) = KeyringCredentialStore::new(keys::SERVICE, keys::ACCESS_TOKEN).delete() {
                        warn!("failed to delete stale access token: {del_err}");
                    }
                    open_login_window(Some(key), cx);
                }
            }
        }
        None => {
            open_login_window(None, cx);
        }
    }

    cx.activate(true);
}
