//! Settings panel: sidebar navigation + per-page content. Renders as the
//! content of a dedicated settings window (see `settings_window_view`), not
//! as an in-window overlay.
//!
//! Page navigation is a custom `Sidebar`/`SidebarMenu` rather than
//! gpui-kit's `Settings` widget: that widget tracks the active page in
//! its own per-window state with no way to read it back, which would reset
//! the active page to the first one every time the settings window is closed
//! and reopened (a new window has its own state). Driving the active page
//! from `SettingsController::active_page_ix` instead lets it persist across
//! close/reopen (see `dtrpg_core::data::ui_state::UiState`).

use std::path::PathBuf;

use dtrpg_core::data::file_openers::FileOpenerEntry;
use dtrpg_core::data::native_preferences::NativePreferencesFile;
use gpui::prelude::*;
use gpui::{
    AnyElement, Entity, FocusHandle, IntoElement, MouseButton, ParentElement, Styled, div, px,
};
use gpui_kit::component::input::InputState;
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::sidebar::{Sidebar, SidebarMenu, SidebarMenuItem};
use rust_i18n::t;

use crate::controllers::library::{FilterValueSection, LibraryController};
use crate::controllers::settings::{AuthStateSnapshot, SettingsController};
use crate::data::theme::{ColorTokens, LibriTheme};
use crate::ui::widgets::suppress_selection_outside_text;
use crate::ui::views::{
    settings_account_view::render_account_section,
    settings_advanced_view::render_advanced_section,
    settings_agent_view::render_agent_section,
    settings_appearance_view::render_appearance_section,
    settings_file_openers_view::render_file_openers_section,
    settings_storage_view::{StoragePageData, render_storage_section},
    settings_theme_editor::ThemeEditorData,
};
use crate::view_models::agent_display::AgentPageData;

/// Number of settings pages (Account, Appearance, Downloads Location, File
/// Openers, Native Integration, Advanced); see [`page_title`] and the `match`
/// in [`render_settings_panel`]. Sidebar order matches index order.
const PAGE_COUNT: usize = 6;

/// Returns the i18n title for page `ix`, or the Account page's title if `ix`
/// is out of range (defensive default for a persisted index from a prior app
/// version with fewer pages).
fn page_title(ix: usize) -> String {
    match ix {
        1 => t!("settings.appearance_title").to_string(),
        2 => t!("settings.downloads_location").to_string(),
        3 => t!("settings.file_openers_title").to_string(),
        4 => t!("agent.title").to_string(),
        5 => t!("settings.advanced_title").to_string(),
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
                             storage: StoragePageData, agent: AgentPageData,
                             theme_editor: ThemeEditorData, entity: Entity<SettingsController>,
                             library: Entity<LibraryController>, focus_handle: &FocusHandle,
                             colors: &ColorTokens, theme: &LibriTheme,
                             email_input: Option<Entity<InputState>>,
                             password_input: Option<Entity<InputState>>,
                             sign_in_in_progress: bool, sign_in_enabled: bool,
                             sign_in_error: Option<String>,
                             file_opener_extension_input: Entity<InputState>,
                             pending_file_opener: Option<PathBuf>, active_page_ix: usize,
                             recently_updated_window_days: u32,
                             recently_updated_window_input: Option<Entity<InputState>>,
                             native: NativePreferencesFile,
                             filter_values: Vec<FilterValueSection>)
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
        1 => render_appearance_section(library, colors, theme, theme_editor).into_any_element(),
        2 => render_storage_section(entity.clone(), colors, storage).into_any_element(),
        3 => render_file_openers_section(file_openers,
                                         entity.clone(),
                                         colors,
                                         file_opener_extension_input,
                                         pending_file_opener).into_any_element(),
        4 => render_agent_section(entity.clone(),
                                  agent,
                                  colors,
                                  &theme.fonts.label_font,
                                  &theme.fonts.mono_font).into_any_element(),
        5 => render_advanced_section(entity.clone(),
                                     native,
                                     colors,
                                     &theme.fonts.label_font,
                                     recently_updated_window_days,
                                     recently_updated_window_input,
                                     &filter_values,
                                     &library).into_any_element(),
        _ => render_account_section(&auth,
                                    entity.clone(),
                                    colors,
                                    email_input,
                                    password_input,
                                    sign_in_in_progress,
                                    sign_in_enabled,
                                    sign_in_error,
                                    &theme.fonts.mono_font),
    };

    // Reserves vertical clearance for the macOS traffic light buttons, which
    // overlay the top-left of the window when it opens with
    // `appears_transparent: true` (see `open_settings_window`) and no
    // titlebar row of its own — without this, the buttons sit directly on
    // top of the sidebar's first menu item. Also renders the window title
    // (native title text is suppressed by `appears_transparent`) and doubles
    // as a drag handle, matching the main library window's
    // `title_bar_view::render_title_bar`. Shorter than that view's 44px bar
    // since there's no account button/wordmark to balance here — just the
    // centered title.
    let drag_region = div().id("settings-drag-region")
                           .h(px(28.0))
                           .flex_none()
                           .flex()
                           .items_center()
                           .justify_center()
                           .text_base()
                           .line_height(px(28.0))
                           .font_weight(gpui::FontWeight::BOLD)
                           .text_color(colors.text_primary)
                           .child(t!("settings.title").to_string())
                           .on_mouse_down(MouseButton::Left, |_, window, _| {
                               window.start_window_move();
                           });

    div().id("settings-window-root")
         .track_focus(focus_handle)
         .on_mouse_down(MouseButton::Left, suppress_selection_outside_text)
         .on_key_down(move |event, _window, cx| {
             if event.keystroke.key == "escape" {
                 entity_escape.update(cx, |ctrl, cx| ctrl.close(cx));
             }
         })
         .size_full()
         .bg(surface)
         .flex()
         .flex_col()
         .child(drag_region)
         .child(div().flex_1()
                     .min_h_0()
                     .flex()
                     .flex_row()
                     .child(sidebar)
                     // `overflow_y_scrollbar()` wraps this element in
                     // `Scrollable`, whose `render` only copies the `size`
                     // style field onto its own outer wrapper — `flex_1()`/
                     // `min_w_0()`/`min_h_0()` set here would be silently
                     // dropped. Those sizing properties live on this plain
                     // outer div instead; the inner div just needs
                     // `size_full()` to fill it, which `Scrollable` already
                     // applies on its own.
                     .child(div().flex_1().min_w_0().min_h_0().child(div().flex()
                                                                          .flex_col()
                                                                          .size_full()
                                                                          .overflow_y_scrollbar()
                                                                          .child(content))))
         .into_any_element()
}

#[cfg(test)]
mod page_tests {
    use rust_i18n::t;

    use super::page_title;
    use crate::controllers::settings::FILE_OPENERS_PAGE_IX;

    #[test]
    fn the_file_openers_page_index_is_the_file_openers_page() {
        assert_eq!(page_title(FILE_OPENERS_PAGE_IX),
                   t!("settings.file_openers_title"));
    }
}
