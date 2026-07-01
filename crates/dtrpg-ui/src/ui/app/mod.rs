use gpui::*;
use gpui_component::{Root, init};
use tracing::warn;

use crate::credentials::{CredentialStore, KeyringCredentialStore, keys};
use crate::services::{LibraryService, LoginService, LoginTokens, collections::CollectionsService};
use crate::ui::actions::*;
use crate::ui::views::root_view::LibraryRootView;
use crate::util::init::init_globals;

/// Holds the factory closure used to create a [`LibraryService`] on demand.
///
/// `None` tokens mean the user is not authenticated; the factory should return a
/// service that reflects the unauthenticated state without crashing.
///
/// Set this global before calling [`setup`].
pub struct ServiceFactory(
    pub Box<dyn Fn(Option<LoginTokens>) -> Box<dyn LibraryService> + Send + Sync + 'static>,
);

impl Global for ServiceFactory {}

/// Holds the factory closure used to create a [`CollectionsService`] on demand.
///
/// `None` tokens mean the user is not authenticated; the factory should return a
/// service that reflects the unauthenticated state without crashing.
///
/// Set this global before calling [`setup`].
pub struct CollectionsServiceFactory(
    pub Box<dyn Fn(Option<LoginTokens>) -> Box<dyn CollectionsService> + Send + Sync + 'static>,
);

impl Global for CollectionsServiceFactory {}

/// Holds the factory closure used to create a [`LoginService`] on demand.
///
/// Used by [`SettingsController`] to authenticate the user from the Account tab.
pub struct LoginServiceFactory(pub Box<dyn Fn() -> Box<dyn LoginService> + Send + Sync + 'static>);

impl Global for LoginServiceFactory {}

/// Opens the library window in the unauthenticated state.
///
/// The window opens immediately. If `startup_api_key` is `Some`, the root view
/// kicks off a background re-authentication and transitions to authenticated on success.
///
/// # Panics
///
/// Panics if the window cannot be opened or if `ServiceFactory` has not been set.
#[allow(clippy::expect_used)]
pub fn open_library_window(startup_api_key: Option<String>, cx: &mut App) {
    let service = (cx.global::<ServiceFactory>().0)(None);
    let collections_service = (cx.global::<CollectionsServiceFactory>().0)(None);
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
            let view = cx.new(|cx| {
                LibraryRootView::new(window, cx, service, collections_service, startup_api_key)
            });
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

    // Key bindings
    cx.bind_keys([
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-,", ShowSettings, None),
        KeyBinding::new("cmd-h", HideApplication, None),
        KeyBinding::new("alt-cmd-h", HideOthers, None),
        KeyBinding::new("cmd-m", Minimize, None),
        KeyBinding::new("ctrl-cmd-f", ToggleFullscreen, None),
    ]);

    // App-level action handlers
    cx.on_action::<Quit>(|_, cx| cx.quit());
    cx.on_action::<HideApplication>(|_, cx| cx.hide());
    cx.on_action::<HideOthers>(|_, cx| cx.hide_other_apps());
    cx.on_action::<About>(|_, _cx| {});
    cx.on_action::<Minimize>(|_, cx| {
        if let Some(win) = cx.active_window() {
            win.update(cx, |_, window, _| window.minimize_window()).ok();
        }
    });
    cx.on_action::<Zoom>(|_, cx| {
        if let Some(win) = cx.active_window() {
            win.update(cx, |_, window, _| window.zoom_window()).ok();
        }
    });
    cx.on_action::<ToggleFullscreen>(|_, cx| {
        if let Some(win) = cx.active_window() {
            win.update(cx, |_, window, _| window.toggle_fullscreen())
                .ok();
        }
    });

    // Menu bar
    cx.set_menus([
        Menu::new("Libri").items([
            MenuItem::action("About Libri", About),
            MenuItem::separator(),
            MenuItem::action("Settings\u{2026}", ShowSettings),
            MenuItem::separator(),
            MenuItem::os_submenu("Services", SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action("Hide Libri", HideApplication),
            MenuItem::action("Hide Others", HideOthers),
            MenuItem::action("Show All", ShowAll),
            MenuItem::separator(),
            MenuItem::action("Quit Libri", Quit),
        ]),
        Menu::new("Edit").items([
            MenuItem::os_action("Undo", Undo, OsAction::Undo),
            MenuItem::os_action("Redo", Redo, OsAction::Redo),
            MenuItem::separator(),
            MenuItem::os_action("Cut", Cut, OsAction::Cut),
            MenuItem::os_action("Copy", Copy, OsAction::Copy),
            MenuItem::os_action("Paste", Paste, OsAction::Paste),
            MenuItem::os_action("Select All", SelectAll, OsAction::SelectAll),
        ]),
        Menu::new("View").items([MenuItem::action("Enter Full Screen", ToggleFullscreen)]),
        Menu::new("Window").items([
            MenuItem::action("Minimize", Minimize),
            MenuItem::action("Zoom", Zoom),
        ]),
        Menu::new("Help").items([MenuItem::action("About Libri", About)]),
    ]);

    let startup_api_key = match KeyringCredentialStore::new(keys::SERVICE, keys::API_KEY).load() {
        Ok(Some(cred)) => Some(cred.secret),
        Ok(None) => None,
        Err(e) => {
            warn!("keyring check failed: {e}");
            None
        }
    };

    open_library_window(startup_api_key, cx);
    cx.activate(true);
}
