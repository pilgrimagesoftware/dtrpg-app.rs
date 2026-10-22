//! Inspector controller: the state one inspector surface (the docked pane, or a
//! popped-out window) keeps, and the requests it makes of the main window.
//!
//! The inspector shows one item at a time. Controls inside it cannot reach the
//! main window directly, so they ask through this controller's events: pop the
//! item out into its own window, close the docked pane, or show an item in the
//! docked pane.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{App, AppContext as _, Context, Entity, Window};
use gpui_kit::component::table::{TableEvent, TableState};

use crate::controllers::library::LibraryController;
use crate::data::events::{
    InspectorCloseRequested, InspectorPopBackRequested, InspectorPopOutRequested,
    InspectorShowRequested,
};
use crate::ui::views::detail_panel_view::{ItemListDelegate, item_list_columns};
use crate::view_models::inspector::InspectorTab;

/// Where an inspector is shown.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InspectorPlacement {
    /// The pane docked beside the catalog in the main window.
    Docked,
    /// A window of its own for one item.
    Window,
}

/// Owns the state of one inspector surface and relays its requests.
pub struct InspectorController {
    placement:       InspectorPlacement,
    /// The file list of the one item shown most recently. A surface shows one
    /// item at a time, so one slot is enough, and the list keeps its column
    /// widths and selection across re-renders of the same item.
    item_list_table: Option<(Arc<str>, Entity<TableState<ItemListDelegate>>)>,
}

impl InspectorController {
    /// Creates the controller for an inspector shown at `placement`.
    #[must_use]
    pub fn new(placement: InspectorPlacement) -> Self {
        Self { placement,
               item_list_table: None }
    }

    /// Where this inspector is shown.
    #[must_use]
    pub fn placement(&self) -> InspectorPlacement {
        self.placement
    }

    /// Asks the main window to move the inspector for `item_id` into its own
    /// window.
    pub fn request_pop_out(&mut self, item_id: Arc<str>, cx: &mut Context<Self>) {
        cx.emit(InspectorPopOutRequested { item_id });
    }

    /// Asks the popped-out window this inspector belongs to to put its item
    /// back in the docked pane.
    pub fn request_pop_back(&mut self, cx: &mut Context<Self>) {
        cx.emit(InspectorPopBackRequested);
    }

    /// Asks the main window to show `item_id` in the docked pane, on `tab`,
    /// opening the pane if it is closed.
    pub fn request_show(&mut self, item_id: Arc<str>, tab: InspectorTab, cx: &mut Context<Self>) {
        cx.emit(InspectorShowRequested { item_id, tab });
    }

    /// Asks the main window to close the docked pane.
    pub fn request_close(&mut self, cx: &mut Context<Self>) {
        cx.emit(InspectorCloseRequested);
    }
}

