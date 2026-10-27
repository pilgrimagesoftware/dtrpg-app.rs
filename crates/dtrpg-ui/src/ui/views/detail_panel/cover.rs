use super::*;
use crate::ui::app::open_cover_viewer_window;

/// Width of the cover generated for an item that has no image, on the Cover
/// tab, in pixels. The height follows the book proportions.
const GENERATED_COVER_WIDTH: f32 = 280.0;

/// Wraps an item's cover so that a double-click opens it at full size in a
/// window of its own.
pub(super) fn render_clickable_cover(item: &LibraryItem, cover: AnyElement, width: f32,
                                     height: f32, controller: Entity<LibraryController>)
                                     -> AnyElement {
    let item = item.clone();
    div().id("inspector-cover")
         .flex_none()
         .w(px(width))
         .h(px(height))
         .rounded(px(4.0))
         .overflow_hidden()
         .cursor_pointer()
         .tooltip(|window, cx| {
             Tooltip::new(t!("detail.cover_open_tooltip").to_string()).build(window, cx)
         })
         .on_click(move |event, _, cx| {
             if event.click_count() >= 2 {
                 open_cover_viewer(&item, &controller, cx);
             }
         })
         .child(cover)
         .into_any_element()
}

/// Opens the cover viewer for `item`, and asks for the full-size image if it
/// has not been fetched yet.
fn open_cover_viewer(item: &LibraryItem, controller: &Entity<LibraryController>, cx: &mut App) {
    controller.update(cx, |ctrl, cx| ctrl.ensure_detail_cover(&item.id, cx));
    open_cover_viewer_window(item.clone(), controller.clone(), cx);
}

/// Renders the Cover tab: the cover as large as the inspector is wide.
/// Double-clicking it opens the viewer window. An item with no image shows its
/// generated cover.
pub(super) fn render_cover_tab(item: &LibraryItem, controller: Entity<LibraryController>,
                               colors: &ColorTokens, cx: &App)
                               -> AnyElement {
    let covers = cx.global::<CoverCache>();
    let image = covers.get(&detail_cache_key(&item.id))
                      .or_else(|| covers.get(&item.id));
    let cover: AnyElement = match image {
        Some(image) => img(image).w_full()
                                 .object_fit(ObjectFit::Contain)
                                 .into_any_element(),
        None => render_generative_cover(item,
                                        GENERATED_COVER_WIDTH,
                                        GENERATED_COVER_WIDTH * 10.0 / 7.0,
                                        true).into_any_element(),
    };
    let item = item.clone();
    div().id("inspector-cover-tab")
         .p(px(16.0))
         .flex()
         .justify_center()
         .bg(colors.surface)
         .cursor_pointer()
         .tooltip(|window, cx| {
             Tooltip::new(t!("detail.cover_open_tooltip").to_string()).build(window, cx)
         })
         .on_click(move |event, _, cx| {
             if event.click_count() >= 2 {
                 open_cover_viewer(&item, &controller, cx);
             }
         })
         .child(cover)
         .into_any_element()
}
