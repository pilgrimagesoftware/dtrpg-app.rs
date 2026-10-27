//! The inspector's Contents tab: which archive is shown, and a scrolling list
//! of what is in it.

use std::sync::Arc;

use dtrpg_core::util::container::ContainerEntry;
use gpui::uniform_list;
use gpui::{
    AnyElement, App, Entity, InteractiveElement, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement, Styled, div, px,
};
use gpui_kit::component::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use rust_i18n::t;

use super::inspector::*;
use crate::controllers::library::LibraryController;
use crate::data::theme::ColorTokens;
use crate::util::file_size::format_bytes;
use crate::util::pluralize::pluralize;
use crate::view_models::inspector::InspectorContentsState;

/// The height of the entry list, in pixels. The list scrolls inside it.
const ENTRY_LIST_HEIGHT: f32 = 340.0;

/// The height of one entry row, in pixels. Every row is the same height, which
/// is what lets the list draw only the rows in view.
const ENTRY_ROW_HEIGHT: f32 = 28.0;

/// How far each level of folders is indented, in pixels.
const ENTRY_INDENT: f32 = 14.0;

/// Renders the Contents tab for `entry_id`: the archive's name (a menu to pick
/// another when the item has several), then the entries once they are loaded.
pub(super) fn render_inspector_contents(state: &InspectorContentsState, entry_id: &Arc<str>,
                                        entity: Entity<LibraryController>, colors: &ColorTokens,
                                        cx: &App)
                                        -> AnyElement {
    let header = render_archive_header(state, entry_id, &entity, colors, cx);
    let body = match state {
        InspectorContentsState::Unavailable => {
            message(t!("detail.inspector_contents_unavailable").to_string(),
                    colors)
        }
        InspectorContentsState::Loading => {
            message(t!("detail.inspector_contents_loading").to_string(), colors)
        }
        InspectorContentsState::Loaded(entries) => {
            render_loaded(entries, entry_id, &entity, colors, cx)
        }
    };
    div().p(px(16.0))
         .flex()
         .flex_col()
         .gap(px(10.0))
         .children(header)
         .child(body)
         .into_any_element()
}

/// A line of text where the entries would be.
fn message(text: String, colors: &ColorTokens) -> AnyElement {
    div().text_sm()
         .text_color(colors.text_secondary)
         .child(text)
         .into_any_element()
}

/// Says which archive the entries are from: its name, and how many entries it
/// holds. When the item has more than one archive, the name is a menu for
/// choosing which to show. `None` when the item has no archive.
fn render_archive_header(state: &InspectorContentsState, entry_id: &Arc<str>,
                         entity: &Entity<LibraryController>, colors: &ColorTokens, cx: &App)
                         -> Option<AnyElement> {
    let archives = entity.read(cx).inspector_archive_files(entry_id);
    let selected = entity.read(cx).inspector_selected_archive_for(entry_id);
    let name = archives.get(selected)?.clone();
    let count = match state {
        InspectorContentsState::Loaded(entries) => Some(pluralize(entries.len(),
                                                                  "detail.inspector_contents_entry",
                                                                  "detail.inspector_contents_entries")),
        _ => None,
    };
    let title: AnyElement = if archives.len() > 1 {
        let entity = entity.clone();
        let entry_id = Arc::clone(entry_id);
        Button::new("inspector-archive-picker").outline()
                                               .label(name)
                                               .dropdown_caret(true)
                                               .tooltip(t!("detail.inspector_archive_picker_tooltip")
                                                            .to_string())
                                               .dropdown_menu(move |menu, _, _| {
                                                   archives.iter()
                                                           .enumerate()
                                                           .fold(menu, |menu, (index, archive)| {
                                                               let entity = entity.clone();
                                                               let entry_id = Arc::clone(&entry_id);
                                                               menu.item(PopupMenuItem::new(archive.clone())
                                                                             .checked(index == selected)
                                                                             .on_click(move |_, _, cx| {
                                                                                 entity.update(cx, |ctrl, cx| {
                                                                                           ctrl.set_inspector_selected_archive(Arc::clone(&entry_id), index, cx);
                                                                                       });
                                                                             }))
                                                           })
                                               })
                                               .into_any_element()
    }
    else {
        div().text_sm()
             .font_weight(gpui::FontWeight::SEMIBOLD)
             .text_color(colors.text_primary)
             .truncate()
             .child(name)
             .into_any_element()
    };
    Some(div().flex()
              .items_center()
              .gap(px(8.0))
              .child(title)
              .children(count.map(|count| {
                                 div().flex_none()
                                      .text_xs()
                                      .text_color(colors.text_secondary)
                                      .child(count)
                             }))
              .into_any_element())
}

