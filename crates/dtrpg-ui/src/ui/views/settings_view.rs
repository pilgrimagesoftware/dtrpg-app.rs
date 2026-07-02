//! Settings panel overlay: sidebar navigation + per-page content via gpui-component Settings.

use std::path::PathBuf;

use gpui::prelude::*;
use gpui::{AnyElement, Entity, FocusHandle, IntoElement, ParentElement, Styled, div, px};
use gpui_component::input::InputState;
use gpui_component::setting::{SettingGroup, SettingItem, SettingPage, Settings};

use crate::controllers::settings::{AuthStateSnapshot, SettingsController};
use crate::data::file_openers::FileOpenerEntry;
use crate::data::theme::ColorTokens;
use crate::ui::views::{
    settings_account_view::render_account_section,
    settings_advanced_view::{render_about_section, render_advanced_section},
    settings_file_openers_view::render_file_openers_section,
    settings_storage_view::render_storage_section,
};
use rust_i18n::t;

// ── Public render entry point ─────────────────────────────────────────────────

/// Renders the settings panel overlay when settings are open.
///
/// The returned element is positioned absolute and fills its containing block,
/// which must have `position: relative` set. The sidebar is outside that
/// container so it remains visible.
#[allow(clippy::too_many_arguments)]
pub fn render_settings_panel(
    file_openers: &[FileOpenerEntry],
    auth: AuthStateSnapshot,
    storage_root_path: PathBuf,
    storage_path_exists: bool,
    entity: Entity<SettingsController>,
    focus_handle: &FocusHandle,
    colors: &ColorTokens,
    api_key_input: Option<Entity<InputState>>,
    email_input: Option<Entity<InputState>>,
    sign_in_in_progress: bool,
    sign_in_error: Option<String>,
    storage_path_input: Option<Entity<InputState>>,
    file_opener_extension_input: Entity<InputState>,
    pending_file_opener: Option<PathBuf>,
) -> AnyElement {
    let surface = colors.surface;
    let border = colors.border;
    let text_primary = colors.text_primary;
    let backdrop = gpui::hsla(0.0, 0.0, 0.0, 0.35);

    let entity_close = entity.clone();
    let entity_escape = entity.clone();

    // ── Capture data for each page's render closure ───────────────────────────

    let account_entity = entity.clone();
    let account_auth = auth.clone();
    let account_colors = colors.clone();
    let account_api_key_input = api_key_input.clone();
    let account_email_input = email_input.clone();

    let storage_entity = entity.clone();
    let storage_path = storage_root_path.clone();
    let storage_colors = colors.clone();
    let storage_path_input = storage_path_input.clone();

    let file_openers_vec = file_openers.to_vec();
    let file_openers_entity = entity.clone();
    let file_openers_colors = colors.clone();
    let file_opener_extension_input = file_opener_extension_input.clone();
    let pending_file_opener = pending_file_opener.clone();

    let advanced_entity = entity.clone();
    let advanced_colors = colors.clone();
    let about_colors = colors.clone();

    // ── Build the Settings component ──────────────────────────────────────────

    let settings =
        Settings::new("settings-panel")
            .sidebar_width(px(160.0))
            .page(
                SettingPage::new(t!("settings.account_title")).group(SettingGroup::new().item(
                    SettingItem::render(move |_, _window, _cx| {
                        render_account_section(
                            &account_auth,
                            account_entity.clone(),
                            &account_colors,
                            account_api_key_input.clone(),
                            account_email_input.clone(),
                            sign_in_in_progress,
                            sign_in_error.clone(),
                        )
                    }),
                )),
            )
            .page(SettingPage::new(t!("settings.downloads_location")).group(
                SettingGroup::new().item(SettingItem::render(move |_, _window, _cx| {
                    render_storage_section(
                        storage_path.clone(),
                        storage_path_exists,
                        storage_entity.clone(),
                        &storage_colors,
                        storage_path_input.clone(),
                    )
                })),
            ))
            .page(SettingPage::new(t!("settings.file_openers_title")).group(
                SettingGroup::new().item(SettingItem::render(move |_, _window, _cx| {
                    render_file_openers_section(
                        &file_openers_vec,
                        file_openers_entity.clone(),
                        &file_openers_colors,
                        file_opener_extension_input.clone(),
                        pending_file_opener.clone(),
                    )
                })),
            ))
            .page(
                SettingPage::new(t!("settings.advanced_title")).group(SettingGroup::new().item(
                    SettingItem::render(move |_, _window, _cx| {
                        render_advanced_section(advanced_entity.clone(), &advanced_colors)
                    }),
                )),
            )
            .page(
                SettingPage::new(t!("settings.about_title")).group(SettingGroup::new().item(
                    SettingItem::render(move |_, _window, _cx| render_about_section(&about_colors)),
                )),
            );

    div()
        .id("settings-backdrop")
        .track_focus(focus_handle)
        .occlude()
        .on_key_down(move |event, _window, cx| {
            if event.keystroke.key == "escape" {
                entity_escape.update(cx, |ctrl, cx| ctrl.close(cx));
            }
        })
        .absolute()
        .inset_0()
        .bg(backdrop)
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(720.0))
                .h(px(480.0))
                .bg(surface)
                .border_1()
                .border_color(border)
                .rounded(px(12.0))
                .shadow_lg()
                .flex()
                .flex_col()
                .overflow_hidden()
                // ── Title bar ─────────────────────────────────────────────
                .child(
                    div()
                        .h(px(48.0))
                        .flex_none()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px(px(20.0))
                        .border_b_1()
                        .border_color(border)
                        .child(
                            div()
                                .text_base()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(text_primary)
                                .child(t!("settings.title")),
                        )
                        .child(
                            div()
                                .id("settings-close")
                                .size(px(24.0))
                                .rounded_full()
                                .bg(colors.hover)
                                .flex()
                                .items_center()
                                .justify_center()
                                .cursor_pointer()
                                .text_sm()
                                .text_color(colors.text_secondary)
                                .on_click(move |_, _, cx| {
                                    entity_close.update(cx, |ctrl, cx| ctrl.close(cx));
                                })
                                .child("x"),
                        ),
                )
                // ── Settings component (sidebar + active page) ────────────
                .child(div().flex_1().min_h_0().child(settings)),
        )
        .into_any_element()
}
