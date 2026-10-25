//! Item and file selection state.

use super::super::*;

impl LibraryController {
    // ── Selection mutations
    // ───────────────────────────────────────────────────

    /// Selects the item `id`, which the inspector pane then shows.
    ///
    /// Popping the item out into its own window is a separate, double-click
    /// action, requested through `InspectorController::request_pop_out`.
    pub fn select_item(&mut self, id: Arc<str>, cx: &mut Context<Self>) {
        self.maybe_check_item(Arc::clone(&id), cx);
        self.verify_selected_item_download(Arc::clone(&id), cx);
        self.selection = Selection::Item(id);
        cx.emit(LibraryChanged);
    }

    /// Clears the selection, so the inspector pane shows nothing.
    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selection = Selection::None;
        cx.emit(LibraryChanged);
    }

    // ── Item selection (multi-item detail tab) ───────────────────────────────

    /// Returns the row index of the currently selected item file for a
    /// multi-item entry's detail tab, if any item has been selected.
    #[must_use]
    pub fn selected_item_file(&self, entry_id: &str) -> Option<usize> {
        self.selected_item_file.get(entry_id).copied()
    }

    /// Selects the file at `row_ix` as the active item within `entry_id`'s
    /// expanded detail tab, updating the item metadata area in place.
    ///
    /// Clicking the already-selected row deselects it (toggle), returning
    /// the item list to its unselected prompt state.
    ///
    /// Emits [`LibraryChanged`].
    pub fn select_item_file(&mut self, entry_id: Arc<str>, row_ix: usize, cx: &mut Context<Self>) {
        toggle_row_selection(&mut self.selected_item_file, entry_id, row_ix);
        cx.emit(LibraryChanged);
    }

    /// Clears any selected item for `entry_id`'s detail tab, so reopening it
    /// shows no pre-selected item (selection is ephemeral, see
    /// `catalog-entry-detail-view`).
    ///
    /// Emits [`LibraryChanged`].
    pub fn clear_item_selection(&mut self, entry_id: &str, cx: &mut Context<Self>) {
        let had_selection = self.selected_item_file.remove(entry_id).is_some();
        let had_inspector_tab = self.inspector_tabs.remove(entry_id).is_some();
        let had_inspector_contents = self.inspector_contents.remove(entry_id).is_some();
        let had_inspector_selection = self.inspector_selected_entries.remove(entry_id).is_some();
        let had_inspector_extraction = self.inspector_extractions.remove(entry_id).is_some();
        let had_inspector_preview = self.inspector_preview.remove(entry_id).is_some();
        let _had_inspector_archive = self.inspector_selected_archive.remove(entry_id).is_some();
        let had_inspector_dirs = self.inspector_collapsed_dirs.remove(entry_id).is_some();
        let had_inspector_filter = self.inspector_filter_inputs.remove(entry_id).is_some();
        if had_selection
           || had_inspector_tab
           || had_inspector_contents
           || had_inspector_selection
           || had_inspector_extraction
           || had_inspector_preview
           || had_inspector_dirs
           || had_inspector_filter
        {
            cx.emit(LibraryChanged);
        }
    }
}
