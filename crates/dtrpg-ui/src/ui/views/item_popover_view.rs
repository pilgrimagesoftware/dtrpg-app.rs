//! Item popover: lightweight, anchored detail view opened by single-clicking
//! a catalog item.
//!
//! Distinct from the expanded detail tab (opened by double-clicking), which
//! renders full attributes and, for multi-item entries, a persistent item
//! list in its own closable tab — see `main-window-tabs` and
//! `catalog-entry-detail-view`. This popover never renders an item list,
//! regardless of item count.

use std::sync::Arc;

use gpui::prelude::*;
use gpui::{
    AnyElement, Entity, IntoElement, ParentElement, Pixels, Point, Styled, anchored, deferred, div,
    px,
};
use gpui_component::Disableable;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::description_list::{DescriptionItem, DescriptionList};
use gpui_component::{IconName, Sizable};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::controllers::tabs::TabsController;
use crate::data::constants::{ITEM_POPOVER_MARGIN, ITEM_POPOVER_WIDTH};
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::theme::ColorTokens;

/// Renders a compact popover anchored at `position`, showing `item`'s title,
/// publisher, and a few key attributes, plus a close button and action
/// buttons to toggle the download status and open the item's detail tab.
///
/// `position` is the top-left corner at which the popover is drawn — callers
/// are responsible for computing it (see
/// `catalog_view::popover_anchor_point`) so the popover sits beside the
/// catalog entry rather than over it.
pub fn render_item_popover(item: &LibraryItem, position: Point<Pixels>,
                           entity: Entity<LibraryController>, tabs: Entity<TabsController>,
                           colors: &ColorTokens)
                           -> AnyElement {
    let surface = colors.surface;
    let border = colors.border;
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;

    let title = item.title.to_string();
    let publisher = item.publisher.to_string();
    // let status_label = if item.status == ItemStatus::Downloaded {
    //     t!("detail.status_on_device").to_string()
    // }
    // else {
    //     t!("detail.status_in_cloud").to_string()
    // };

    let entity_close = entity.clone();
    let entity_download = entity.clone();
    let entity_open_detail = entity.clone();
    let is_downloaded = item.status == ItemStatus::Downloaded;
    let item_id = Arc::clone(&item.id);
    let item_id_for_detail = Arc::clone(&item.id);
    let item_title = item.title.to_string();

    let content = div()
        .id("item-popover")
        .occlude()
        .w(px(ITEM_POPOVER_WIDTH))
        .bg(surface)
        .border_1()
        .border_color(border)
        .rounded(px(8.0))
        .shadow_lg()
        .p(px(12.0))
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .gap(px(8.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(text_primary)
                                .truncate()
                                .child(title),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_secondary)
                                .truncate()
                                .child(publisher),
                        ),
                )
                .child(
                    Button::new("item-popover-close")
                        .ghost()
                        .compact()
                        .icon(IconName::Close)
                        .on_click(move |_, _, cx| {
                            entity_close.update(cx, |ctrl, cx| ctrl.clear_selection(cx));
                        }),
                ),
        )
        .child(
            DescriptionList::vertical()
                .columns(1)
                .bordered(false)
                .small()
                // TODO: add updated date HUMAN READABLE
                .when(!item.line.is_empty(), |list|
                    list.child(
                        DescriptionItem::new(t!("detail.field_system").to_string())
                            .value(item.line.to_string()),
                    ))
                .child(
                    DescriptionItem::new(t!("detail.field_format").to_string())
                        .value(item.format.to_string()),
                )
                .when(item.files.len() > 1, |list| {
                    list.child(
                        DescriptionItem::new(t!("detail.field_file_count").to_string())
                            .value(item.files.len().to_string()),
                    )
                })
                // .child(
                //     DescriptionItem::new(t!("detail.field_status").to_string()).value(status_label),
                // ),
        )
        .child(
            div()
                .flex()
                .gap(px(8.0))
                .child(
                    Button::new("item-popover-status")
                        .primary()
                        .small()
                        .disabled(true)
                        .icon(if is_downloaded {
                            IconName::ArrowDown // TODO: FileCheckCorner
                        }
                        else {
                            IconName::Globe // TODO: CloudCog
                        })
                )
                .child(
                    Button::new("item-popover-download")
                        .ghost()
                        .outline()
                        .compact()
                        .small()
                        .icon(download_button_icon(is_downloaded))
                        .tooltip(if is_downloaded {
                            t!("catalog.action_remove_download")
                        }
                        else {
                            t!("catalog.action_download")
                        })
                        .on_click(move |_, _, cx| {
                            let id = Arc::clone(&item_id);
                            entity_download.update(cx, |ctrl, cx| ctrl.toggle_download(&id, cx));
                        }),
                )
                .child(
                    Button::new("item-popover-open-detail")
                        .ghost()
                        .outline()
                        .compact()
                        .small()
                        .icon(IconName::ExternalLink)
                        .tooltip(t!("detail.open_in_detail_button"))
                        .on_click(move |_, _, cx| {
                            let id = Arc::clone(&item_id_for_detail);
                            let title = item_title.clone();
                            tabs.update(cx, |ctrl, cx| {
                                    ctrl.open_detail_tab(Arc::clone(&id), title, cx);
                                });
                            // Reopening a detail tab must show no pre-selected
                            // item (selection is ephemeral, see
                            // `catalog-entry-detail-view`).
                            entity_open_detail
                                .update(cx, |ctrl, cx| ctrl.clear_item_selection(&id, cx));
                        }),
                ),
        );

    deferred(anchored().snap_to_window_with_margin(px(ITEM_POPOVER_MARGIN))
                       .position(position)
                       .child(content)).with_priority(1)
                                       .into_any_element()
}

/// Returns the icon for the popover's download action button: a checkmark
/// once the item is downloaded, otherwise a download arrow.
fn download_button_icon(is_downloaded: bool) -> IconName {
    if is_downloaded {
        IconName::CircleCheck
    }
    else {
        IconName::ArrowDown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_button_icon_shows_check_when_downloaded() {
        assert!(matches!(download_button_icon(true), IconName::CircleCheck));
    }

    #[test]
    fn download_button_icon_shows_arrow_when_not_downloaded() {
        assert!(matches!(download_button_icon(false), IconName::ArrowDown));
    }
}
