//! Popped-out inspector windows: opening one from an item's tab, and
//! reopening the ones left open at the last quit.

use dtrpg_core::data::ui_state::WindowBoundsPref;

use super::*;

impl LibraryRootView {
    /// Moves the inspector for `item_id` out of its tab and into its own
    /// window. If the item already has a window, that one comes to front.
    pub(crate) fn pop_out_inspector(&mut self, item_id: &Arc<str>, cx: &mut Context<Self>) {
        self.show_inspector_window(item_id, None, cx);
        self.tabs
            .update(cx, |tabs, cx| tabs.close_detail_tab(item_id, cx));
    }

    /// Reopens remembered inspector windows whose items have reached the
    /// catalog. Once the catalog has finished loading, any remembered window
    /// whose item is still missing is forgotten, since the item has left the
    /// library.
    pub(crate) fn restore_inspector_windows(&mut self, cx: &mut Context<Self>) {
        if self.pending_inspector_restores.is_empty() {
            return;
        }
        let controller = self.controller.read(cx);
        let catalog_loading = controller.is_catalog_loading();
        let (ready, waiting): (Vec<_>, Vec<_>) =
            std::mem::take(&mut self.pending_inspector_restores)
                .into_iter()
                .partition(|saved| controller.item_by_id(&saved.item_id).is_some());

        if catalog_loading {
            self.pending_inspector_restores = waiting;
        }
        else {
            for missing in waiting {
                UiState::load().remove_inspector_window(&missing.item_id);
            }
        }
        for saved in ready {
            self.show_inspector_window(&Arc::from(saved.item_id), saved.bounds, cx);
        }
    }

    /// Opens a window for the inspector of `item_id`, or brings its existing
    /// window to front.
    fn show_inspector_window(&mut self, item_id: &Arc<str>,
                             saved_bounds: Option<WindowBoundsPref>, cx: &mut Context<Self>) {
        let already_open = self.inspector_windows
                               .get(item_id)
                               .map(|handle| {
                                   handle.update(cx, |_, window, _| window.activate_window())
                                         .is_ok()
                               })
                               .unwrap_or(false);
        if already_open {
            return;
        }
        let Some(item) = self.controller.read(cx).item_by_id(item_id)
        else {
            return;
        };
        let handle = open_inspector_window(Arc::clone(item_id),
                                           &item.title,
                                           saved_bounds,
                                           self.controller.clone(),
                                           self.settings.clone(),
                                           cx);
        self.inspector_windows.insert(Arc::clone(item_id), handle);
    }
}
