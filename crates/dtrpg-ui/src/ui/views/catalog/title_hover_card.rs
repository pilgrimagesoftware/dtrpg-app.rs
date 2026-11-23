//! Hover card shown over a truncated catalog title.

use std::time::Duration;

use dtrpg_core::data::enums::ItemStatus;
use gpui::prelude::*;
use gpui::{ElementId, IntoElement, Styled, div, px};
use gpui_kit::component::hover_card::HoverCard;
use rust_i18n::t;

use crate::data::theme::ColorTokens;

/// Delay before the card opens; matches the plain tooltip it replaces.
const OPEN_DELAY: Duration = Duration::from_millis(300);
/// Delay before the card closes after the pointer leaves.
const CLOSE_DELAY: Duration = Duration::from_millis(100);
/// Maximum card width, so long titles wrap instead of stretching the card.
const CARD_MAX_WIDTH: f32 = 320.0;

/// What the hover card shows for a catalog item.
pub(super) struct TitleCardInfo {
    pub title:     String,
    pub publisher: String,
    pub status:    ItemStatus,
    pub offloaded: bool,
}

/// Wraps `title_el` in a [`HoverCard`] listing the item's full title,
/// publisher and status.
///
/// The status text is looked up inside the content closure, which only runs
/// while the card is open, so rows that are merely rendered pay nothing.
pub(super) fn title_hover_card(id: impl Into<ElementId>, title_el: impl IntoElement + 'static,
                               info: TitleCardInfo, colors: &ColorTokens)
                               -> impl IntoElement {
    let primary = colors.text_primary;
    let secondary = colors.text_secondary;
    HoverCard::new(id).open_delay(OPEN_DELAY)
                      .close_delay(CLOSE_DELAY)
                      .trigger(div().w_full().min_w_0().child(title_el))
                      .content(move |_, _, _| {
                          let status = match (info.status, info.offloaded) {
                              (ItemStatus::Downloaded, true) => t!("detail.item_status_offloaded"),
                              (ItemStatus::Downloaded, false) => t!("detail.status_on_device"),
                              (ItemStatus::Cloud, _) => t!("detail.status_in_cloud"),
                          };
                          div().max_w(px(CARD_MAX_WIDTH))
                               .flex()
                               .flex_col()
                               .gap(px(2.0))
                               .child(div().text_sm()
                                           .text_color(primary)
                                           .child(info.title.clone()))
                               .child(div().text_xs()
                                           .text_color(secondary)
                                           .child(info.publisher.clone()))
                               .child(div().text_xs()
                                           .text_color(secondary)
                                           .child(status.to_string()))
                      })
}
