use std::sync::Arc;

use dtrpg_core::data::library::LibraryItem;
use dtrpg_core::util::matching::{collection_member_id, member_ids_contain};
use gpui::{AnyElement, App, Entity, IntoElement, ParentElement, SharedString, Styled, div, px};
use gpui_kit::component::IconName;
use gpui_kit::component::button::{Button, ButtonVariants};
use rust_i18n::t;

use super::metadata_table::*;
use crate::controllers::library::LibraryController;
use crate::data::theme::{ColorTokens, LibriTheme};
use crate::ui::library::cover::cover_style;
use crate::ui::views::dialog::manage_collections::open_manage_collections_dialog;
use crate::ui::widgets::selectable_text;

/// Renders the Other details tab: the ids and other fields not shown in the
/// Details table (stable id, numeric id, order product id, product id,
/// added-order value, and the generative cover color), laid out like the
/// Details table.
pub(super) fn render_other_details_tab(item: &LibraryItem, colors: &ColorTokens, cx: &App)
                                       -> AnyElement {
    let label_font_family = cx.global::<LibriTheme>().fonts.label_font.clone();
    let value_font_family = cx.global::<LibriTheme>().fonts.value_font.clone();
    let copyable = |id: &'static str, value: String| {
        copyable_value(SharedString::from(id), value, &value_font_family)
    };
    let swatch = div().flex()
                      .items_center()
                      .gap(px(6.0))
                      .child(div().size(px(12.0))
                                  .rounded_full()
                                  .bg(cover_style(item).background))
                      .child(copyable("other-details-cover-color", item.color.to_string()))
                      .into_any_element();
    let rows = vec![(t!("detail.field_stable_id").to_string(),
                     copyable("other-details-stable-id", item.id.to_string())),
                    (t!("detail.field_numeric_id").to_string(),
                     copyable("other-details-numeric-id", item.numeric_id.to_string())),
                    (t!("detail.field_order_product_id").to_string(),
                     copyable("other-details-order-product-id",
                              item.order_product_id.to_string())),
                    (t!("detail.field_product_id").to_string(),
                     copyable("other-details-product-id", item.product_id.to_string())),
                    (t!("detail.field_added_order").to_string(),
                     styled_value(item.added_order.to_string(), &value_font_family)),
                    (t!("detail.field_cover_color").to_string(), swatch)];
    div().p(px(16.0))
         .child(render_detail_rows(rows, colors, &label_font_family))
         .into_any_element()
}

/// Renders a summary of the collections this entry currently belongs to, with a
/// "Manage…" button opening the Manage Collections dialog scoped to this entry.
///
/// See `detail-view-collection-membership`.
pub(super) fn render_collections_section(item: &LibraryItem, entity: Entity<LibraryController>,
                                         colors: &ColorTokens, cx: &App)
                                         -> impl IntoElement + 'static {
    let member_id = collection_member_id(item);
    let product_id = item.product_id;
    let member_names: Vec<Arc<str>> =
        entity.read(cx)
              .collections()
              .iter()
              .filter(|c| member_ids_contain(&c.member_ids, member_id, product_id))
              .map(|c| Arc::clone(&c.name))
              .collect();

    let summary: AnyElement = if member_names.is_empty() {
        div().text_sm()
             .text_color(colors.text_tertiary)
             .child(t!("detail.collections_empty").to_string())
             .into_any_element()
    }
    else {
        selectable_text("detail-tab-collections-summary",
                        member_names.iter()
                                    .map(|n| n.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")).text_sm()
                                                .text_color(colors.text_secondary)
                                                .into_any_element()
    };

    let item_title = Arc::clone(&item.title);

    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(colors.text_primary)
                .child(t!("detail.collections_heading").to_string()),
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .child(summary)
                .child(
                    Button::new("detail-tab-manage-collections")
                        .ghost()
                        .outline()
                        .icon(IconName::Settings)
                        .tooltip(t!("detail.collections_manage_button"))
                        .on_click(move |_, window, cx| {
                            open_manage_collections_dialog(
                                window,
                                cx,
                                entity.clone(),
                                Arc::clone(&item_title),
                                member_id,
                                product_id,
                            );
                        }),
                ),
        )
}
