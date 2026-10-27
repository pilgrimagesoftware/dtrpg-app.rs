//! Catalog view: list, thumbs, and grid layouts with grouping and empty state.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use dtrpg_core::data::library::LibraryItem;
use dtrpg_core::util::publisher::ItemGroup;
use gpui::prelude::*;
use gpui::{Context, Entity, IntoElement, Pixels, Render, UniformListScrollHandle, Window};
use gpui_kit::component::table::{TableEvent, TableState};

use crate::controllers::inspector::InspectorController;
use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::data::events::LibraryChanged;
use crate::ui::views::list_columns::ListColumnSet;

#[derive(Clone, Copy)]
pub(super) enum EmptyReason {
    LibraryEmpty,
    NoMatches,
}

// ── CatalogView
// ───────────────────────────────────────────────────────────────

/// GPUI view for the catalog area. Holds scroll state and delegates to
/// `DataTable` for list layout and `uniform_list` for thumbs/grid layouts.
pub struct CatalogView {
    pub(super) controller:                 Entity<LibraryController>,
    pub(super) settings:                   Entity<SettingsController>,
    pub(super) inspector:                  Entity<InspectorController>,
    pub(super) scroll_handle:              UniformListScrollHandle,
    pub(super) catalog_list_table:         Entity<TableState<CatalogListDelegate>>,
    /// Backs the grouped list presentation. Rows are a flattened list of
    /// group headers and items (see `GroupedRow`), rebuilt only when
    /// `grouped_cache` is refreshed — this is what lets the grouped list
    /// reuse `DataTable`'s row virtualization instead of hand-rolling and
    /// fully materializing its own rows.
    pub(super) catalog_grouped_list_table: Entity<TableState<GroupedCatalogListDelegate>>,
    /// Cached items-per-row for the grid layout; updated each render from
    /// the window viewport width. Initialized to 4 as a safe default.
    pub(super) items_per_row:              usize,
    /// The width the catalog area was last drawn at, which is what is left
    /// beside the sidebar, the filters column and the inspector. Written as
    /// the area is laid out and read by the next render.
    pub(super) drawn_width:                Rc<Cell<Option<Pixels>>>,
    /// Cached reference to the controller's grouped items. Compared by Arc
    /// pointer so the grouped list only rebuilds its flattened rows when the
    /// controller cache actually changes, not on every hover-triggered
    /// re-render. Also drives the flattened rows pushed into
    /// `catalog_grouped_list_table`.
    pub(super) grouped_cache:              Option<Arc<Vec<ItemGroup>>>,
    /// The items the list columns were last sized to fit. Compared by Arc
    /// pointer, like `grouped_cache`, so the columns are measured again only
    /// when the items on show change.
    pub(super) fitted_items:               Option<Arc<Vec<LibraryItem>>>,
}

