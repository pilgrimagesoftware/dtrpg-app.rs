//! Application menu construction, for the native menu bar on macOS and the
//! in-window menu bar on Windows and Linux.

use dtrpg_core::platform::{Platform, PlatformIntegration, PlatformKind};
use gpui_kit::component::GlobalState;
use gpui_kit::component::menu::AppMenuBar;

use super::*;

/// The in-window menu bar's entity, kept so [`install_menus`] can tell it to
/// reload when the menus change. Set only on Windows and Linux.
pub struct MenuBarHandle(pub Entity<AppMenuBar>);

impl Global for MenuBarHandle {}

/// Builds the menus for the platform the app is running on.
pub fn build_menus(state: &ViewMenuState) -> Vec<Menu> {
    build_menus_for(Platform.kind(), state)
}

/// Makes `build_menus(state)` the application's menus.
///
/// On macOS they become the native menu bar. On Windows and Linux, which have
/// none, they are also handed to the in-window menu bar, which reloads from
/// them.
pub fn install_menus(cx: &mut App, state: &ViewMenuState) {
    cx.set_menus(build_menus(state));
    if Platform.kind() == PlatformKind::MacOs {
        return;
    }
    let owned = build_menus(state).into_iter().map(Menu::owned).collect();
    GlobalState::global_mut(cx).set_app_menus(owned);
    if let Some(bar) = cx.try_global::<MenuBarHandle>()
                         .map(|handle| handle.0.clone())
    {
        bar.update(cx, |bar, cx| bar.reload(cx));
    }
}

/// Builds the menus for `kind`.
///
/// macOS has an application menu (About, Settings, Services, Hide, Quit) at the
/// front. Windows and Linux have no such menu, so Settings and Quit go in a
/// File menu instead, the macOS-only items are left out, and the Catalog menu
/// is called Library.
pub fn build_menus_for(kind: PlatformKind, state: &ViewMenuState) -> Vec<Menu> {
    // Column-header clicks produce `SortMethod::Custom { col_key }` rather than
    // the named variants the menu offers; map each back to the menu item it
    // corresponds to so the checkmark still tracks column-driven sorts.
    let normalized_sort = match state.sort {
        SortMethod::Custom { col_key: "publisher", } => Some(SortMethod::Publisher),
        SortMethod::Custom { col_key: "added" } => Some(SortMethod::DateAdded),
        SortMethod::Custom { col_key: "pages" } => Some(SortMethod::PageCount),
        SortMethod::Custom { .. } => None,
        method => Some(method),
    };
    let sort_checked = |target: SortMethod| normalized_sort == Some(target);

    let first_menu = if kind == PlatformKind::MacOs {
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
        ])
    }
    else {
        Menu::new(t!("menu.file_title").to_string()).items([
            MenuItem::action(t!("menu.app_settings").to_string(), ShowSettings),
            MenuItem::separator(),
            MenuItem::action(t!("menu.app_quit").to_string(), Quit),
        ])
    };
    let catalog_title = if kind == PlatformKind::MacOs {
        t!("menu.catalog_title")
    }
    else {
        t!("menu.library_title")
    };

    vec![
        first_menu,
        Menu::new(catalog_title.to_string()).items([
            MenuItem::action(t!("menu.catalog_add_collection").to_string(), AddCollection),
            MenuItem::action(t!("menu.catalog_new_smart_collection").to_string(),
                             NewSmartCollection),
            MenuItem::separator(),
            MenuItem::action(t!("menu.catalog_reload").to_string(), ReloadCatalog),
            MenuItem::action(
                t!("menu.catalog_refresh_thumbnails").to_string(),
                RefreshThumbnails,
            ),
            MenuItem::action(
                t!("menu.catalog_check_availability").to_string(),
                CheckItemAvailability,
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
                    .checked(state.group_by == GroupBy::Publisher),
                    MenuItem::action(t!("menu.toggle_group_by_tag").to_string(), ToggleGroupByTag)
                        .checked(state.group_by == GroupBy::Tag),
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
        Menu::new(t!("menu.help_title").to_string()).items([
            MenuItem::action(t!("menu.app_about").to_string(), About),
            MenuItem::separator(),
            MenuItem::action(t!("menu.help_report").to_string(), ReportFeatureBug),
        ]),
    ]
}

#[cfg(test)]
mod tests {
    // Not `use super::*`: the parent glob-imports `gpui::*`, whose `test`
    // macro would shadow the standard `#[test]`.
    use dtrpg_core::platform::PlatformKind;
    use gpui::MenuItem;
    use rust_i18n::t;

    use super::build_menus_for;
    use crate::ui::app::ViewMenuState;

    fn titles(kind: PlatformKind) -> Vec<String> {
        build_menus_for(kind, &ViewMenuState::default()).into_iter()
                                                        .map(|menu| menu.name.to_string())
                                                        .collect()
    }

    #[test]
    fn macos_leads_with_the_application_menu() {
        assert_eq!(titles(PlatformKind::MacOs)[0], t!("sidebar.app_name"));
        assert_eq!(titles(PlatformKind::MacOs)[1], t!("menu.catalog_title"));
    }

    #[test]
    fn windows_and_linux_lead_with_a_file_menu_and_call_catalog_library() {
        for kind in [PlatformKind::Windows, PlatformKind::Linux] {
            let titles = titles(kind);
            assert_eq!(titles[0], t!("menu.file_title"));
            assert_eq!(titles[1], t!("menu.library_title"));
        }
    }

    #[test]
    fn the_platforms_have_the_same_number_of_menus() {
        let count = titles(PlatformKind::MacOs).len();
        assert_eq!(titles(PlatformKind::Windows).len(), count);
        assert_eq!(titles(PlatformKind::Linux).len(), count);
    }

    #[test]
    fn the_file_menu_has_settings_and_quit_and_nothing_macos_only() {
        let menus = build_menus_for(PlatformKind::Linux, &ViewMenuState::default());
        let file = &menus[0];

        assert_eq!(file.items.len(), 3);
        assert!(file.items
                    .iter()
                    .all(|item| !matches!(item, MenuItem::SystemMenu(_))));
    }
}
