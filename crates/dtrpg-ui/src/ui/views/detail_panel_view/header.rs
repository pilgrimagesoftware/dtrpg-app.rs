use dtrpg_core::data::constants::INSPECTOR_THUMBNAIL_WIDTH;

use super::*;

/// Renders the inspector's header: the cover thumbnail beside the title,
/// publisher and "System · Type" line. In the docked pane, `pane_controls`
/// adds the buttons that pop the item out into its own window and close the
/// pane.
pub(super) fn render_inspector_header(item: &LibraryItem, cover_image: Option<Arc<Image>>,
                                      pane_controls: Option<&Entity<InspectorController>>,
                                      controller: Entity<LibraryController>,
                                      colors: &ColorTokens, cx: &App)
                                      -> AnyElement {
    let view_model = InspectorViewModel::new(item);
    let snapshot = view_model.snapshot();
    let is_checking = controller.read(cx).is_checking(&item.id);
    let is_verifying = controller.read(cx).is_verifying_downloads(&item.id);
    let is_downloaded = item.status == ItemStatus::Downloaded;

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

    let title_block =
        div().flex_1()
             .min_w_0()
             .flex()
             .flex_col()
             .gap(px(2.0))
             .child(div().flex()
                         .items_center()
                         .gap(px(6.0))
                         .child(selectable_text("inspector-title", snapshot.title.to_string())
                                    .text_lg()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(colors.text_primary))
                         .child(render_status_icon(is_downloaded,
                                                   item.is_offloaded(),
                                                   is_verifying,
                                                   colors.text_secondary))
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
         .children(pane_controls.map(|inspector| render_pane_controls(&item.id, inspector)))
         .into_any_element()
}

/// The two buttons at the top right of the docked pane: pop the item out into
/// its own window, and close the pane.
fn render_pane_controls(item_id: &Arc<str>, inspector: &Entity<InspectorController>) -> AnyElement {
    let pop_out_id = Arc::clone(item_id);
    let pop_out_inspector = inspector.clone();
    let close_inspector = inspector.clone();
    div().flex_none()
         .flex()
         .gap(px(2.0))
         .child(Button::new("inspector-pop-out").ghost()
                                                .compact()
                                                .icon(IconName::ExternalLink)
                                                .tooltip(t!("detail.pop_out_tooltip"))
                                                .on_click(move |_, _, cx| {
                                                    pop_out_inspector.update(cx, |inspector, cx| {
                                                        inspector.request_pop_out(Arc::clone(&pop_out_id), cx)
                                                    });
                                                }))
         .child(Button::new("inspector-close").ghost()
                                              .compact()
                                              .icon(IconName::Close)
                                              .tooltip(t!("detail.close_inspector_tooltip"))
                                              .on_click(move |_, _, cx| {
                                                  close_inspector.update(cx, |inspector, cx| {
                                                                     inspector.request_close(cx)
                                                                 });
                                              }))
         .into_any_element()
}
