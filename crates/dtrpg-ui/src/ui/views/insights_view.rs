//! The Insights page: the library's aggregates as cards and charts, in place of
//! the catalog.

use gpui::{
    Entity, InteractiveElement as _, IntoElement, ParentElement, StatefulInteractiveElement as _,
    Styled, div, px,
};
use gpui_kit::component::Disableable as _;
use gpui_kit::component::button::Button;
use gpui_kit::component::{Icon, IconName};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::theme::ColorTokens;
use crate::ui::views::insights_cache::cache_card;
use crate::ui::views::insights_cards::{
    analyzed_label, availability_card, disk_card, glance_card, roots_card, time_card, top_card,
    type_card,
};

/// The widest the page's content gets, in pixels.
const CONTENT_MAX_WIDTH: f32 = 1100.0;

/// The gap between cards, in pixels.
const CARD_GAP: f32 = 16.0;

/// The narrowest a chart card is before it moves to the next row, in pixels.
const CARD_MIN_WIDTH: f32 = 300.0;

/// The widest a chart card grows, in pixels: a third of the content width, so
/// three fit a row at full width and a lone card on the last row stays the
/// same size as the others.
const CARD_MAX_WIDTH: f32 = (CONTENT_MAX_WIDTH - 2.0 * CARD_GAP) / 3.0;

/// Renders the Insights page for `library`'s current aggregates.
pub fn render_insights(library: &Entity<LibraryController>, colors: &ColorTokens, cx: &gpui::App)
                       -> impl IntoElement + use<> {
    let (insights, computing, live, cache_counts) = {
        let controller = library.read(cx);
        (controller.insights(),
         controller.is_insights_computing(),
         controller.live_availability(),
         controller.cache_counts())
    };
    let recompute = library.clone();
    let export = library.clone();
    let header = div().flex()
                      .items_center()
                      .justify_between()
                      .px(px(24.0))
                      .py(px(12.0))
                      .border_b_1()
                      .border_color(colors.border)
                      .child(div().flex()
                                  .items_baseline()
                                  .gap(px(8.0))
                                  .child(div().text_lg()
                                              .font_weight(gpui::FontWeight::SEMIBOLD)
                                              .text_color(colors.text_primary)
                                              .child(t!("insights.title").to_string()))
                                  .child(div().text_sm()
                                              .text_color(colors.text_secondary)
                                              .child(insights.as_ref()
                                                             .map(|insights| {
                                                                 analyzed_label(insights.titles)
                                                             })
                                                             .unwrap_or_default())))
                      .child(div().flex()
                                  .gap(px(8.0))
                                  .child(Button::new("insights-recompute")
                                             .outline()
                                             .icon(Icon::new(IconName::Redo))
                                             .loading(computing)
                                             .label(t!("insights.recompute").to_string())
                                             .on_click(move |_, _, cx| {
                                                 recompute.update(cx, |ctrl, cx| {
                                                              ctrl.recompute_insights(cx);
                                                          });
                                             }))
                                  .child(Button::new("insights-export")
                                             .outline()
                                             .disabled(insights.is_none())
                                             .label(t!("insights.export_csv").to_string())
                                             .on_click(move |_, _, cx| {
                                                 export.update(cx, |ctrl, cx| {
                                                           ctrl.export_insights_csv(cx);
                                                       });
                                             })));

    let body = match insights {
        Some(insights) if insights.titles > 0 => {
            // The content is kept to a readable width and centered, so the
            // charts do not stretch across a very wide window.
            div().flex()
                 .justify_center()
                 .p(px(24.0))
                 .child(div().flex()
                             .flex_col()
                             .gap(px(16.0))
                             .w_full()
                             .max_w(px(CONTENT_MAX_WIDTH))
                             .child(glance_card(&insights, colors))
                             .child(div().flex()
                                         .flex_wrap()
                                         .gap(px(CARD_GAP))
                                         .child(tile(top_card("insights-publishers",
                                                              "insights.publisher_title",
                                                              None,
                                                              &insights.by_publisher,
                                                              colors)))
                                         .child(tile(top_card("insights-systems",
                                                              "insights.system_title",
                                                              Some(t!("insights.system_note").to_string()),
                                                              &insights.by_system,
                                                              colors)))
                                         .child(tile(type_card(&insights, colors)))
                                         .child(tile(availability_card(&insights, live, colors))))
                             .child(disk_card(&insights, colors))
                             .child(time_card(&insights.added_by_month, colors))
                             .child(roots_card(&insights.roots, colors))
                             .child(cache_card(cache_counts, colors)))
                 .into_any_element()
        }
        Some(_) => message(t!("insights.empty").to_string(), colors),
        None => message(t!("insights.computing").to_string(), colors),
    };

    div().flex_1()
         .min_h_0()
         .flex()
         .flex_col()
         .child(header)
         .child(div().id("insights-scroll")
                     .flex_1()
                     .min_h_0()
                     .overflow_y_scroll()
                     .child(body))
}

/// A line of text in the middle of the page, for when there is nothing to
/// chart.
fn message(text: String, colors: &ColorTokens) -> gpui::AnyElement {
    div().flex()
         .items_center()
         .justify_center()
         .h(px(240.0))
         .text_sm()
         .text_color(colors.text_secondary)
         .child(text)
         .into_any_element()
}

/// Wraps a chart card so that the cards share a row, wrap to the next when the
/// window is narrow, and stop growing at [`CARD_MAX_WIDTH`].
fn tile(card: impl IntoElement) -> gpui::Div {
    div().flex()
         .flex_col()
         .flex_1()
         .min_w(px(CARD_MIN_WIDTH))
         .max_w(px(CARD_MAX_WIDTH))
         .child(card)
}
