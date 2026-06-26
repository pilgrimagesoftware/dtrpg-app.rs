//! Storage settings section: catalog root path display and folder picker.
//!
//! StorageConfig integration is stubbed until `catalog-storage-location` is complete.

use gpui::{div, px, Entity, IntoElement, ParentElement, Styled};

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;

/// Renders the Storage settings section.
pub fn render_storage_section(
    _entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let border = colors.border;
    let surface_alt = colors.surface_alt;

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .p(px(24.0))
        // ── Section header ────────────────────────────────────────────────
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child("Catalog Storage Location"),
        )
        // ── Path display ──────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .h(px(34.0))
                        .px(px(12.0))
                        .rounded(px(8.0))
                        .border_1()
                        .border_color(border)
                        .bg(surface_alt)
                        .flex()
                        .items_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(text_secondary)
                                .truncate()
                                // Placeholder: real path comes from StorageConfig
                                .child("~/Library/Application Support/DTRPG"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(text_tertiary)
                        .child(
                            "\"Change…\" and \"Show in Finder\" are available once \
                             catalog-storage-location is connected.",
                        ),
                ),
        )
        // ── Divider ───────────────────────────────────────────────────────
        .child(div().h(px(1.0)).bg(border))
        // ── Actions ───────────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .gap(px(12.0))
                .child(render_outline_button("Change…", text_primary, border))
                .child(render_outline_button("Show in Finder", text_primary, border)),
        )
        .child(
            div()
                .text_xs()
                .text_color(text_tertiary)
                .child(
                    "Changing the storage location will not move existing downloaded files.",
                ),
        )
}

fn render_outline_button(label: &'static str, text: gpui::Hsla, border: gpui::Hsla) -> impl IntoElement + 'static {
    div()
        .h(px(34.0))
        .px(px(16.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .child(div().text_sm().text_color(text).child(label))
}
