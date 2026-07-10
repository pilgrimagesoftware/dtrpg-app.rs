//! Settings panel: sidebar navigation + per-page content. Renders as the
//! content of a dedicated settings window (see `settings_window_view`), not
//! as an in-window overlay.
//!
//! Page navigation is a custom `Sidebar`/`SidebarMenu` rather than
//! gpui-component's `Settings` widget: that widget tracks the active page in
//! its own per-window state with no way to read it back, which would reset
//! the active page to the first one every time the settings window is closed
//! and reopened (a new window has its own state). Driving the active page
//! from `SettingsController::active_page_ix` instead lets it persist across
//! close/reopen (see `crate::data::ui_prefs::UiPrefs`).

use std::path::PathBuf;

use gpui::prelude::*;
use gpui::{AnyElement, Entity, FocusHandle, IntoElement, ParentElement, Styled, div, px};
use gpui_component::input::InputState;
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::sidebar::{Sidebar, SidebarMenu, SidebarMenuItem};
use rust_i18n::t;

use crate::controllers::settings::{AuthStateSnapshot, CacheCounts, SettingsController};
use crate::data::file_openers::FileOpenerEntry;
use crate::data::theme::ColorTokens;
use crate::ui::views::{
    settings_account_view::render_account_section,
    settings_advanced_view::{render_about_section, render_advanced_section},
    settings_file_openers_view::render_file_openers_section,
    settings_storage_view::render_storage_section,
};

/// Number of settings pages (Account, Downloads Location, File Openers,
/// Advanced, About); see [`page_title`] and the `match` in
/// [`render_settings_panel`].
const PAGE_COUNT: usize = 5;

/// Returns the i18n title for page `ix`, or the Account page's title if `ix`
/// is out of range (defensive default for a persisted index from a prior app
/// version with fewer pages).
fn page_title(ix: usize) -> String {
    match ix {
        1 => t!("settings.downloads_location").to_string(),
        2 => t!("settings.file_openers_title").to_string(),
        3 => t!("settings.advanced_title").to_string(),
        4 => t!("settings.about_title").to_string(),
        _ => t!("settings.account_title").to_string(),
    }
}

// ── Public render entry point
// ─────────────────────────────────────────────────

/// Renders the settings panel, filling the settings window's content area.
///
/// `focus_handle` is the settings window's root focus handle; Escape closes
/// the window via [`SettingsController::close`].
#[allow(clippy::too_many_arguments)]
pub fn render_settings_panel(file_openers: &[FileOpenerEntry], auth: AuthStateSnapshot,
                             storage_root_path: PathBuf, storage_path_exists: bool,
                             entity: Entity<SettingsController>, focus_handle: &FocusHandle,
                             colors: &ColorTokens, email_input: Option<Entity<InputState>>,
                             password_input: Option<Entity<InputState>>,
                             sign_in_in_progress: bool, sign_in_enabled: bool,
                             sign_in_error: Option<String>,
                             storage_path_input: Option<Entity<InputState>>,
                             file_opener_extension_input: Entity<InputState>,
                             pending_file_opener: Option<PathBuf>, active_page_ix: usize,
                             cache_counts: CacheCounts)
                             -> AnyElement {
    let surface = colors.surface;
    let active_page_ix = if active_page_ix < PAGE_COUNT {
        active_page_ix
    }
    else {
        0
    };

    let entity_escape = entity.clone();

    let sidebar_menu = SidebarMenu::new().children((0..PAGE_COUNT).map(|ix| {
                                                       let entity = entity.clone();
                                                       SidebarMenuItem::new(page_title(ix))
                    .active(ix == active_page_ix)
                    .on_click(move |_, _, cx| {
                        entity.update(cx, |ctrl, cx| ctrl.set_active_page_ix(ix, cx));
                    })
                                                   }));
    let sidebar = Sidebar::new("settings-sidebar").w(px(160.0))
                                                  .border_0()
                                                  .collapsible(false)
                                                  .collapsed(false)
                                                  .child(sidebar_menu);

    let content: AnyElement = match active_page_ix {
        1 => render_storage_section(storage_root_path,
                                    storage_path_exists,
                                    entity.clone(),
                                    colors,
                                    storage_path_input).into_any_element(),
        2 => render_file_openers_section(file_openers,
                                         entity.clone(),
                                         colors,
                                         file_opener_extension_input,
                                         pending_file_opener).into_any_element(),
        3 => render_advanced_section(entity.clone(), cache_counts, colors).into_any_element(),
        4 => render_about_section(colors).into_any_element(),
        _ => render_account_section(&auth,
                                    entity.clone(),
                                    colors,
                                    email_input,
                                    password_input,
                                    sign_in_in_progress,
                                    sign_in_enabled,
                                    sign_in_error),
    };

    div().id("settings-window-root")
         .track_focus(focus_handle)
         .on_key_down(move |event, _window, cx| {
             if event.keystroke.key == "escape" {
                 entity_escape.update(cx, |ctrl, cx| ctrl.close(cx));
             }
         })
         .size_full()
         .bg(surface)
         .flex()
         .flex_row()
         .child(sidebar)
         .child(div().flex_1()
                     .min_w_0()
                     .min_h_0()
                     .flex()
                     .flex_col()
                     .overflow_y_scrollbar()
                     .child(content))
         .into_any_element()
}
