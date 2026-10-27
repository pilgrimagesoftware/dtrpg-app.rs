//! Sizing the list's columns to their widest item.

use std::sync::Arc;

use dtrpg_core::data::enums::ListColumn;
use dtrpg_core::data::library::LibraryItem;
use gpui::{App, Context, Entity, FontWeight, Pixels, Window, px};
use gpui_kit::component::table::TableState;

use super::grouped_delegate::*;
use super::list_cell::*;
use super::list_delegate::*;
use super::view::*;
use crate::ui::views::list_columns::{FIT_CANDIDATES, FIT_CELL_PADDING, widest_of_longest};

/// Room the title cell keeps for the badges beside the title, in pixels.
const TITLE_BADGES_ALLOWANCE: f32 = 72.0;

/// Room a sortable column's heading keeps for the sort arrow, in pixels.
const SORT_ARROW_ALLOWANCE: f32 = 20.0;

/// The text a column shows for `item`, or `None` for a column that shows a
/// glyph rather than text. `tags` is the item's tag names, already joined.
fn cell_text(column: ListColumn, item: &LibraryItem, tags: &str) -> Option<String> {
    match column {
        ListColumn::Title => Some(item.title.to_string()),
        ListColumn::Publisher => Some(item.publisher.to_string()),
        ListColumn::System => Some(item.line.to_string()),
        ListColumn::Pages => Some(item.pages.to_string()),
        ListColumn::Size => Some(format!("{:.0} MB", item.size_mb)),
        ListColumn::Added => Some(item.year.to_string()),
        ListColumn::Tags => Some(tags.to_owned()),
        ListColumn::Status | ListColumn::Open => None,
    }
}

impl CatalogView {
    /// Sizes the list's columns to their widest item whenever the items on show
    /// have changed since the last time.
    ///
    /// Measuring text shapes it, so the widest few items by length stand in for
    /// all of them. Called as the view renders; it does nothing while the items
    /// are the same.
    pub(super) fn refit_columns(&mut self, window: &Window, cx: &mut Context<Self>) {
        let items = self.controller.read(cx).visible_items_all();
        let unchanged = self.fitted_items
                            .as_ref()
                            .is_some_and(|fitted| Arc::ptr_eq(fitted, &items));
        if unchanged {
            return;
        }
        self.fitted_items = Some(Arc::clone(&items));

        let shown = self.catalog_list_table
                        .read(cx)
                        .delegate()
                        .cols
                        .shown()
                        .to_vec();
        let tags: Vec<String> = if shown.contains(&ListColumn::Tags) {
            let controller = self.controller.read(cx);
            items.iter()
                 .map(|item| {
                     controller.tags_of(&item.id)
                               .iter()
                               .map(|tag| tag.name.to_string())
                               .collect::<Vec<_>>()
                               .join(", ")
                 })
                 .collect()
        }
        else {
            Vec::new()
        };

        let font_size = text_sm_size(window);
        let measure =
            |text: &str| shaped_text_width(window, text, font_size, FontWeight::default());
        let widths: Vec<Option<Pixels>> =
            shown.iter()
                 .map(|column| {
                     let texts: Vec<String> = items.iter()
                                                   .enumerate()
                                                   .filter_map(|(ix, item)| {
                                                       let tags =
                                                           tags.get(ix).map_or("", String::as_str);
                                                       cell_text(*column, item, tags)
                                                   })
                                                   .collect();
                     if texts.is_empty() {
                         return None;
                     }
                     let widest = widest_of_longest(texts.iter().map(String::as_str),
                                                    FIT_CANDIDATES,
                                                    measure);
                     let heading = if column.is_sortable() {
                         SORT_ARROW_ALLOWANCE
                     }
                     else {
                         0.0
                     };
                     let extra = if *column == ListColumn::Title {
                         TITLE_BADGES_ALLOWANCE
                     }
                     else {
                         0.0
                     };
                     Some(widest.max(px(heading)) + px(FIT_CELL_PADDING + extra))
                 })
                 .collect();

        self.catalog_list_table.update(cx, |state, cx| {
                                   state.delegate_mut().cols.set_fitted(widths.clone());
                                   state.refresh(cx);
                               });
        self.catalog_grouped_list_table.update(cx, |state, cx| {
                                           state.delegate_mut().cols.set_fitted(widths);
                                           state.refresh(cx);
                                       });
    }
}

/// Tells both list tables how wide they were drawn, which limits their fitted
/// columns, and lays out again a table the new width changes.
pub(super) fn set_list_table_width(list: &Entity<TableState<CatalogListDelegate>>,
                                   grouped: &Entity<TableState<GroupedCatalogListDelegate>>,
                                   width: Pixels, cx: &mut App) {
    list.update(cx, |state, cx| {
            if state.delegate_mut().cols.set_table_width(width) {
                state.refresh(cx);
            }
        });
    grouped.update(cx, |state, cx| {
               if state.delegate_mut().cols.set_table_width(width) {
                   state.refresh(cx);
               }
           });
}
