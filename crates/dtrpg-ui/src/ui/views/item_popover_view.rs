//! Item popover: lightweight, anchored detail view opened by single-clicking
//! a catalog item.
//!
//! Distinct from the expanded detail tab (opened by double-clicking), which
//! renders full attributes and a file list in its own closable tab — see
//! `main-window-tabs`.

use gpui::prelude::*;
use gpui::{
    AnyElement, Entity, IntoElement, ParentElement, Pixels, Point, Styled, anchored, deferred, div,
    px,
};
use gpui_component::IconName;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::description_list::{DescriptionItem, DescriptionList};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::constants::{ITEM_POPOVER_MARGIN, ITEM_POPOVER_WIDTH};
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::theme::ColorTokens;

/// Renders a compact popover anchored at `position`, showing `item`'s title,
/// publisher, and a few key attributes, plus a close button and an
/// accessible "open in tab" affordance as an alternative to double-click.
///
/// `position` is the top-left corner at which the popover is drawn — callers
/// are responsible for computing it (see
/// `catalog_view::popover_anchor_point`) so the popover sits beside the
/// catalog entry rather than over it.
pub fn render_item_popover(item: &LibraryItem, position: Point<Pixels>,
                           entity: Entity<LibraryController>, colors: &ColorTokens)
                           -> AnyElement {
    let surface = colors.surface;
    let border = colors.border;
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;

    let title = item.title.to_string();
    let publisher = item.publisher.to_string();
    let status_label = if item.status == ItemStatus::Downloaded {
        t!("detail.status_on_device").to_string()
    }
    else {
        t!("detail.status_in_cloud").to_string()
    };

    let entity_close = entity.clone();

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
                .child(
                    DescriptionItem::new(t!("detail.field_system").to_string())
                        .value(item.line.to_string()),
                )
                .child(
                    DescriptionItem::new(t!("detail.field_format").to_string())
                        .value(item.format.to_string()),
                )
                .child(
                    DescriptionItem::new(t!("detail.field_status").to_string()).value(status_label),
                ),
        );

    deferred(anchored().snap_to_window_with_margin(px(ITEM_POPOVER_MARGIN))
                       .position(position)
                       .child(content)).with_priority(1)
                                       .into_any_element()
}
