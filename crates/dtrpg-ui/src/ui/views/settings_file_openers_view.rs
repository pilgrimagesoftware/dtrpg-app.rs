//! File Openers settings section: CRUD for extension → application overrides.

use std::path::PathBuf;

use gpui::prelude::*;
use gpui::{AnyElement, Entity, IntoElement, ParentElement, Styled, div, px};
use gpui_component::WindowExt as _;
use gpui_component::tooltip::Tooltip;

use crate::controllers::settings::SettingsController;
use crate::data::file_openers::FileOpenerEntry;
use crate::data::theme::ColorTokens;
use rust_i18n::t;

/// Renders the File Openers settings section.
pub fn render_file_openers_section(
    file_openers: &[FileOpenerEntry],
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_tertiary = colors.text_tertiary;
    let border = colors.border;
    let accent = colors.accent;
    let accent_on = colors.accent_on;

    // Collect extensions whose app path no longer exists on disk.
    let stale_extensions: Vec<String> = file_openers
        .iter()
        .filter(|e| !e.app_path.exists())
        .map(|e| e.extension.clone())
        .collect();

    let mut col = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .p(px(24.0))
        // ── Header row ────────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(text_primary)
                                .child(t!("settings.file_openers_title")),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_tertiary)
                                .child(t!("settings.file_openers_description")),
                        ),
                )
                .child(render_add_button(entity.clone(), accent, accent_on)),
        )
        // ── Divider ───────────────────────────────────────────────────────
        .child(div().h(px(1.0)).bg(border));

    if file_openers.is_empty() {
        col = col.child(render_empty_state(text_tertiary));
    } else {
        for entry in file_openers {
            let is_stale = stale_extensions.contains(&entry.extension);
            col = col.child(render_entry_row(entry, is_stale, entity.clone(), colors));
        }
    }

    col
}

// ── Empty state ───────────────────────────────────────────────────────────────

fn render_empty_state(text_tertiary: gpui::Hsla) -> impl IntoElement + 'static {
    div()
        .flex()
        .items_center()
        .justify_center()
        .py(px(32.0))
        .child(
            div()
                .text_sm()
                .text_color(text_tertiary)
                .child(t!("settings.file_openers_empty")),
        )
}

// ── Entry row ─────────────────────────────────────────────────────────────────

fn render_entry_row(
    entry: &FileOpenerEntry,
    is_stale: bool,
    entity: Entity<SettingsController>,
    colors: &ColorTokens,
) -> AnyElement {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let border = colors.border;

    let extension = entry.extension.clone();
    let app_name = app_name_from_path(&entry.app_path);
    let ext_label = format!(".{extension}");

    let entity_remove = entity.clone();
    let extension_for_remove = extension.clone();

    div()
        .flex()
        .items_center()
        .gap(px(12.0))
        .py(px(8.0))
        .border_b_1()
        .border_color(border)
        // ── Extension badge ───────────────────────────────────────────────
        .child(
            div()
                .px(px(8.0))
                .py(px(3.0))
                .rounded(px(6.0))
                .border_1()
                .border_color(border)
                .text_xs()
                .font_weight(gpui::FontWeight::MEDIUM)
                .text_color(text_primary)
                .child(ext_label),
        )
        // ── App name ──────────────────────────────────────────────────────
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .truncate()
                        .child(app_name),
                )
                .when(is_stale, |el| {
                    el.child(
                        div()
                            .text_xs()
                            .text_color(gpui::hsla(0.08, 0.9, 0.55, 1.0)) // amber warning
                            .child("⚠ App not found"),
                    )
                }),
        )
        // ── Remove button ─────────────────────────────────────────────────
        .child(
            div()
                .id(format!("remove-opener-{extension}"))
                .size(px(28.0))
                .rounded(px(6.0))
                .border_1()
                .border_color(border)
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .tooltip(|window, cx| Tooltip::new("Remove").build(window, cx))
                .on_click(move |_, window, cx| {
                    let ext = extension_for_remove.clone();
                    let entity = entity_remove.clone();
                    window.open_alert_dialog(cx, move |alert, _, _| {
                        let ext2 = ext.clone();
                        let entity2 = entity.clone();
                        alert
                            .confirm()
                            .title(format!("Remove .{ext} opener?"))
                            .description("This file opener entry will be deleted.")
                            .on_ok(move |_, _window, cx| {
                                entity2.update(cx, |ctrl, cx| {
                                    ctrl.remove_file_opener(&ext2, cx);
                                });
                                true
                            })
                    });
                })
                .child(div().text_xs().text_color(text_tertiary).child("\u{00d7}")),
        )
        .into_any_element()
}

// ── Add button ────────────────────────────────────────────────────────────────

fn render_add_button(
    entity: Entity<SettingsController>,
    accent: gpui::Hsla,
    accent_on: gpui::Hsla,
) -> impl IntoElement + 'static {
    div()
        .id("add-file-opener")
        .size(px(30.0))
        .rounded(px(8.0))
        .bg(accent)
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .tooltip(|window, cx| Tooltip::new("Add file opener").build(window, cx))
        // Adding an entry requires a native app picker dialog (rfd crate, pending
        // open-item-in-default-app change). For now clicking opens a no-op stub.
        .on_click(move |_, _, cx| {
            // Stub: add a placeholder entry so the list renders correctly.
            let stub = FileOpenerEntry {
                extension: "example".to_owned(),
                app_path: PathBuf::from("/Applications/ExampleApp.app"),
            };
            entity.update(cx, |ctrl, cx| {
                ctrl.add_file_opener(stub, cx);
            });
        })
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::MEDIUM)
                .text_color(accent_on)
                .child("+"),
        )
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Derives a display-friendly application name from the path.
fn app_name_from_path(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}
