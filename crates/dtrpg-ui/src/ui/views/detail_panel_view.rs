//! Expanded detail tab content: full item metadata and actions, filling a
//! tab's content area (opened by double-clicking a catalog item).

use std::path::PathBuf;
use std::sync::Arc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, Entity, Image, InteractiveElement, IntoElement, ObjectFit, ParentElement,
    SharedString, StatefulInteractiveElement, Styled, StyledImage, div, img, px,
};
use gpui_component::Disableable;
use gpui_component::Icon;
use gpui_component::IconName;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::description_list::{DescriptionItem, DescriptionList};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::tooltip::Tooltip;
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::theme::ColorTokens;
use crate::ui::library::cover::render_generative_cover;
use crate::util::datetime::{format_absolute, format_relative};
use crate::util::reveal::reveal_in_file_manager;

/// Renders the expanded detail tab's content: a large cover, title,
/// description, actions, and metadata, filling the tab's content area.
///
/// Has no absolute positioning, resize handle, or close button of its own —
/// it's opened as a full tab (double-click on a catalog item, see
/// `main-window-tabs`) and closed via the tab strip.
///
/// Does not render a file list for multi-item entries: that requires a
/// per-item file list data model this crate does not yet have (tracked as a
/// known gap in the `add-rust-main-window-structure` change).
pub fn render_detail_tab_content(item: &LibraryItem, storage_root_path: PathBuf,
                                 entity: Entity<LibraryController>, colors: &ColorTokens,
                                 cover_image: Option<Arc<Image>>)
                                 -> AnyElement {
    let surface = colors.surface;
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;

    let item = item.clone();
    let entity_download = entity.clone();
    let entity_refresh_thumbnail = entity;
    let item_id = Arc::clone(&item.id);
    let reveal_item_id = Arc::clone(&item.id);
    let read_item_id = Arc::clone(&item.id);
    let is_downloaded = item.status == ItemStatus::Downloaded;

    let cover_w = crate::data::constants::DETAIL_PANEL_COVER_MAX_WIDTH * 1.5;
    let cover_h = cover_w * 10.0 / 7.0;
    let cover: AnyElement = if let Some(image) = cover_image {
        img(image).w(px(cover_w))
                  .h(px(cover_h))
                  .object_fit(ObjectFit::Cover)
                  .into_any_element()
    }
    else {
        render_generative_cover(&item, cover_w, cover_h, true).into_any_element()
    };
    let cover_url = item.cover_url.clone();

    div()
        .id("detail-tab-content")
        .flex_1()
        .min_h_0()
        .min_w_0()
        .flex()
        .bg(surface)
        .child({
            let mut cover_box = div().relative().w(px(cover_w)).flex_none().child(cover);
            if let Some(cover_url) = cover_url {
                cover_box = cover_box.child(
                    div()
                        .id("detail-tab-refresh-thumbnail")
                        .absolute()
                        .top(px(8.0))
                        .left(px(8.0))
                        .size(px(24.0))
                        .rounded_full()
                        .bg(colors.scrim)
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .text_sm()
                        .text_color(colors.accent_on)
                        .tooltip(|window, cx| {
                            Tooltip::new(t!("detail.refresh_thumbnail_tooltip").to_string())
                                .build(window, cx)
                        })
                        .on_click(move |_, _, cx| {
                            entity_refresh_thumbnail
                                .update(cx, |ctrl, cx| ctrl.load_thumbnail(cover_url.clone(), cx));
                        })
                        .child("\u{27f3}"),
                );
            }
            div().flex_none().pr(px(16.0)).py(px(16.0)).child(cover_box)
        })
        .child(
            div().flex_1().min_h_0().flex().flex_col().child(
                div()
                    .overflow_y_scrollbar()
                    .p(px(20.0))
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.text_tertiary)
                                    .child(item.publisher.to_string()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_color(text_primary)
                                            .child(item.title.to_string()),
                                    )
                                    .child(render_status_icon(is_downloaded, text_secondary)),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(text_secondary)
                                    .child(item.line.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary)
                            .child(item.desc.to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child({
                                let read_path =
                                    storage_root_path.join("items").join(read_item_id.as_ref());
                                Button::new("detail-tab-read")
                                    .primary()
                                    .icon(IconName::BookOpen)
                                    .disabled(!is_downloaded)
                                    .when(!is_downloaded, |b| {
                                        b.tooltip(format!("{}\n{}",
                                            t!("detail.read_button"),
                                            t!("detail.tooltip_download_first")))
                                    })
                                    .when(is_downloaded, |b| {
                                        b.tooltip(t!("detail.read_button")).on_click(
                                            move |_, _, _| {
                                                use crate::util::item_opener::{
                                                    ItemOpener, OpenError,
                                                };

                                                if !read_path.exists() {
                                                    tracing::warn!(
                                                        path = %read_path.display(),
                                                        "open: file not found — item may need re-download"
                                                    );
                                                    return;
                                                }
                                                if let Err(e) = ItemOpener::open(&read_path) {
                                                    match e {
                                                        OpenError::FileNotFound(path) => {
                                                            tracing::warn!(
                                                                "open: file not found: {path}"
                                                            );
                                                        }
                                                        OpenError::NoDefaultApp => {
                                                            tracing::warn!(
                                                                "open: no default application configured"
                                                            );
                                                        }
                                                        OpenError::OsFailed(msg) => {
                                                            tracing::warn!("open: OS failed: {msg}");
                                                        }
                                                        OpenError::MultipleFilesRequireSelection => {
                                                            tracing::warn!(
                                                                "open: multiple files require selection"
                                                            );
                                                        }
                                                    }
                                                }
                                            },
                                        )
                                    })
                            })
                            .child(
                                Button::new("detail-tab-download")
                                    .ghost()
                                    .outline()
                                    .icon(if is_downloaded {
                                        IconName::CircleCheck
                                    } else {
                                        IconName::ArrowDown
                                    })
                                    .tooltip(if is_downloaded {
                                        t!("detail.downloaded_button")
                                    } else {
                                        t!("detail.download_button")
                                    })
                                    .on_click(move |_, _, cx| {
                                        let id = Arc::clone(&item_id);
                                        entity_download.update(cx, |ctrl, cx| {
                                            ctrl.toggle_download(&id, cx);
                                        });
                                    }),
                            )
                            .when(is_downloaded, |row| {
                                let item_path =
                                    storage_root_path.join("items").join(reveal_item_id.as_ref());
                                row.child(
                                    Button::new("detail-tab-reveal")
                                        .ghost()
                                        .outline()
                                        .icon(IconName::FolderOpen)
                                        .tooltip(platform_reveal_label().into_owned())
                                        .on_click(move |_, _, _cx| {
                                            if !item_path.exists() {
                                                tracing::warn!(
                                                    path = %item_path.display(),
                                                    "reveal: file not found — item may need re-download"
                                                );
                                                return;
                                            }
                                            if let Err(e) = reveal_in_file_manager(&item_path) {
                                                tracing::warn!(
                                                    "reveal_in_file_manager failed: {e}"
                                                );
                                            }
                                        }),
                                )
                            }),
                    )
                    .child(render_metadata_table(&item, colors)),
            ),
        )
        .into_any_element()
}

