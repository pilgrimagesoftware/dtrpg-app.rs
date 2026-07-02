//! Advanced settings section: destructive/maintenance actions (currently just cache clearing).
//!
//! Also renders the About section, which is purely informational (app name, version,
//! description) and shares no state with Advanced beyond both living in the Settings panel.

use gpui::{Entity, IntoElement, ParentElement, Styled, div, px};
use gpui_component::WindowExt as _;
use gpui_component::button::{Button, ButtonVariants};

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use rust_i18n::t;

/// Renders the Advanced settings section: a "Clear cache" action with a confirmation dialog.
pub fn render_advanced_section(
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let border = colors.border;

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .p(px(24.0))
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(t!("settings.advanced_title")),
        )
        .child(div().h(px(1.0)).bg(border))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child(t!("settings.clear_cache_description")),
                )
                .child(
                    Button::new("clear-cache-btn")
                        .danger()
                        .label(t!("settings.clear_cache_button"))
                        .on_click(move |_, window, cx| {
                            let entity = entity.clone();
                            window.open_alert_dialog(cx, move |alert, _, _| {
                                let entity = entity.clone();
                                alert
                                    .confirm()
                                    .title(t!("settings.clear_cache_confirm_title").to_string())
                                    .description(
                                        t!("settings.clear_cache_confirm_description").to_string(),
                                    )
                                    .on_ok(move |_, _window, cx| {
                                        entity.update(cx, |ctrl, cx| ctrl.clear_cache(cx));
                                        true
                                    })
                            });
                        }),
                ),
        )
}

/// Renders the About settings section: app name, version, and a short description.
pub fn render_about_section(colors: &ColorTokens) -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;

    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .p(px(24.0))
        .child(
            div()
                .text_lg()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(t!("sidebar.app_name")),
        )
        .child(
            div()
                .text_sm()
                .text_color(text_secondary)
                .child(t!("about.version", version = env!("CARGO_PKG_VERSION"))),
        )
        .child(
            div()
                .text_xs()
                .text_color(text_secondary)
                .child(t!("about.description")),
        )
}
