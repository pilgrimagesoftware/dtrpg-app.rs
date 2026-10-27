use std::path::Path;

use dtrpg_core::data::file_openers::FileOpenerEntry;
use dtrpg_core::data::library::LibraryItem;
use gpui::prelude::FluentBuilder as _;
use gpui::{AnyElement, App, Entity, IntoElement, ParentElement, Styled, div, px};
use rust_i18n::t;

use super::metadata_table::*;
use super::other_details::*;
use super::sections::*;
use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::data::theme::{ColorTokens, LibriTheme};
use crate::ui::views::tags_section::render_tags_section;
use crate::ui::widgets::selectable_text;

/// Renders the Info tab: the description, tags, collections, a details table,
/// and what opens the file.
pub(super) fn render_info_tab(item: &LibraryItem, storage_root_path: &Path,
                              file_openers: &[FileOpenerEntry],
                              settings: &Entity<SettingsController>,
                              controller: Entity<LibraryController>, colors: &ColorTokens,
                              cx: &App)
                              -> AnyElement {
    let label_font_family = cx.global::<LibriTheme>().fonts.label_font.clone();
    let value_font_family = cx.global::<LibriTheme>().fonts.value_font.clone();
    let section = |heading: String, body: AnyElement| {
        render_section(heading, body, colors, &label_font_family)
    };

    let description = selectable_text("inspector-description", item.desc.to_string())
        .text_sm()
        .text_color(colors.text_secondary)
        .into_any_element();
    let tags = render_tags_section(item, controller.clone(), colors, cx).into_any_element();
    let collections =
        render_collections_section(item, controller.clone(), colors, cx).into_any_element();
    let location = controller.read(cx)
                             .item_location(&item.id)
                             .map(str::to_owned);
    let details = render_metadata_table(item,
                                        location,
                                        storage_root_path,
                                        colors,
                                        &label_font_family,
                                        &value_font_family).into_any_element();

    div().flex()
         .flex_col()
         .gap(px(20.0))
         .p(px(16.0))
         .when(!item.desc.trim().is_empty(), |this| {
             this.child(section(t!("detail.description_heading").to_string(), description))
         })
         .child(section(t!("detail.tags_heading").to_string(), tags))
         .child(section(t!("detail.collections_heading").to_string(), collections))
         .child(section(t!("detail.details_heading").to_string(), details))
         .child(section(t!("detail.opens_with_heading").to_string(),
                        render_opens_with(item, file_openers, settings, colors)))
         .into_any_element()
}