fn platform_reveal_label() -> std::borrow::Cow<'static, str> {
    #[cfg(target_os = "macos")]
    return t!("detail.show_in_finder");
    #[cfg(target_os = "windows")]
    return t!("detail.show_in_explorer");
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return t!("detail.show_in_files");
}

/// Renders a small status icon next to the item title: a checkmark when
/// downloaded, a cloud glyph otherwise. Replaces the old text-only "Status" row
/// in the metadata table.
fn render_status_icon(is_downloaded: bool, color: gpui::Hsla) -> impl IntoElement + 'static {
    let (glyph, tooltip_text) = if is_downloaded {
        ("\u{2713}", t!("detail.status_on_device").to_string())
    }
    else {
        ("\u{2601}", t!("detail.status_in_cloud").to_string())
    };

    div().id("detail-status-icon")
         .text_sm()
         .text_color(color)
         .tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))
         .child(glyph)
}

/// Renders a metadata value, falling back to an em dash when the underlying
/// data is empty (e.g. the API did not report a game system/line for this
/// item).
fn value_or_dash(value: &str) -> String {
    if value.trim().is_empty() {
        "\u{2014}".to_string()
    }
    else {
        value.to_string()
    }
}

fn render_metadata_table(item: &LibraryItem, colors: &ColorTokens)
                         -> impl IntoElement + 'static + use<> {
    let category_label = div().flex()
                              .items_center()
                              .gap(px(4.0))
                              .child(Icon::new(IconName::Folder).text_color(colors.text_secondary))
                              .child(t!("detail.field_category").to_string())
                              .into_any_element();

    let mut list = DescriptionList::vertical()
        .columns(2)
        .bordered(false)
        .child(
            DescriptionItem::new(t!("detail.field_system").to_string())
                .value(value_or_dash(&item.line)),
        )
        .child(
            DescriptionItem::new(t!("detail.field_released").to_string())
                .value(item.year.to_string()),
        )
        .child(
            DescriptionItem::new(t!("detail.field_format").to_string())
                .value(item.format.to_string()),
        )
        .child(
            DescriptionItem::new(t!("detail.field_file_size").to_string())
                .value(format!("{:.0} MB", item.size_mb)),
        )
        .child(DescriptionItem::new(category_label).value(item.kind.to_string()).span(2));

    // The DriveThruRPG order-product API does not always report a page count; omit
    // the row entirely rather than showing a misleading "0".
    if item.pages > 0 {
        list = list.child(
            DescriptionItem::new(t!("detail.field_pages").to_string())
                .value(item.pages.to_string())
                .span(2),
        );
    }

    if let Some(ts) = item.date_added {
        let relative = format_relative(ts);
        let absolute = format_absolute(ts);
        let id = SharedString::from(format!("detail-added-{}", item.id));
        let value =
            div().id(id)
                 .child(relative)
                 .tooltip(move |window, cx| Tooltip::new(absolute.clone()).build(window, cx))
                 .into_any_element();
        list = list.child(DescriptionItem::new(t!("detail.field_added").to_string()).value(value)
                                                                                    .span(2));
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_or_dash_passes_through_non_empty_value() {
        assert_eq!(value_or_dash("Pathfinder"), "Pathfinder");
    }

    #[test]
    fn value_or_dash_falls_back_to_em_dash_on_empty_string() {
        assert_eq!(value_or_dash(""), "\u{2014}");
    }

    #[test]
    fn value_or_dash_falls_back_to_em_dash_on_whitespace_only() {
        assert_eq!(value_or_dash("   "), "\u{2014}");
    }
}
