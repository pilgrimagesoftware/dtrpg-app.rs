//! Detail panel: slide-over showing full item metadata and actions.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, AppContext as _, Context, DragMoveEvent, Empty, Entity, Image, InteractiveElement,
    IntoElement, ObjectFit, ParentElement, Render, SharedString, StatefulInteractiveElement,
    Styled, StyledImage, Window, div, img, px,
};
use gpui_component::Disableable;
use gpui_component::IconName;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::description_list::{DescriptionItem, DescriptionList};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::tooltip::Tooltip;

use crate::controllers::library::LibraryController;
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::theme::ColorTokens;
use crate::ui::library::cover::render_generative_cover;
use crate::util::datetime::{format_absolute, format_relative};
use crate::util::reveal::reveal_in_file_manager;
use rust_i18n::t;

/// Drag-payload marker identifying an in-progress detail panel resize drag.
///
/// Carries no state — the new width is computed on each drag-move event from
/// the live cursor position and the panel's own bounds, so no drag-start
/// offset needs to be captured here.
struct DetailPanelResizeDrag;

impl Render for DetailPanelResizeDrag {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// Renders the detail panel overlay if `selected_item` is `Some`; otherwise an empty div.
///
/// `cover_image`, when `Some`, is rendered in place of the generative cover —
/// callers should look it up from `CoverCache` for the selected item.
pub fn render_detail_panel(
    selected_item: Option<&LibraryItem>,
    storage_root_path: PathBuf,
    entity: Entity<LibraryController>,
    colors: &ColorTokens,
    cover_image: Option<Arc<Image>>,
    width: f32,
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
    let entity_resize = entity.clone();
    let entity_refresh_thumbnail = entity.clone();
    let item_id = Arc::clone(&item.id);
    let reveal_item_id = Arc::clone(&item.id);
    let read_item_id = Arc::clone(&item.id);
    let is_downloaded = item.status == ItemStatus::Downloaded;

    div()
        .id("detail-panel")
        .occlude()
        .absolute()
        .right_0()
        .top_0()
        .bottom_0()
        .w(px(width))
        .bg(surface)
        .border_l_1()
        .border_color(border)
        .flex()
        .flex_col()
        .on_drag_move::<DetailPanelResizeDrag>(
            move |event: &DragMoveEvent<DetailPanelResizeDrag>, _window, cx| {
                let new_width = f32::from(event.bounds.right() - event.event.position.x);
                entity_resize.update(cx, |ctrl, cx| ctrl.set_detail_panel_width(new_width, cx));
            },
        )
        // Resize handle (left edge)
        .child(
            div()
                .id("detail-panel-resize-handle")
                .occlude()
                .absolute()
                .left_0()
                .top_0()
                .bottom_0()
                .w(px(6.0))
                .cursor_col_resize()
                .hover(|s| s.bg(border))
                .on_drag(DetailPanelResizeDrag, move |_value, _point, _window, cx| {
                    cx.new(|_| DetailPanelResizeDrag)
                }),
        )
        // Cover — capped at `DETAIL_PANEL_COVER_MAX_WIDTH` and re-centered
        // horizontally (staying top-aligned) as the panel is resized wider.
        .child({
            let cover_w = width.min(crate::data::constants::DETAIL_PANEL_COVER_MAX_WIDTH);
            let cover_h = cover_w * 10.0 / 7.0;
            let cover: AnyElement = if let Some(image) = cover_image {
                img(image)
                    .w(px(cover_w))
                    .h(px(cover_h))
                    .object_fit(ObjectFit::Cover)
                    .into_any_element()
            } else {
                render_generative_cover(&item, cover_w, cover_h, true).into_any_element()
            };
            let cover_url = item.cover_url.clone();
            let mut cover_box = div()
                .relative()
                .w(px(cover_w))
                .h(px(cover_h))
                .flex_none()
                .child(cover);
            if let Some(cover_url) = cover_url {
                cover_box = cover_box.child(
                    div()
                        .id("detail-refresh-thumbnail")
                        .absolute()
                        .top(px(8.0))
                        .left(px(8.0))
                        .size(px(24.0))
                        .rounded_full()
                        .bg(scrim)
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .text_sm()
                        .text_color(accent_on)
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
            div()
                .w_full()
                .flex()
                .flex_none()
                .justify_center()
                .child(cover_box)
        })
        // Scrollable body
        //
        // `overflow_y_scrollbar()` wraps this div in a `Scrollable` element whose outer
        // wrapper only inherits explicit width/height from the wrapped element's style,
        // not `flex_1`/`min_h_0`. Without an outer flex_1/min_h_0 wrapper the scroll
        // area sizes to its content instead of the panel's remaining height, so it never
        // scrolls. See `catalog_view.rs` for the same pattern.
        .child(
            div().flex_1().min_h_0().flex().flex_col().child(
                div()
                    .overflow_y_scrollbar()
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
                    // Description
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary)
                            .child(item.desc.to_string()),
                    )
                    // Actions — icon buttons with tooltips instead of full-width
                    // labeled buttons, so the row stays compact regardless of panel width.
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child({
                                let read_path =
                                    storage_root_path.join("items").join(read_item_id.as_ref());
                                Button::new("detail-read")
                                    .primary()
                                    .icon(IconName::BookOpen)
                                    .disabled(!is_downloaded)
                                    .when(!is_downloaded, |b| {
                                        b.tooltip(t!("detail.tooltip_download_first"))
                                    })
                                    .when(is_downloaded, |b| {
                                        b.tooltip(t!("detail.read_button"))
                                        .on_click(move |_, _, _| {
                                        use crate::util::item_opener::{ItemOpener, OpenError};

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
                                                    tracing::warn!("open: file not found: {path}");
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
                                    })
                                    })
                            })
                            .child(
                                Button::new("detail-download")
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
                                let item_path = storage_root_path
                                    .join("items")
                                    .join(reveal_item_id.as_ref());
                                row.child(
                                Button::new("detail-reveal")
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
                                            tracing::warn!("reveal_in_file_manager failed: {e}");
                                        }
                                    }),
                            )
                            }),
                    )
                    // Metadata table
                    .child(render_metadata_table(&item, colors)),
            ),
        )
        // Close button — rendered last so it paints on top of the cover image and
        // scroll body; GPUI stacks sibling children in child-list order regardless
        // of `absolute()` positioning, so an earlier position in the chain would
        // have the cover painted over it.
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

