//! Root view for the settings window: a separate, non-modal window (see
//! `implement-rust-settings-window-gpui`) rather than an overlay inside the
//! main library window.

use gpui::{
    Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled, Window,
    div,
};
use gpui_kit::component::input::InputState;

use crate::controllers::library::LibraryController;
use crate::controllers::settings::{ADVANCED_PAGE_IX, STORAGE_PAGE_IX, SettingsController};
use crate::data::theme::LibriTheme;
use crate::ui::app::save_settings_window_bounds_soon;
use crate::ui::views::settings_storage_view::StoragePageData;
use crate::ui::views::settings_theme_editor::{ThemeEditorData, load_theme_editor};
use crate::ui::views::settings_view::render_settings_panel;
use crate::util::debounce::Debounce;
use crate::view_models::agent_display::AgentPageData;
use crate::view_models::storage_display::StorageDisplay;

/// Root view for the settings window.
///
/// Shares the same [`SettingsController`] entity as
/// [`super::root_view::LibraryRootView`] so drafts, active tab, and other
/// settings state persist across close/reopen within the app session — the
/// entity isn't recreated per window open. Also shares the same
/// [`LibraryController`] entity so the Appearance page's font/theme setters
/// mutate the same controller the main window reads from.
pub struct SettingsWindowView {
    settings:                    Entity<SettingsController>,
    library:                     Entity<LibraryController>,
    /// Draft input for the in-progress "add file opener" row. Owned by the
    /// main window's `LibraryRootView` (not the settings controller, unlike
    /// the email/password/storage-path inputs), so it's passed in explicitly.
    file_opener_extension_input: Entity<InputState>,
    focus_handle:                FocusHandle,
    /// Waits for a move or resize to stop before saving the bounds.
    bounds_debounce:             Debounce,
}

impl SettingsWindowView {
    /// Constructs the settings window's root view and focuses it immediately
    /// so Escape closes the window without an extra click first.
    pub fn new(window: &mut Window, cx: &mut Context<Self>,
               settings: Entity<SettingsController>, library: Entity<LibraryController>,
               file_opener_extension_input: Entity<InputState>)
               -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        // Remember the size and place on every move or resize, as the main
        // window does, so the next opening is the same.
        cx.observe_window_bounds(window, |this, window, cx| {
              save_settings_window_bounds_soon(window, &this.bounds_debounce, cx);
          })
          .detach();
        // The editor shows the active theme's file, so it follows theme changes
        // (a pick in the gallery, or the file changing on disk).
        cx.observe_global_in::<LibriTheme>(window, |this, window, cx| {
              this.load_theme_editor(window, cx);
          })
          .detach();
        let view = Self { settings,
                          library,
                          file_opener_extension_input,
                          focus_handle,
                          bounds_debounce: Debounce::new() };
        view.load_theme_editor(window, cx);
        view
    }

    /// Fills the theme editor with the active theme's file, if the editor is
    /// attached.
    fn load_theme_editor(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(editor) = self.settings.read(cx).theme_editor_input() {
            load_theme_editor(&editor, window, cx);
            self.settings
                .update(cx, |ctrl, cx| ctrl.set_theme_editor_error(None, cx));
        }
    }
}

impl Focusable for SettingsWindowView {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SettingsWindowView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<LibriTheme>().clone();
        let colors = &theme.colors;
        // Adjusting the shared rem size scales every `rems(...)`-based size
        // utility (`.text_sm()`, `.text_xs()`, etc.) in this window, like
        // zooming a page — see `LibraryController::set_ui_text_size`.
        window.set_rem_size(theme.fonts.ui_text_size);
        let snap = self.settings.read(cx).snapshot();

        // One-shot focus request from the "Login to DriveThruRPG" banner
        // action (`LibraryRootView::show_settings_focused_on_email`) — must
        // run here rather than at the click site, since that site only has
        // the main window's `Window`, not this settings window's.
        let focus_email_pending = self.settings
                                      .update(cx, |ctrl, _cx| ctrl.take_focus_email_pending());
        if focus_email_pending
           && !snap.auth.is_logged_in
           && let Some(email_state) = snap.email_input.clone()
        {
            email_state.update(cx, |input, cx| input.focus(window, cx));
        }

        let sign_in_enabled = !snap.sign_in_in_progress
                              && !snap.email_draft.trim().is_empty()
                              && !snap.password_draft.trim().is_empty();

        // Only the page on show needs its library-derived data; building the
        // rest on every frame is what made scrolling sluggish.
        let on_storage = snap.active_page_ix == STORAGE_PAGE_IX;
        let on_advanced = snap.active_page_ix == ADVANCED_PAGE_IX;
        let storage_display = if on_storage {
            StorageDisplay { default_root_id:  snap.default_root_id,
                             volumes:          snap.root_volumes.clone(),
                             title_counts:     self.library.read(cx).root_title_counts().clone(),
                             collection_names: self.library.read(cx).collection_names(), }
        }
        else {
            StorageDisplay::default()
        };
        let publisher_names = if on_storage {
            self.library.read(cx).publisher_names()
        }
        else {
            Vec::new()
        };
        let filter_value_sections = if on_advanced {
            self.library.read(cx).filter_value_sections()
        }
        else {
            Vec::new()
        };
        let storage = StoragePageData { roots: snap.storage_roots,
                                        rules: snap.storage_rules,
                                        display: storage_display,
                                        root_path_exists: snap.storage_path_exists,
                                        check_free_space: snap.check_free_space,
                                        create_collections: snap.create_collections,
                                        max_concurrent_downloads: snap.max_concurrent_downloads,
                                        max_concurrent_input:
                                            snap.max_concurrent_downloads_input,
                                        publisher_names,
                                        rule_editor: snap.rule_editor };
        let theme_editor = ThemeEditorData { input:    snap.theme_editor_input.clone(),
                                             error:    snap.theme_editor_error.clone(),
                                             settings: self.settings.clone(), };
        let panel =
            render_settings_panel(&snap.file_openers,
                                  snap.auth,
                                  storage,
                                  AgentPageData { settings:     snap.mcp.clone(),
                                                  activity:     snap.mcp_activity.clone(),
                                                  command_path: snap.mcp_command_path.clone(),
                                                  bind_display: snap.mcp_bind_display.clone(),
                                                  log_path:     snap.mcp_log_path.clone(),
                                                  summaries:    snap.summaries.clone(),
                                                  summary_page: snap.summary_page.clone(), },
                                  theme_editor,
                                  self.settings.clone(),
                                  self.library.clone(),
                                  &self.focus_handle,
                                  colors,
                                  &theme,
                                  snap.email_input,
                                  snap.password_input,
                                  snap.sign_in_in_progress,
                                  sign_in_enabled,
                                  snap.sign_in_error,
                                  self.file_opener_extension_input.clone(),
                                  snap.pending_file_opener,
                                  snap.active_page_ix,
                                  snap.recently_updated_window_days,
                                  snap.recently_updated_window_input,
                                  snap.native,
                                  filter_value_sections,
                                  snap.text_scale_input);

        div().size_full().child(panel)
    }
}
