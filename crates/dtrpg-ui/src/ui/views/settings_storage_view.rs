//! Storage settings section: catalog root path display, folder picker, and reveal action.

use std::path::PathBuf;

use gpui::{div, px, Entity, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled};

use crate::controllers::settings::SettingsController;
use crate::data::storage::validate_writable;
use crate::data::theme::ColorTokens;

/// Renders the Storage settings section.
///
/// Displays the current `storage_root_path`, a "Change…" button that opens the OS
/// folder picker, and a "Show in Finder/Explorer/Files" button that reveals the
/// storage root in the file manager.
pub fn render_storage_section(
    storage_root_path: PathBuf,
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let border = colors.border;
    let surface_alt = colors.surface_alt;

    let path_display = storage_root_path.to_string_lossy().into_owned();
    let entity_change = entity.clone();
    let entity_reveal = entity;

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
                                .child(path_display),
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
                .child(render_action_button(
                    "change-storage",
                    "Change\u{2026}",
                    text_primary,
                    border,
                    move |_event, _window, cx| {
                        let picked = rfd::FileDialog::new().pick_folder();
                        if let Some(path) = picked {
                            match validate_writable(&path) {
                                Ok(()) => {
                                    entity_change.update(cx, |ctrl, cx| {
                                        if let Err(e) = ctrl.apply_storage_path(path, cx) {
                                            tracing::warn!("storage path rejected: {e}");
                                        }
                                    });
                                }
                                Err(e) => tracing::warn!("storage path not writable: {e}"),
                            }
                        }
                    },
                ))
                .child(render_action_button(
                    "reveal-storage",
                    platform_reveal_label(),
                    text_primary,
                    border,
                    move |_event, _window, cx| {
                        entity_reveal.read(cx).reveal_storage_location();
                    },
                )),
        )
        // ── Warning ───────────────────────────────────────────────────────
        .child(
            div()
                .text_xs()
                .text_color(text_tertiary)
                .child("Changing the storage location will not move existing downloaded files."),
        )
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn platform_reveal_label() -> &'static str {
    #[cfg(target_os = "macos")]
    return "Show in Finder";
    #[cfg(target_os = "windows")]
    return "Show in Explorer";
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return "Show in Files";
}

fn render_action_button(
    id: &'static str,
    label: &'static str,
    text: gpui::Hsla,
    border: gpui::Hsla,
    on_click: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
) -> impl IntoElement + 'static {
    div()
        .id(id)
        .h(px(34.0))
        .px(px(16.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(border)
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .on_click(on_click)
        .child(div().text_sm().text_color(text).child(label))
}
