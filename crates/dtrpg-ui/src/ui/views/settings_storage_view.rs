//! Downloads settings section: download root path display, folder picker, and
//! reveal action.

use std::path::PathBuf;

use gpui::{
    AnyElement, Element, Entity, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, div, px,
};
use gpui_component::input::{Input, InputState};
use gpui_component::tooltip::Tooltip;
use rust_i18n::t;

use crate::controllers::settings::SettingsController;
use crate::data::storage::validate_writable;
use crate::data::theme::ColorTokens;
use crate::ui::widgets::selectable_text;

/// Renders the Storage settings section.
///
/// Displays the current `storage_root_path`, inline icon buttons for "Change…"
/// and "Show in Finder/Explorer/Files", and an optional warning row when
/// `storage_path_exists` is `false`.
pub fn render_storage_section(storage_root_path: PathBuf, storage_path_exists: bool,
                              entity: Entity<SettingsController>, colors: &ColorTokens,
                              storage_path_input: Option<Entity<InputState>>)
                              -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let border = colors.border;
    let surface_alt = colors.surface_alt;
    let warning_bg = colors.warning_bg;
    let warning_text = colors.warning_text;

    let path_display = storage_root_path.to_string_lossy().into_owned();
    let entity_change = entity.clone();
    let entity_reveal = entity.clone();

    let reveal_label = platform_reveal_label().into_owned();

    let mut section = div()
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
                .child(t!("settings.storage_title")),
        )
        // ── Path row with inline action buttons ───────────────────────────
        .child(
            div().flex().flex_col().gap(px(6.0)).child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    // Path input field
                    .child({
                        let path_el: AnyElement = if let Some(input_state) = storage_path_input {
                            Input::new(&input_state)
                                .appearance(true)
                                .into_element()
                                .into_any()
                        } else {
                            div()
                                .flex_1()
                                .min_w_0()
                                .h(px(34.0))
                                .px(px(12.0))
                                .rounded(px(8.0))
                                .border_1()
                                .border_color(border)
                                .bg(surface_alt)
                                .flex()
                                .items_center()
                                .child(
                                    selectable_text("settings-storage-path", path_display)
                                        .text_sm()
                                        .text_color(text_secondary)
                                        .truncate(),
                                )
                                .into_any()
                        };
                        path_el
                    })
                    // "Change…" icon button
                    .child(
                        div()
                            .id("change-storage")
                            .flex_none()
                            .size(px(32.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(border)
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .tooltip(|window, cx| {
                                Tooltip::new(t!("settings.storage_change_tooltip").to_string())
                                    .build(window, cx)
                            })
                            .on_click(move |_event, _window, cx| {
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
                            })
                            .child(div().text_sm().text_color(text_primary).child("📂")),
                    )
                    // "Show in Finder/Explorer/Files" icon button
                    .child(
                        div()
                            .id("reveal-storage")
                            .flex_none()
                            .size(px(32.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(border)
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .tooltip(move |window, cx| {
                                Tooltip::new(reveal_label.clone()).build(window, cx)
                            })
                            .on_click(move |_event, _window, cx| {
                                entity_reveal.read(cx).reveal_storage_location();
                            })
                            .child(div().text_sm().text_color(text_primary).child("↗")),
                    ),
            ),
        );

    // ── Missing-path warning ──────────────────────────────────────────────
    if !storage_path_exists {
        section = section.child(div().rounded(px(6.0))
                                     .px(px(10.0))
                                     .py(px(6.0))
                                     .bg(warning_bg)
                                     .flex()
                                     .items_center()
                                     .gap(px(6.0))
                                     .child(div().text_sm().text_color(warning_text).child("⚠"))
                                     .child(div().text_xs()
                                                 .text_color(warning_text)
                                                 .child(t!("settings.storage_missing"))));
    }

    section
           // ── Divider ───────────────────────────────────────────────────────
           .child(div().h(px(1.0)).bg(border))
           // ── Note ─────────────────────────────────────────────────────────
           .child(div().text_xs()
                       .text_color(gpui::hsla(0.08, 0.9, 0.55, 1.0))
                       .child(format!("\u{26A0} {}", t!("settings.storage_note"))))
}

// ── Helpers
// ───────────────────────────────────────────────────────────────────

fn platform_reveal_label() -> std::borrow::Cow<'static, str> {
    #[cfg(target_os = "macos")]
    return t!("detail.show_in_finder");
    #[cfg(target_os = "windows")]
    return t!("detail.show_in_explorer");
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return t!("detail.show_in_files");
}
