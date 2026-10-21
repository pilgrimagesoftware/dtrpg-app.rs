//! The Smart collections section of the sidebar: each collection with how many
//! items match its rules. Choosing one shows only those items.
//!
//! Unlike a manual collection, a smart collection does not take dropped items,
//! because its members come from its rules. So its rows are the sidebar's own
//! menu items, not a hand-drawn row like the manual collections'.

use std::sync::Arc;

use dtrpg_core::util::filter::SidebarFilter;
use gpui::prelude::*;
use gpui::{App, Entity, SharedString, Window, div};
use gpui_kit::component::Icon;
use gpui_kit::component::WindowExt as _;
use gpui_kit::component::menu::PopupMenuItem;
use gpui_kit::component::sidebar::{SidebarMenu, SidebarMenuItem};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::ui::views::sidebar_view::SIDEBAR_ROW_GAP;

/// What the sidebar needs to draw one smart collection.
#[derive(Clone)]
pub struct SmartCollectionRow {
    /// The collection's id.
    pub id:    u64,
    /// The collection's name.
    pub name:  Arc<str>,
    /// How many items match its rules.
    pub count: usize,
}

/// Builds the Smart collections menu: a header over a row for each collection
/// in `rows`. `active` is the filter now applied, so its row is marked.
pub fn smart_collections_menu(rows: Vec<SmartCollectionRow>, active: &SidebarFilter,
                              entity: &Entity<LibraryController>)
                              -> SidebarMenu {
    let count = rows.len();
    let items: Vec<SidebarMenuItem> = rows.into_iter()
                                          .map(|row| smart_collection_row(row, active, entity))
                                          .collect();
    SidebarMenu::new().gap(SIDEBAR_ROW_GAP)
                      .child(SidebarMenuItem::new(t!("sidebar.smart_collections").to_string())
        .default_open(true)
        .click_to_toggle(true)
        .suffix(move |_, _| div().text_xs().child(count.to_string()))
        .children(items))
}

/// One smart collection's row. A right click offers to delete it.
fn smart_collection_row(row: SmartCollectionRow, active: &SidebarFilter,
                        entity: &Entity<LibraryController>)
                        -> SidebarMenuItem {
    let filter = SidebarFilter::SmartCollection(row.id, Arc::clone(&row.name));
    let is_active = *active == filter;
    let (click_entity, delete_entity) = (entity.clone(), entity.clone());
    let (id, name, count) = (row.id, row.name.to_string(), row.count);
    SidebarMenuItem::new(SharedString::from(name.clone()))
        .icon(Icon::empty().path("icons/folder-cog.svg"))
        .active(is_active)
        .suffix(move |_, _| div().text_xs().child(count.to_string()))
        .on_click(move |_, _, cx| {
            click_entity.update(cx, |ctrl, cx| ctrl.set_filter(filter.clone(), cx));
        })
        .context_menu(move |menu, _, _| {
            let (entity, name) = (delete_entity.clone(), name.clone());
            menu.item(PopupMenuItem::new(t!("smart_collections.delete")).on_click(
                move |_, window, cx| confirm_delete(&entity, id, &name, window, cx),
            ))
        })
}

/// Asks for confirmation, then deletes the smart collection `id`.
fn confirm_delete(entity: &Entity<LibraryController>, id: u64, name: &str, window: &mut Window,
                  cx: &mut App) {
    let entity = entity.clone();
    let title = t!("smart_collections.delete_confirm_title", name = name).to_string();
    window.open_alert_dialog(cx, move |alert, _, _| {
              let entity = entity.clone();
              alert.confirm()
                   .title(title.clone())
                   .description(t!("smart_collections.delete_confirm_description").to_string())
                   .on_ok(move |_, _, cx| {
                       entity.update(cx, |ctrl, cx| ctrl.delete_smart_collection(id, cx));
                       true
                   })
          });
}
