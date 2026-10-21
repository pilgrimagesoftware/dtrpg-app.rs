//! Storage-root settings controls.

use dtrpg_core::data::storage::roots::{RootKind, root_kind};
use gpui::Div;
use gpui::prelude::*;
use gpui_kit::component::tag::Tag;
use gpui_kit::component::tooltip::Tooltip;

use super::*;
use crate::view_models::storage_display::{StorageDisplay, free_space_label, titles_here_label};

/// Renders the configured roots and lets the user add or remove named roots.
pub(super) fn render_storage_roots(roots: Vec<StorageRoot>, display: &StorageDisplay,
                                   entity: Entity<SettingsController>, colors: &ColorTokens)
                                   -> impl IntoElement + 'static + use<> {
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let border = colors.border;
    let surface_alt = colors.surface_alt;
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child(t!("settings.storage_roots_title")),
                )
                .child(
                    Button::new("add-storage-root")
                        .ghost()
                        .outline()
                        .icon(IconName::Plus)
                        .tooltip(t!("settings.storage_add_root_tooltip").to_string())
                        .on_click({
                            let entity = entity.clone();
                            move |_, _, cx| {
                                let entity = entity.clone();
                                cx.spawn(async move |cx| {
                                    let Some(handle) =
                                        rfd::AsyncFileDialog::new().pick_folder().await
                                    else {
                                        return;
                                    };
                                    let path = handle.path().to_path_buf();
                                    let name = path
                                        .file_name()
                                        .and_then(|name| name.to_str())
                                        .filter(|name| !name.is_empty())
                                        .unwrap_or("Storage")
                                        .to_owned();
                                    if let Err(error) = entity.update(cx, |ctrl, cx| {
                                        ctrl.add_storage_root(name, path, cx)
                                    }) {
                                        tracing::warn!(%error, "storage root rejected");
                                    }
                                })
                                .detach();
                            }
                        }),
                ),
        )
        .child(render_default_location(&roots, display.default_root_id, &entity, colors))
        .children(roots.into_iter().map(|root| {
            let root_id = root.id;
            let can_remove = root_id != dtrpg_core::data::storage::DEFAULT_ROOT_ID;
            let entity = entity.clone();
            let kind = root_kind(&root.path);
            let (free, total) = display.volumes.get(&root_id).copied().unwrap_or_default();
            let details = [
                free_space_label(free, total),
                Some(titles_here_label(display.title_counts.get(&root_id).copied().unwrap_or(0))),
            ];
            let details = details.into_iter().flatten().collect::<Vec<_>>().join(" · ");
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .p(px(8.0))
                .rounded(px(6.0))
                .border_1()
                .border_color(border)
                .bg(surface_alt)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(div().text_sm().text_color(text_primary).child(root.name))
                                .children(kind_badge(kind)),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_secondary)
                                .child(root.path.to_string_lossy().into_owned())
                                .truncate(),
                        )
                        .child(div().text_xs().text_color(text_secondary).child(details)),
                )
                .when(can_remove, |row| {
                    row.child(
                        Button::new(SharedString::from(format!("remove-storage-root-{root_id}")))
                            .ghost()
                            .icon(IconName::Close)
                            .tooltip(t!("settings.storage_remove_root_tooltip").to_string())
                            .on_click(move |_, _, cx| {
                                entity.update(cx, |ctrl, cx| {
                                    ctrl.remove_storage_root(root_id, cx);
                                });
                            }),
                    )
                })
        }))
}

/// The EXTERNAL or CLOUD marker for a location, or nothing for a local one.
fn kind_badge(kind: RootKind) -> Option<impl IntoElement> {
    let (label, tooltip) = match kind {
        RootKind::Local => return None,
        RootKind::External => {
            (t!("settings.storage_badge_external"), t!("settings.storage_badge_external_tooltip"))
        }
        RootKind::Cloud => {
            (t!("settings.storage_badge_cloud"), t!("settings.storage_badge_cloud_tooltip"))
        }
    };
    let tooltip = tooltip.to_string();
    Some(div().id(SharedString::from(format!("root-kind-{label}")))
              .tooltip(move |window, cx| Tooltip::new(tooltip.clone()).build(window, cx))
              .child(Tag::secondary().outline().child(label.to_string())))
}

/// The row that picks where downloads go when no placement rule matches.
fn render_default_location(roots: &[StorageRoot], default_root_id: u32,
                           entity: &Entity<SettingsController>, colors: &ColorTokens)
                           -> Div {
    div().flex()
         .items_center()
         .gap(px(6.0))
         .child(div().flex_1()
                     .flex()
                     .flex_col()
                     .child(div().text_sm()
                                 .text_color(colors.text_primary)
                                 .child(t!("settings.storage_default_location")))
                     .child(div().text_xs()
                                 .text_color(colors.text_secondary)
                                 .child(t!("settings.storage_default_location_note"))))
         .children(roots.iter().map(|root| {
                                   let id = root.id;
                                   let entity = entity.clone();
                                   Button::new(SharedString::from(format!("default-root-{id}")))
                                       .ghost()
                                       .selected(id == default_root_id)
                                       .child(root.name.clone())
                                       .on_click(move |_, _, cx| {
                                           entity.update(cx, |ctrl, cx| {
                                                     ctrl.set_default_root(id, cx);
                                                 });
                                       })
                               }))
}