/// Returns the cached item list `TableState` entity for `entry_id`, creating
/// and caching one on first use.
///
/// On cache-miss creation, subscribes to the new entity's `TableEvent`s so
/// row selection drives `LibraryController::select_item_file`, and — if the
/// controller already has a selection recorded for this entry (e.g. the tab
/// was closed and reopened after a cache eviction) — restores the visual
/// selection once via `TableState::set_selected_row`.
pub(crate) fn item_list_table(inspector: &Entity<InspectorController>,
                              controller: &Entity<LibraryController>, entry_id: &Arc<str>,
                              entry_dir: PathBuf, window: &mut Window, cx: &mut App)
                              -> Entity<TableState<ItemListDelegate>> {
    let cached = inspector.read(cx)
                          .item_list_table
                          .as_ref()
                          .filter(|(id, _)| id == entry_id)
                          .map(|(_, table)| table.clone());
    if let Some(table) = cached {
        return table;
    }

    let cols = item_list_columns();
    let col_count = cols.len();
    let delegate = ItemListDelegate { controller: controller.clone(),
                                      entry_id: Arc::clone(entry_id),
                                      columns: cols,
                                      user_widths: vec![None; col_count],
                                      entry_dir,
                                      table_width: None };
    let table = cx.new(|cx| {
                      TableState::new(delegate, window, cx).row_selectable(true)
                                                           .col_resizable(true)
                                                           .sortable(false)
                  });

    if let Some(selected_ix) = controller.read(cx).selected_item_file(entry_id) {
        table.update(cx, |state, cx| state.set_selected_row(selected_ix, cx));
    }

    cx.subscribe(&table, {
          let controller = controller.clone();
          let entry_id = Arc::clone(entry_id);
          move |_table, event: &TableEvent, cx| {
              if let TableEvent::SelectRow(row_ix) = event {
                  let row_ix = *row_ix;
                  let entry_id = Arc::clone(&entry_id);
                  controller.update(cx, |ctrl, cx| ctrl.select_item_file(entry_id, row_ix, cx));
              }
          }
      })
      .detach();

    inspector.update(cx, |inspector, _cx| {
                 inspector.item_list_table = Some((Arc::clone(entry_id), table.clone()));
             });

    table
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    /// An item and tab a show request named.
    type ShownRequest = (Arc<str>, InspectorTab);

    #[test]
    fn a_new_controller_remembers_where_it_is_shown() {
        assert_eq!(InspectorController::new(InspectorPlacement::Docked).placement(),
                   InspectorPlacement::Docked);
        assert_eq!(InspectorController::new(InspectorPlacement::Window).placement(),
                   InspectorPlacement::Window);
    }

    #[gpui::test]
    fn requesting_pop_out_announces_the_item(cx: &mut gpui::TestAppContext) {
        let requested: Rc<RefCell<Vec<Arc<str>>>> = Rc::default();
        let inspector = cx.new(|_| InspectorController::new(InspectorPlacement::Docked));
        let log = Rc::clone(&requested);
        cx.update(|cx| {
              cx.subscribe(&inspector,
                           move |_inspector, event: &InspectorPopOutRequested, _cx| {
                               log.borrow_mut().push(Arc::clone(&event.item_id));
                           })
                .detach();
          });

        inspector.update(cx, |inspector, cx| {
                     inspector.request_pop_out(Arc::from("item-1"), cx)
                 });

        assert_eq!(requested.borrow().as_slice(), [Arc::from("item-1")]);
    }

    #[gpui::test]
    fn requesting_show_announces_the_item_and_tab(cx: &mut gpui::TestAppContext) {
        let requested: Rc<RefCell<Vec<ShownRequest>>> = Rc::default();
        let inspector = cx.new(|_| InspectorController::new(InspectorPlacement::Window));
        let log = Rc::clone(&requested);
        cx.update(|cx| {
              cx.subscribe(&inspector,
                           move |_inspector, event: &InspectorShowRequested, _cx| {
                               log.borrow_mut()
                                  .push((Arc::clone(&event.item_id), event.tab));
                           })
                .detach();
          });

        inspector.update(cx, |inspector, cx| {
                     inspector.request_show(Arc::from("item-1"), InspectorTab::Files, cx)
                 });

        assert_eq!(requested.borrow().as_slice(),
                   [(Arc::from("item-1"), InspectorTab::Files)]);
    }

    #[gpui::test]
    fn requesting_close_is_announced(cx: &mut gpui::TestAppContext) {
        let closes = Rc::new(RefCell::new(0));
        let inspector = cx.new(|_| InspectorController::new(InspectorPlacement::Docked));
        let count = Rc::clone(&closes);
        cx.update(|cx| {
              cx.subscribe(&inspector,
                           move |_inspector, _event: &InspectorCloseRequested, _cx| {
                               *count.borrow_mut() += 1;
                           })
                .detach();
          });

        inspector.update(cx, |inspector, cx| inspector.request_close(cx));

        assert_eq!(*closes.borrow(), 1);
    }
}