/// The preview of the chosen entry, the list of entries, and the button that
/// extracts the ticked ones.
///
/// The list is virtual: only the rows in view are built, so an archive with
/// thousands of entries scrolls as smoothly as one with a few.
fn render_loaded(entries: &Arc<Vec<ContainerEntry>>, entry_id: &Arc<str>,
                 entity: &Entity<LibraryController>, colors: &ColorTokens, cx: &App)
                 -> AnyElement {
    let selected_count = entity.read(cx).selected_inspector_entry_count(entry_id);
    let extracting = entity.read(cx).is_inspector_extracting(entry_id);
    let preview = entity.read(cx).inspector_preview_for(entry_id);

    let list = {
        let entries = Arc::clone(entries);
        let entity = entity.clone();
        let entry_id = Arc::clone(entry_id);
        let colors = colors.clone();
        uniform_list("inspector-contents-list",
                     entries.len(),
                     move |range, _window, cx| {
                         range.filter_map(|index| {
                                  let entry = entries.get(index)?;
                                  let selected =
                                      entity.read(cx)
                                            .is_inspector_entry_selected(&entry_id, &entry.name);
                                  Some(render_entry_row(entry, selected, &entry_id, &entity,
                                                        &colors))
                              })
                              .collect()
                     }).size_full()
    };

    div().flex()
         .flex_col()
         .gap(px(8.0))
         .children(render_inspector_preview(&preview, entry_id, entity.clone(), colors))
         .child(div().h(px(ENTRY_LIST_HEIGHT))
                     .w_full()
                     .rounded_md()
                     .border_1()
                     .border_color(colors.border)
                     .child(list))
         .child(Button::new("inspector-extract-selected")
                    .ghost()
                    .disabled(selected_count == 0 || extracting)
                    .loading(extracting)
                    .child(t!("detail.inspector_extract_selected").to_string())
                    .on_click({
                        let click_entity = entity.clone();
                        let click_id = Arc::clone(entry_id);
                        move |_, _, cx| {
                            click_entity.update(cx, |controller, cx| {
                                            controller.extract_inspector_entries(Arc::clone(&click_id),
                                                                                 cx);
                                        });
                        }
                    }))
         .into_any_element()
}

/// One entry: a box to tick it for extraction (a folder has an arrow instead),
/// its name indented by how deep it is, its size, and a Preview button.
fn render_entry_row(entry: &ContainerEntry, selected: bool, entry_id: &Arc<str>,
                    entity: &Entity<LibraryController>, colors: &ColorTokens)
                    -> AnyElement {
    let name = entry.name.clone();
    let is_directory = entry.is_directory;
    let depth = name.trim_end_matches('/').matches('/').count();
    let label = name.trim_end_matches('/')
                    .rsplit('/')
                    .next()
                    .unwrap_or(&name)
                    .to_string();
    let click_entity = entity.clone();
    let click_id = Arc::clone(entry_id);
    let toggle_name = name.clone();
    let preview_entity = entity.clone();
    let preview_id = Arc::clone(entry_id);
    let preview_name = name.clone();
    div().id(SharedString::from(format!("inspector-entry-{name}")))
         .h(px(ENTRY_ROW_HEIGHT))
         .px(px(8.0))
         .pl(px(8.0 + depth as f32 * ENTRY_INDENT))
         .flex()
         .items_center()
         .gap(px(8.0))
         .cursor_pointer()
         .on_click(move |_, _, cx| {
             if !is_directory {
                 click_entity.update(cx, |controller, cx| {
                                 controller.toggle_inspector_entry(Arc::clone(&click_id),
                                                                   toggle_name.clone(),
                                                                   cx);
                             });
             }
         })
         .child(if is_directory {
                    "\u{25B8}"
                }
                else if selected {
                    "\u{2611}"
                }
                else {
                    "\u{2610}"
                })
         .child(div().flex_1()
                     .min_w_0()
                     .truncate()
                     .text_sm()
                     .text_color(colors.text_primary)
                     .child(if is_directory { format!("{label}/") } else { label }))
         .child(div().flex_none()
                     .text_xs()
                     .text_color(colors.text_tertiary)
                     .child(if is_directory {
                                String::new()
                            }
                            else {
                                format_bytes(entry.size_bytes)
                            }))
         .child(if is_directory {
                    div().into_any_element()
                }
                else {
                    Button::new(SharedString::from(format!("inspector-preview-{name}")))
                        .ghost()
                        .child(t!("detail.inspector_preview").to_string())
                        .on_click(move |_, _, cx| {
                            cx.stop_propagation();
                            preview_entity.update(cx, |controller, cx| {
                                              controller.preview_inspector_entry(Arc::clone(&preview_id),
                                                                                 preview_name.clone(),
                                                                                 cx);
                                          });
                        })
                        .into_any_element()
                })
         .into_any_element()
}
