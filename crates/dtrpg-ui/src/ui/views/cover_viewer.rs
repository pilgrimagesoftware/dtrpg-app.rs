//! Root view for the cover viewer: one item's cover at the largest size the
//! window allows. Escape closes the window.

use dtrpg_core::data::library::LibraryItem;
use gpui::{
    Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement, ObjectFit,
    ParentElement, Render, Styled, StyledImage, Window, div, img,
};

use crate::controllers::library::LibraryController;
use crate::data::theme::LibriTheme;
use crate::ui::library::cover::{CoverCache, detail_cache_key, render_generative_cover};

/// Width of the generated cover shown when the item has no image, in pixels.
const GENERATED_COVER_WIDTH: f32 = 360.0;

/// Root view for a cover viewer window.
pub struct CoverViewerView {
    item:         LibraryItem,
    focus_handle: FocusHandle,
}

impl CoverViewerView {
    /// Shows `item`'s cover, redrawn as the library fetches images. Focuses the
    /// window so Escape closes it without a click first.
    pub fn new(item: LibraryItem, controller: Entity<LibraryController>, window: &mut Window,
               cx: &mut Context<Self>)
               -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        cx.observe(&controller, |_this, _controller, cx| cx.notify())
          .detach();
        Self { item, focus_handle }
    }
}

impl Focusable for CoverViewerView {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CoverViewerView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<LibriTheme>().clone();
        let colors = &theme.colors;
        window.set_rem_size(theme.fonts.ui_text_size);

        let covers = cx.global::<CoverCache>();
        let image = covers.get(&detail_cache_key(&self.item.id))
                          .or_else(|| covers.get(&self.item.id));
        let cover = match image {
            Some(image) => img(image).size_full()
                                     .object_fit(ObjectFit::Contain)
                                     .into_any_element(),
            None => render_generative_cover(&self.item,
                                            GENERATED_COVER_WIDTH,
                                            GENERATED_COVER_WIDTH * 10.0 / 7.0,
                                            true).into_any_element(),
        };

        div().id("cover-viewer")
             .track_focus(&self.focus_handle)
             .on_key_down(|event, window, _cx| {
                 if event.keystroke.key == "escape" {
                     window.remove_window();
                 }
             })
             .size_full()
             .flex()
             .items_center()
             .justify_center()
             .p(gpui::px(16.0))
             .bg(colors.surface)
             .child(cover)
    }
}
