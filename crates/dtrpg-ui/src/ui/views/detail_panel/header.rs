use std::sync::Arc;

use dtrpg_core::data::constants::INSPECTOR_THUMBNAIL_WIDTH;
use dtrpg_core::data::library::LibraryItem;
use gpui::{
    AnyElement, App, Entity, Image, IntoElement, ObjectFit, ParentElement, Styled, StyledImage,
    div, img, px,
};
use gpui_kit::component::IconName;
use gpui_kit::component::button::{Button, ButtonVariants};
use rust_i18n::t;

use super::cover::*;
use crate::controllers::inspector::{InspectorController, InspectorPlacement};
use crate::controllers::library::LibraryController;
use crate::data::theme::ColorTokens;
use crate::ui::library::cover::render_generative_cover;
use crate::ui::views::catalog::view::render_checking_indicator;
use crate::ui::widgets::selectable_text;
use crate::view_models::inspector::InspectorViewModel;

/// Renders the inspector's header: the cover thumbnail beside the title,
/// publisher and "System · Type" line, and the buttons at the top right. The
/// docked pane offers popping the item out into its own window and closing the
/// pane; a popped-out window offers putting the item back in the pane.
pub(super) fn render_inspector_header(item: &LibraryItem, cover_image: Option<Arc<Image>>,
                                      inspector: &Entity<InspectorController>,
                                      controller: Entity<LibraryController>,
                                      colors: &ColorTokens, cx: &App)
                                      -> AnyElement {
    let view_model = InspectorViewModel::new(item);
    let snapshot = view_model.snapshot();
    let is_checking = controller.read(cx).is_checking(&item.id);

    let thumbnail_height = INSPECTOR_THUMBNAIL_WIDTH * 10.0 / 7.0;
    let thumbnail: AnyElement =
        match cover_image {
            Some(image) => img(image).w(px(INSPECTOR_THUMBNAIL_WIDTH))
                                     .h(px(thumbnail_height))
                                     .object_fit(ObjectFit::Cover)
                                     .into_any_element(),
            None => render_generative_cover(item,
                                            INSPECTOR_THUMBNAIL_WIDTH,
                                            thumbnail_height,
                                            true).into_any_element(),
        };

    // The title takes the room the checking indicator leaves and wraps within
    // it, so a long title never runs under the buttons.
    let title_block =
        div().flex_1()
             .min_w_0()
             .overflow_hidden()
             .flex()
             .flex_col()
             .gap(px(2.0))
             .child(div().flex()
                         .items_start()
                         .gap(px(6.0))
                         .child(selectable_text("inspector-title", snapshot.title.to_string())
                                    .flex_1()
                                    .min_w_0()
                                    .text_lg()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(colors.text_primary))
                         .children(render_checking_indicator(is_checking)))
             .child(selectable_text("inspector-publisher", snapshot.publisher.to_string())
                        .text_sm()
                        .text_color(colors.text_secondary))
             .child(selectable_text("inspector-system-type", snapshot.system_and_type())
                        .text_xs()
                        .text_color(colors.text_tertiary));

    div().flex()
         .items_start()
         .gap(px(12.0))
         .p(px(16.0))
         .child(render_clickable_cover(item,
                                       thumbnail,
                                       INSPECTOR_THUMBNAIL_WIDTH,
                                       thumbnail_height,
                                       controller.clone()))
         .child(title_block)
         .child(render_header_buttons(&item.id, inspector, cx))
         .into_any_element()
}

/// The buttons at the top right of the header, which depend on where the
/// inspector is shown.
fn render_header_buttons(item_id: &Arc<str>, inspector: &Entity<InspectorController>, cx: &App)
                         -> AnyElement {
    let buttons = div().flex_none().flex().gap(px(2.0));
    match inspector.read(cx).placement() {
        InspectorPlacement::Docked => {
            let pop_out_id = Arc::clone(item_id);
            let pop_out_inspector = inspector.clone();
            let close_inspector = inspector.clone();
            buttons.child(Button::new("inspector-pop-out")
                              .ghost()
                              .compact()
                              .icon(IconName::ExternalLink)
                              .tooltip(t!("detail.pop_out_tooltip"))
                              .on_click(move |_, _, cx| {
                                  pop_out_inspector.update(cx, |inspector, cx| {
                                      inspector.request_pop_out(Arc::clone(&pop_out_id), cx)
                                  });
                              }))
                   .child(Button::new("inspector-close")
                              .ghost()
                              .compact()
                              .icon(IconName::Close)
                              .tooltip(t!("detail.close_inspector_tooltip"))
                              .on_click(move |_, _, cx| {
                                  close_inspector.update(cx, |inspector, cx| {
                                                     inspector.request_close(cx)
                                                 });
                              }))
        }
        InspectorPlacement::Window => {
            let pop_back_inspector = inspector.clone();
            buttons.child(Button::new("inspector-pop-back")
                              .ghost()
                              .compact()
                              .icon(IconName::PanelRightOpen)
                              .tooltip(t!("detail.pop_back_tooltip"))
                              .on_click(move |_, _, cx| {
                                  pop_back_inspector.update(cx, |inspector, cx| {
                                                        inspector.request_pop_back(cx)
                                                    });
                              }))
        }
    }.into_any_element()
}
