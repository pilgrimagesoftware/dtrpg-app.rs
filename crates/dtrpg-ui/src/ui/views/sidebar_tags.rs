//! The Tags section of the sidebar: each tag with its color and how many items
//! have it. Choosing one shows only those items; a right click sets its color.

use dtrpg_core::data::tag_library::TagEntry;
use dtrpg_core::util::filter::SidebarFilter;
use gpui::prelude::*;
use gpui::{App, Entity, SharedString, div, px};
use gpui_kit::component::sidebar::SidebarMenuItem;

use crate::controllers::library::LibraryController;
use crate::data::theme::LibriTheme;
use crate::ui::library::tag::{color_menu, tag_dot};

/// Builds a row for each tag in `tags`. `active` is the filter now applied, so
/// its tag is marked.
pub fn tag_rows(tags: Vec<TagEntry>, active: &SidebarFilter, entity: &Entity<LibraryController>)
                -> Vec<SidebarMenuItem> {
    tags.into_iter()
        .map(|tag| tag_row(tag, active, entity))
        .collect()
}

/// One tag's row.
fn tag_row(tag: TagEntry, active: &SidebarFilter, entity: &Entity<LibraryController>)
           -> SidebarMenuItem {
    let is_active = matches!(active, SidebarFilter::Tag { name, .. }
                             if name.to_lowercase() == tag.name.to_lowercase());
    let (click_entity, color_entity) = (entity.clone(), entity.clone());
    let (name, color, count) = (tag.name.clone(), tag.color, tag.count);
    let menu_name = tag.name.clone();
    SidebarMenuItem::new(SharedString::from(tag.name.to_string()))
        .active(is_active)
        .suffix(move |_, cx: &mut App| {
            let no_color = cx.global::<LibriTheme>().colors.text_tertiary;
            div().flex()
                 .items_center()
                 .gap(px(6.0))
                 .text_xs()
                 .child(tag_dot(color, no_color))
                 .child(count.to_string())
                 .into_any_element()
        })
        .on_click(move |_, _, cx| {
            click_entity.update(cx, |ctrl, cx| ctrl.filter_by_tag(&name, cx));
        })
        .context_menu(move |menu, _, _| color_menu(menu, &color_entity, &menu_name, color))
}
