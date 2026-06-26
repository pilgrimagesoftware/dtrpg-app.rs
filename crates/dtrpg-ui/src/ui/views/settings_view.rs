//! Settings panel overlay: tab strip + active section content.

use gpui::prelude::*;
use gpui::{div, px, AnyElement, Entity, FocusHandle, IntoElement, ParentElement, Styled};

use crate::controllers::settings::{SettingsController, SettingsTab};
use crate::data::file_openers::FileOpenerEntry;
use crate::data::theme::ColorTokens;
use crate::ui::views::{
    settings_account_view::render_account_section,
    settings_file_openers_view::render_file_openers_section,
    settings_storage_view::render_storage_section,
};

// ── Public render entry point ─────────────────────────────────────────────────

/// Renders the settings panel overlay when settings are open.
///
/// The returned element is positioned absolute and fills its containing block,
/// which must have `position: relative` set.  The sidebar is outside that
/// container so it remains visible.
pub fn render_settings_panel(
    active_tab: SettingsTab,
    file_openers: &[FileOpenerEntry],
    is_authenticated: bool,
    entity: Entity<SettingsController>,
    focus_handle: &FocusHandle,
    colors: &ColorTokens,
) -> AnyElement {
    let surface = colors.surface;
    let border = colors.border;
    let text_primary = colors.text_primary;
    let backdrop = gpui::hsla(0.0, 0.0, 0.0, 0.35);

    let entity_close = entity.clone();
    let entity_tab = entity.clone();
    let entity_escape = entity.clone();

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
        // ── Modal card ────────────────────────────────────────────────────
        .child(
            div()
                .w(px(560.0))
                .h(px(440.0))
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
                                .child("Settings"),
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
                                .child("✕"),
                        ),
                )
                // ── Tab strip ─────────────────────────────────────────────
                .child(render_tab_strip(active_tab, entity_tab, colors))
                // ── Section content ───────────────────────────────────────
                .child(
                    div()
                        .flex_1()
                        .min_h_0()
                        .overflow_y_hidden()
                        .child(render_active_section(active_tab, file_openers, is_authenticated, entity, colors)),
                ),
        )
        .into_any_element()
}

// ── Tab strip ─────────────────────────────────────────────────────────────────

fn render_tab_strip(
    active_tab: SettingsTab,
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let border = colors.border;
    let accent = colors.accent;
    let text_tertiary = colors.text_tertiary;
    let accent_soft = colors.accent_soft;

    let tabs = [SettingsTab::Account, SettingsTab::Storage, SettingsTab::FileOpeners];

    let mut strip = div()
        .h(px(40.0))
        .flex_none()
        .flex()
        .items_center()
        .border_b_1()
        .border_color(border)
        .px(px(4.0));

    for tab in tabs {
        let is_active = tab == active_tab;
        let e = entity.clone();
        let text_color = if is_active { accent } else { text_tertiary };
        let bg = if is_active { accent_soft } else { gpui::hsla(0.0, 0.0, 0.0, 0.0) };

        strip = strip.child(
            div()
                .id(tab.label())
                .h(px(32.0))
                .px(px(14.0))
                .mx(px(2.0))
                .rounded(px(6.0))
                .bg(bg)
                .flex()
                .items_center()
                .cursor_pointer()
                .on_click(move |_, _, cx| {
                    e.update(cx, |ctrl, cx| ctrl.set_tab(tab, cx));
                })
                .child(
                    div()
                        .text_sm()
                        .font_weight(if is_active {
                            gpui::FontWeight::SEMIBOLD
                        } else {
                            gpui::FontWeight::NORMAL
                        })
                        .text_color(text_color)
                        .child(tab.label()),
                ),
        );
    }

    strip
}

// ── Active section ────────────────────────────────────────────────────────────

fn render_active_section(
    active_tab: SettingsTab,
    file_openers: &[FileOpenerEntry],
    is_authenticated: bool,
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> AnyElement {
    match active_tab {
        SettingsTab::Account => render_account_section(is_authenticated, entity, colors).into_any_element(),
        SettingsTab::Storage => render_storage_section(entity, colors).into_any_element(),
        SettingsTab::FileOpeners => {
            render_file_openers_section(file_openers, entity, colors).into_any_element()
        }
    }
}
