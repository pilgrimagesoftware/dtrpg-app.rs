//! Detail panel: slide-over showing full item metadata and actions.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, Entity, InteractiveElement, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement, Styled, div, px,
};
use gpui_component::Disableable;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::description_list::{DescriptionItem, DescriptionList};
use gpui_component::tooltip::Tooltip;

use crate::controllers::library::LibraryController;
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::theme::ColorTokens;
use crate::ui::library::cover::render_generative_cover;
use crate::util::datetime::{format_absolute, format_relative};
use crate::util::reveal::reveal_in_file_manager;

/// Renders the detail panel overlay if `selected_item` is `Some`; otherwise an empty div.
pub fn render_detail_panel(
    selected_item: Option<&LibraryItem>,
    storage_root_path: PathBuf,
    entity: Entity<LibraryController>,
    colors: &ColorTokens,
) -> AnyElement {
    let Some(item) = selected_item else {
        return div().into_any_element();
    };

    let surface = colors.surface;
    let border = colors.border;
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;
    let text_tertiary = colors.text_tertiary;
    let scrim = colors.scrim;
    let accent_on = colors.accent_on;

    let item = item.clone();
    let entity_close = entity.clone();
    let entity_download = entity.clone();
    let item_id = Arc::clone(&item.id);
    let reveal_item_id = Arc::clone(&item.id);
    let is_downloaded = item.status == ItemStatus::Downloaded;

    div()
        .id("detail-panel")
        .occlude()
        .absolute()
        .right_0()
        .top_0()
        .bottom_0()
        .w(px(320.0))
        .bg(surface)
        .border_l_1()
        .border_color(border)
        .flex()
        .flex_col()
        // Close button
        .child(
            div()
                .absolute()
                .top(px(12.0))
                .right(px(12.0))
                .id("detail-close")
                .size(px(24.0))
                .rounded_full()
                .bg(scrim)
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .text_sm()
                .text_color(accent_on)
                .on_click(move |_, _, cx| {
                    entity_close.update(cx, |ctrl, cx| ctrl.clear_selection(cx));
                })
                .child("✕"),
        )
        // Cover
        .child({
            let cover_w = 320.0_f32;
            let cover_h = cover_w * 10.0 / 7.0;
            div()
                .w(px(cover_w))
                .h(px(cover_h))
                .flex_none()
                .child(render_generative_cover(&item, cover_w, cover_h, true))
        })
        // Scrollable body
        .child(
            div()
                .flex_1()
                .min_h_0()
                .overflow_y_hidden()
                .p(px(20.0))
                .flex()
                .flex_col()
                .gap(px(16.0))
                // Publisher + title + line
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_tertiary)
                                .child(item.publisher.to_string()),
                        )
                        .child(
                            div()
                                .text_xl()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(text_primary)
                                .child(item.title.to_string()),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(text_secondary)
                                .child(item.line.to_string()),
                        ),
                )
                // Description
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child(item.desc.to_string()),
                )
                // Actions
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(
                            Button::new("detail-read")
                                .primary()
                                .label("Read")
                                .w_full()
                                .disabled(!is_downloaded)
                                .when(!is_downloaded, |b| b.tooltip("Download this item first")),
                        )
                        .child(
                            Button::new("detail-download")
                                .ghost()
                                .outline()
                                .label(if is_downloaded {
                                    "Downloaded"
                                } else {
                                    "Download"
                                })
                                .w_full()
                                .on_click(move |_, _, cx| {
                                    let id = Arc::clone(&item_id);
                                    entity_download.update(cx, |ctrl, cx| {
                                        ctrl.toggle_download(&id, cx);
                                    });
                                }),
                        )
                        .when(is_downloaded, |col| {
                            let item_path = storage_root_path
                                .join("items")
                                .join(reveal_item_id.as_ref());
                            col.child(
                                Button::new("detail-reveal")
                                    .ghost()
                                    .outline()
                                    .label(platform_reveal_label())
                                    .w_full()
                                    .on_click(move |_, _, _cx| {
                                        if !item_path.exists() {
                                            tracing::warn!(
                                                path = %item_path.display(),
                                                "reveal: file not found — item may need re-download"
                                            );
                                            return;
                                        }
                                        if let Err(e) = reveal_in_file_manager(&item_path) {
                                            tracing::warn!("reveal_in_file_manager failed: {e}");
                                        }
                                    }),
                            )
                        }),
                )
                // Metadata table
                .child(render_metadata_table(&item, colors)),
        )
        .into_any_element()
}

fn platform_reveal_label() -> &'static str {
    #[cfg(target_os = "macos")]
    return "Show in Finder";
    #[cfg(target_os = "windows")]
    return "Show in Explorer";
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return "Show in Files";
}

fn render_metadata_table(
    item: &LibraryItem,
    _colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let status_str: String = match item.status {
        ItemStatus::Downloaded => "On this device".into(),
        ItemStatus::Cloud => "In the cloud".into(),
    };

    let mut list = DescriptionList::vertical()
        .columns(1)
        .child(DescriptionItem::new("System").value(item.line.to_string()))
        .child(DescriptionItem::new("Category").value(item.kind.to_string()))
        .child(DescriptionItem::new("Format").value(item.format.to_string()))
        .child(DescriptionItem::new("Pages").value(item.pages.to_string()))
        .child(DescriptionItem::new("File size").value(format!("{:.0} MB", item.size_mb)))
        .child(DescriptionItem::new("Released").value(item.year.to_string()))
        .child(DescriptionItem::new("Status").value(status_str));

    if let Some(ts) = item.date_added {
        let relative = format_relative(ts);
        let absolute = format_absolute(ts);
        let id = SharedString::from(format!("detail-added-{}", item.id));
        let value = div()
            .id(id)
            .child(relative)
            .tooltip(move |window, cx| Tooltip::new(absolute.clone()).build(window, cx))
            .into_any_element();
        list = list.child(DescriptionItem::new("Added").value(value));
    }

    list
}
