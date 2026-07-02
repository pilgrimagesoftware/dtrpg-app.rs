use gpui::*;
use gpui_component::{Root, init};
use rust_i18n::t;
use tracing::warn;

use crate::credentials::{CredentialStore, KeyringCredentialStore};
use crate::data::constants::{KEYRING_API_KEY, KEYRING_SERVICE};
use crate::data::enums::CatalogPresentation;
use crate::services::{LibraryService, LoginService, LoginTokens, collections::CollectionsService};
use crate::ui::actions::*;
use crate::ui::views::root_view::LibraryRootView;
use crate::util::init::init_globals;
use crate::util::sort::{SortDirection, SortMethod};

/// Snapshot of catalog view state needed to render checkmarks in the native menu bar's
/// View menu (presentation mode, sort field/direction, and grouping).
///
/// Rebuilt and passed to [`build_menus`] every time the catalog view state changes, so
/// the OS menu's checkmarks stay in sync with the toolbar/keyboard-driven selection.
#[derive(Debug, Clone, Copy, Default)]
pub struct ViewMenuState {
    pub presentation: CatalogPresentation,
    pub sort: SortMethod,
    pub sort_direction: SortDirection,
    pub grouped: bool,
}

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
                title: Some(t!("sidebar.app_name").to_string().into()),
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

    // Sync gpui-component's table colors (DataTable/Table) with the active Libri
    // theme; otherwise the catalog list view renders with gpui-component's default
    // light table colors regardless of which Libri theme is active.
    let initial_colors = cx.global::<crate::data::theme::LibriTheme>().colors.clone();
    cx.update_global::<gpui_component::Theme, _>(|theme, _cx| {
        crate::data::theme::apply_table_colors(theme, &initial_colors);
    });

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
    // The real handler lives on `LibraryRootView` (opens the About dialog). This is a
    // harmless fallback in case the action fires before any window has focus.
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
    cx.set_menus(build_menus(&ViewMenuState::default()));

    let startup_api_key = match KeyringCredentialStore::new(KEYRING_SERVICE, KEYRING_API_KEY).load()
    {
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

/// Builds the full native menu bar, applying checkmarks to the View menu's
/// Presentation and Sort submenus (and the Group toggle) based on `state`.
///
/// `cx.set_menus` replaces the entire menu bar on every call, so this must
/// reconstruct the whole bar rather than just the affected submenus. Called once at
/// startup with the default state, and again by `LibraryRootView` whenever the
/// catalog's presentation/sort/grouping changes, so the OS menu's checkmarks track
/// the toolbar's current selection.
pub fn build_menus(state: &ViewMenuState) -> Vec<Menu> {
    // Column-header clicks produce `SortMethod::Custom { col_key }` rather than the
    // named variants the menu offers; map each back to the menu item it corresponds
    // to so the checkmark still tracks column-driven sorts.
    let normalized_sort = match state.sort {
        SortMethod::Custom {
            col_key: "publisher",
        } => Some(SortMethod::Publisher),
        SortMethod::Custom { col_key: "added" } => Some(SortMethod::DateAdded),
        SortMethod::Custom { col_key: "pages" } => Some(SortMethod::PageCount),
        SortMethod::Custom { .. } => None,
        method => Some(method),
    };
    let sort_checked = |target: SortMethod| normalized_sort == Some(target);

    vec![
        Menu::new(t!("sidebar.app_name").to_string()).items([
            MenuItem::action(t!("menu.app_about").to_string(), About),
            MenuItem::separator(),
            MenuItem::action(t!("menu.app_settings").to_string(), ShowSettings),
            MenuItem::separator(),
            MenuItem::os_submenu(
                t!("menu.app_services").to_string(),
                SystemMenuType::Services,
            ),
            MenuItem::separator(),
            MenuItem::action(t!("menu.app_hide").to_string(), HideApplication),
            MenuItem::action(t!("menu.app_hide_others").to_string(), HideOthers),
            MenuItem::action(t!("menu.app_show_all").to_string(), ShowAll),
            MenuItem::separator(),
            MenuItem::action(t!("menu.app_quit").to_string(), Quit),
        ]),
        Menu::new(t!("menu.catalog_title").to_string()).items([
            MenuItem::action(t!("menu.catalog_add_collection").to_string(), AddCollection),
            MenuItem::separator(),
            MenuItem::action(t!("menu.catalog_reload").to_string(), ReloadCatalog),
            MenuItem::action(
                t!("menu.catalog_refresh_thumbnails").to_string(),
                RefreshThumbnails,
            ),
        ]),
        Menu::new(t!("menu.edit_title").to_string()).items([
            MenuItem::os_action(t!("menu.edit_undo").to_string(), Undo, OsAction::Undo),
            MenuItem::os_action(t!("menu.edit_redo").to_string(), Redo, OsAction::Redo),
            MenuItem::separator(),
            MenuItem::os_action(t!("menu.edit_cut").to_string(), Cut, OsAction::Cut),
            MenuItem::os_action(t!("menu.edit_copy").to_string(), Copy, OsAction::Copy),
            MenuItem::os_action(t!("menu.edit_paste").to_string(), Paste, OsAction::Paste),
            MenuItem::os_action(
                t!("menu.edit_select_all").to_string(),
                SelectAll,
                OsAction::SelectAll,
            ),
        ]),
        Menu::new(t!("menu.view_title").to_string()).items([
            MenuItem::action(t!("menu.view_full_screen").to_string(), ToggleFullscreen),
            MenuItem::separator(),
            MenuItem::submenu(
                Menu::new(t!("menu.view_mode_title").to_string()).items([
                    MenuItem::action(t!("menu.view_as_list").to_string(), ViewAsList)
                        .checked(state.presentation == CatalogPresentation::List),
                    MenuItem::action(t!("menu.view_as_thumbs").to_string(), ViewAsThumbs)
                        .checked(state.presentation == CatalogPresentation::Thumbs),
                    MenuItem::action(t!("menu.view_as_grid").to_string(), ViewAsGrid)
                        .checked(state.presentation == CatalogPresentation::Grid),
                ]),
            ),
            MenuItem::submenu(
                Menu::new(t!("menu.sort_title").to_string()).items([
                    MenuItem::action(t!("menu.sort_by_title").to_string(), SortByTitle)
                        .checked(sort_checked(SortMethod::Title)),
                    MenuItem::action(t!("menu.sort_by_publisher").to_string(), SortByPublisher)
                        .checked(sort_checked(SortMethod::Publisher)),
                    MenuItem::action(t!("menu.sort_by_date_added").to_string(), SortByDateAdded)
                        .checked(sort_checked(SortMethod::DateAdded)),
                    MenuItem::action(t!("menu.sort_by_pages").to_string(), SortByPages)
                        .checked(sort_checked(SortMethod::PageCount)),
                    MenuItem::separator(),
                    MenuItem::action(t!("menu.sort_ascending").to_string(), SortAscending)
                        .checked(state.sort_direction == SortDirection::Ascending),
                    MenuItem::action(t!("menu.sort_descending").to_string(), SortDescending)
                        .checked(state.sort_direction == SortDirection::Descending),
                    MenuItem::separator(),
                    MenuItem::action(
                        t!("menu.toggle_group_by_publisher").to_string(),
                        ToggleGroupByPublisher,
                    )
                    .checked(state.grouped),
                ]),
            ),
            MenuItem::separator(),
            MenuItem::action(t!("menu.focus_search").to_string(), FocusSearch),
        ]),
        Menu::new(t!("menu.window_title").to_string()).items([
            MenuItem::action(t!("menu.window_minimize").to_string(), Minimize),
            MenuItem::action(t!("menu.window_zoom").to_string(), Zoom),
            MenuItem::separator(),
            MenuItem::action(t!("menu.window_show_activity").to_string(), ShowActivity),
            MenuItem::action(
                t!("menu.window_show_alert_history").to_string(),
                ShowAlertHistory,
            ),
        ]),
        Menu::new(t!("menu.help_title").to_string())
            .items([MenuItem::action(t!("menu.app_about").to_string(), About)]),
    ]
}
