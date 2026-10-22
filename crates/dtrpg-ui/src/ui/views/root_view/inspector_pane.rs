//! The inspector pane docked beside the catalog. It is shown while an item is
//! selected, and closes when the selection is cleared.

use dtrpg_core::data::library::LibraryItem;

use super::*;
use crate::view_models::inspector::InspectorTab;

impl LibraryRootView {
    /// Shows `item_id` in the inspector pane on `tab`, by selecting it.
    pub(super) fn show_in_inspector_pane(&mut self, item_id: &Arc<str>, tab: InspectorTab,
                                         cx: &mut Context<Self>) {
        self.controller.update(cx, |ctrl, cx| {
                           ctrl.select_item(Arc::clone(item_id), cx);
                           ctrl.set_inspector_tab(Arc::clone(item_id), tab, cx);
                       });
    }

    /// Closes the inspector pane, by clearing the selection it shows.
    pub(super) fn close_inspector_pane(&mut self, cx: &mut Context<Self>) {
        self.controller
            .update(cx, |ctrl, cx| ctrl.clear_selection(cx));
    }
}

/// Renders the pane's content: the inspector for the selected `item`.
pub(super) fn render_inspector_pane(view: &LibraryRootView, item: &LibraryItem,
                                    window: &mut gpui::Window, cx: &mut Context<LibraryRootView>)
                                    -> AnyElement {
    let colors = cx.global::<LibriTheme>().colors.clone();
    div().id("inspector-pane")
         .size_full()
         .flex()
         .flex_col()
         .border_l_1()
         .border_color(colors.border)
         .bg(colors.surface)
         .child(render_item_inspector(item,
                                      &view.controller,
                                      &view.settings,
                                      &view.inspector,
                                      window,
                                      cx))
         .into_any_element()
}