impl CatalogView {
    /// Creates a new `CatalogView` connected to the given controller and
    /// settings.
    pub fn new(window: &mut Window, cx: &mut Context<Self>,
               controller: Entity<LibraryController>, settings: Entity<SettingsController>,
               inspector: Entity<InspectorController>)
               -> Self {
        let storage_root = settings.read(cx).snapshot().storage_root_path;
        let chosen_columns = controller.read(cx).list_columns.clone();
        let delegate = CatalogListDelegate { controller:   controller.clone(),
                                             inspector:    inspector.clone(),
                                             storage_root: storage_root.clone(),
                                             cols:         ListColumnSet::new(&chosen_columns), };
        let catalog_list_table = cx.new(|cx| {
                                       TableState::new(delegate, window, cx).row_selectable(true)
                                                                            .col_selectable(false)
                                                                            .col_movable(false)
                                                                            .col_resizable(true)
                                                                            .sortable(true)
                                   });

        let grouped_delegate = GroupedCatalogListDelegate { controller: controller.clone(),
                                                            inspector: inspector.clone(),
                                                            storage_root,
                                                            cols:
                                                                ListColumnSet::new(&chosen_columns),
                                                            rows: Vec::new() };
        let catalog_grouped_list_table = cx.new(|cx| {
                                               TableState::new(grouped_delegate, window, cx)
                .row_selectable(true)
                .col_selectable(false)
                .col_movable(false)
                .col_resizable(true)
                .sortable(false)
                                           });

        cx.subscribe(&controller, {
              let table = catalog_list_table.clone();
              move |this, ctrl, _event: &LibraryChanged, cx| {
                  let chosen = ctrl.read(cx).list_columns.clone();
                  this.catalog_grouped_list_table.update(cx, |state, cx| {
                                                     if state.delegate_mut()
                                                             .cols
                                                             .set_chosen(&chosen)
                                                     {
                                                         state.refresh(cx);
                                                     }
                                                 });
                  let columns_changed = table.update(cx, |state, cx| {
                                                 let changed =
                                                     state.delegate_mut().cols.set_chosen(&chosen);
                                                 state.refresh(cx);
                                                 changed
                                             });
                  if columns_changed {
                      this.fitted_items = None;
                  }
                  this.grouped_cache = None;
              }
          })
          .detach();

        cx.subscribe(&catalog_list_table,
                     |this, table, event: &TableEvent, cx| match event {
                         TableEvent::SelectRow(row_ix) => {
                             let row_ix = *row_ix;
                             let items = this.controller
                                             .read(cx)
                                             .visible_items_slice(row_ix..row_ix + 1);
                             if let Some(item) = items.first() {
                                 let id = Arc::clone(&item.id);
                                 this.controller
                                     .update(cx, |ctrl, cx| ctrl.select_item(id, cx));
                             }
                         }
                         TableEvent::DoubleClickedRow(row_ix) => {
                             let row_ix = *row_ix;
                             let items = this.controller
                                             .read(cx)
                                             .visible_items_slice(row_ix..row_ix + 1);
                             if let Some(item) = items.first() {
                                 let id = Arc::clone(&item.id);
                                 this.inspector
                                     .update(cx, |inspector, cx| inspector.request_pop_out(id, cx));
                             }
                         }
                         TableEvent::ColumnWidthsChanged(widths) => {
                             let widths = widths.clone();
                             table.update(cx, |state, _cx| {
                                      state.delegate_mut().cols.apply_widths(&widths);
                                  });
                             this.catalog_grouped_list_table.update(cx, |state, _cx| {
                                                                state.delegate_mut()
                                                                     .cols
                                                                     .apply_widths(&widths);
                                                            });
                         }
                         _ => {}
                     })
          .detach();

        cx.subscribe(&catalog_grouped_list_table,
                     |this, table, event: &TableEvent, cx| match event {
                         // Group header rows aren't selectable items — ignore clicks on them.
                         TableEvent::SelectRow(row_ix) => {
                             let row_ix = *row_ix;
                             let id = table.read(cx).delegate().row_at(row_ix).and_then(|row| {
                                                                                  match row {
                            GroupedRow::Item(item) => Some(Arc::clone(&item.id)),
                            GroupedRow::Header { .. } => None,
                        }
                                                                              });
                             if let Some(id) = id {
                                 this.controller
                                     .update(cx, |ctrl, cx| ctrl.select_item(id, cx));
                             }
                         }
                         TableEvent::DoubleClickedRow(row_ix) => {
                             let row_ix = *row_ix;
                             let item = table.read(cx).delegate().row_at(row_ix).and_then(|row| {
                                                                                    match row {
                            GroupedRow::Item(item) => Some(item.clone()),
                            GroupedRow::Header { .. } => None,
                        }
                                                                                });
                             if let Some(item) = item {
                                 let id = Arc::clone(&item.id);
                                 this.inspector
                                     .update(cx, |inspector, cx| inspector.request_pop_out(id, cx));
                             }
                         }
                         TableEvent::ColumnWidthsChanged(widths) => {
                             let widths = widths.clone();
                             table.update(cx, |state, _cx| {
                                      state.delegate_mut().cols.apply_widths(&widths);
                                  });
                             this.catalog_list_table.update(cx, |state, _cx| {
                                                        state.delegate_mut()
                                                             .cols
                                                             .apply_widths(&widths);
                                                    });
                         }
                         _ => {}
                     })
          .detach();

        Self { controller,
               settings,
               inspector,
               scroll_handle: UniformListScrollHandle::default(),
               catalog_list_table,
               catalog_grouped_list_table,
               items_per_row: 4,
               drawn_width: Rc::new(Cell::new(None)),
               grouped_cache: None,
               fitted_items: None }
    }

    /// Returns the controller's grouped items, rebuilding the flattened rows
    /// for `catalog_grouped_list_table` only when the controller's cache has
    /// changed.
    pub(super) fn grouped_items(&mut self, cx: &mut Context<Self>) -> Vec<ItemGroup> {
        let controller = self.controller.read(cx);
        let groups = controller.visible_items_grouped();
        let rebuild = self.grouped_cache
                          .as_ref()
                          .is_none_or(|cached| !Arc::ptr_eq(cached, &groups));
        if rebuild {
            self.grouped_cache = Some(Arc::clone(&groups));

            let group_kind = controller.group_by;
            let mut rows = Vec::new();
            for group in groups.iter() {
                rows.push(GroupedRow::Header { title: Arc::clone(&group.title),
                                               count: group.items.len(),
                                               kind:  group_kind, });
                rows.extend(group.items
                                 .iter()
                                 .map(|item| GroupedRow::Item(Box::new(item.clone()))));
            }
            self.catalog_grouped_list_table.update(cx, |state, cx| {
                                               state.delegate_mut().rows = rows;
                                               state.refresh(cx);
                                           });
        }
        (*groups).clone()
    }
}

impl Render for CatalogView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        super::render::render_catalog_view(self, window, cx)
    }
}

use super::grouped_delegate::*;
pub(crate) use super::list_cell::*;
use super::list_delegate::*;