/// Renders a small status icon next to the item title: a checkmark when downloaded,
/// a cloud glyph otherwise. Replaces the old text-only "Status" row in the metadata
/// table.
fn render_status_icon(is_downloaded: bool, color: gpui::Hsla) -> impl IntoElement + 'static {
    let (glyph, tooltip_text) = if is_downloaded {
        ("\u{2713}", t!("detail.status_on_device").to_string())
    } else {
        ("\u{2601}", t!("detail.status_in_cloud").to_string())
    };

    div()
        .id("detail-status-icon")
        .text_sm()
        .text_color(color)
        .tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))
        .child(glyph)
}

fn render_metadata_table(
    item: &LibraryItem,
    _colors: &ColorTokens,
) -> impl IntoElement + 'static + use<> {
    let mut list = DescriptionList::vertical()
        .columns(1)
        .bordered(false)
        .child(
            DescriptionItem::new(t!("detail.field_system").to_string())
                .value(item.line.to_string()),
        )
        .child(
            DescriptionItem::new(t!("detail.field_category").to_string())
                .value(item.kind.to_string()),
        )
        .child(
            DescriptionItem::new(t!("detail.field_format").to_string())
                .value(item.format.to_string()),
        )
        .child(
            DescriptionItem::new(t!("detail.field_file_size").to_string())
                .value(format!("{:.0} MB", item.size_mb)),
        )
        .child(
            DescriptionItem::new(t!("detail.field_released").to_string())
                .value(item.year.to_string()),
        );

    // The DriveThruRPG order-product API does not always report a page count; omit the
    // row entirely rather than showing a misleading "0".
    if item.pages > 0 {
        list = list.child(
            DescriptionItem::new(t!("detail.field_pages").to_string())
                .value(item.pages.to_string()),
        );
    }

    if let Some(ts) = item.date_added {
        let relative = format_relative(ts);
        let absolute = format_absolute(ts);
        let id = SharedString::from(format!("detail-added-{}", item.id));
        let value = div()
            .id(id)
            .child(relative)
            .tooltip(move |window, cx| Tooltip::new(absolute.clone()).build(window, cx))
            .into_any_element();
        list = list.child(DescriptionItem::new(t!("detail.field_added").to_string()).value(value));
    }

    list
}
