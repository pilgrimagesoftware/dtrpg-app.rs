//! The inspector pane docked beside the catalog.

use super::*;
use crate::view_models::inspector::InspectorTab;

impl LibraryRootView {
    /// Shows or hides the inspector pane.
    pub(crate) fn toggle_inspector(&mut self, cx: &mut Context<Self>) {
        self.set_inspector_open(!self.inspector_open, cx);
    }

    /// Shows or hides the inspector pane, and remembers the choice.
    pub(super) fn set_inspector_open(&mut self, open: bool, cx: &mut Context<Self>) {
        if self.inspector_open == open {
            return;
        }
        self.inspector_open = open;
        UiState::load().save_inspector_open(open);
        self.refresh_menus(cx);
        cx.notify();
    }

    /// Shows `item_id` in the inspector pane on `tab`, opening the pane if it
    /// is closed.
    pub(super) fn show_in_inspector_pane(&mut self, item_id: &Arc<str>, tab: InspectorTab,
                                         cx: &mut Context<Self>) {
        self.controller.update(cx, |ctrl, cx| {
                           ctrl.select_item(Arc::clone(item_id), cx);
                           ctrl.set_inspector_tab(Arc::clone(item_id), tab, cx);
                       });
        self.set_inspector_open(true, cx);
    }
}

/// Renders the pane's content: the inspector for the selected item, or a
/// prompt to select one.
pub(super) fn render_inspector_pane(view: &LibraryRootView, window: &mut gpui::Window,
                                    cx: &mut Context<LibraryRootView>)
                                    -> AnyElement {
    let colors = cx.global::<LibriTheme>().colors.clone();
    let pane = div().id("inspector-pane")
                    .size_full()
                    .flex()
                    .flex_col()
                    .border_l_1()
                    .border_color(colors.border)
                    .bg(colors.surface);
    let Some(item) = view.controller.read(cx).selected_item()
    else {
        return pane.items_center()
                   .justify_center()
                   .p(px(24.0))
                   .text_sm()
                   .text_color(colors.text_secondary)
                   .child(t!("detail.inspector_empty").to_string())
                   .into_any_element();
    };
    pane.child(render_item_inspector(&item,
                                     &view.controller,
                                     &view.settings,
                                     &view.inspector,
                                     window,
                                     cx))
        .into_any_element()
}
