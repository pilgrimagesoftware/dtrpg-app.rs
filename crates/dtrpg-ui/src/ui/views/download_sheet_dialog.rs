//! The download sheet: choose which of an item's files to fetch and which
//! storage root to put them in.
//!
//! The choices live in a [`DownloadSheet`]; this file draws them and passes
//! each click to it. Pressing Download hands the sheet's plan to the
//! controller, which queues the files.

use std::cell::RefCell;
use std::rc::Rc;

use gpui::prelude::*;
use gpui::{App, Entity, SharedString, Window, div, px};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::dialog::{
    DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
};
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{Disableable, WindowExt as _, v_flex};
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::theme::{ColorTokens, LibriTheme};
use crate::view_models::download_sheet::{DownloadSheet, SheetFile, SheetRoot};
use crate::view_models::storage_display::volume_size_label;

/// Opens the download sheet for `sheet`'s item.
pub fn open_download_sheet_dialog(window: &mut Window, cx: &mut App,
                                  controller: Entity<LibraryController>, sheet: DownloadSheet) {
    let state = Rc::new(RefCell::new(sheet));
    window.open_dialog(cx, move |dialog, _window, _cx| {
              let controller = controller.clone();
              let state = state.clone();
              dialog.w(px(520.))
                    .overlay_closable(true)
                    .content(move |content, _window, cx| {
                        render_content(content, cx, &controller, &state)
                    })
          });
}

/// Starts a download of the item `entry_id` from a menu or button: opens the
/// download sheet so the person chooses the files and the location, or, for an
/// item with no file list yet, asks the controller to download it directly.
///
/// Every place that offers to download a single item calls this, so the sheet
/// is the one way a download is set up.
pub fn request_download_with_sheet(window: &mut Window, cx: &mut App,
                                   controller: &Entity<LibraryController>, entry_id: &str,
                                   title: String) {
    let sheet = controller.read(cx).download_sheet_for(entry_id);
    match sheet {
        Some(sheet) => open_download_sheet_dialog(window, cx, controller.clone(), sheet),
        None => {
            controller.update(cx, |ctrl, cx| ctrl.request_download(entry_id, title, cx));
        }
    }
}

/// Megabytes as the app writes sizes: megabytes, then gigabytes, then
/// terabytes, for example `162.4 MB` or `527.2 GB`.
fn size_label(mb: f64) -> String {
    volume_size_label(mb.max(0.0))
}

fn section_heading(label: String, colors: &ColorTokens) -> impl IntoElement {
    div().text_xs()
         .font_weight(gpui::FontWeight::SEMIBOLD)
         .text_color(colors.text_secondary)
         .child(label.to_uppercase())
}

fn render_file_row(file: &SheetFile, selected: bool, colors: &ColorTokens,
                   state: &Rc<RefCell<DownloadSheet>>)
                   -> impl IntoElement {
    let index = file.index;
    let state = state.clone();
    div().flex()
         .items_center()
         .gap(px(10.0))
         .py(px(6.0))
         .child(Checkbox::new(("download-sheet-file", u64::from(index)))
                    .checked(selected)
                    .disabled(file.downloaded)
                    .on_click(move |_, window, _cx| {
                        state.borrow_mut().toggle_file(index);
                        window.refresh();
                    }))
         .child(div().flex_none()
                     .px(px(6.0))
                     .py(px(2.0))
                     .rounded_md()
                     .bg(colors.surface_alt)
                     .text_xs()
                     .text_color(colors.text_secondary)
                     .child(file.format.to_string()))
         .child({
             // A long name is cut off, with the whole name in a tooltip, so the
             // size beside it is never pushed out of the sheet.
             let full_name = file.name.to_string();
             div().id(("download-sheet-name", u64::from(index)))
                  .flex_1()
                  .min_w_0()
                  .truncate()
                  .text_sm()
                  .text_color(colors.text_primary)
                  .tooltip(move |window, cx| {
                      gpui_kit::component::tooltip::Tooltip::new(full_name.clone()).build(window,
                                                                                         cx)
                  })
                  .child(file.name.to_string())
         })
         .child(div().flex_none()
                     .text_sm()
                     .text_color(colors.text_secondary)
                     .child(size_label(file.size_mb)))
         .when(file.downloaded, |row| {
             row.child(div().id(("download-sheet-done", u64::from(index)))
                            .text_sm()
                            .text_color(colors.accent)
                            .child("\u{2713}")
                            .tooltip(|window, cx| {
                                gpui_kit::component::tooltip::Tooltip::new(
                                    t!("download_sheet.already_downloaded").to_string(),
                                )
                                .build(window, cx)
                            }))
         })
}

fn render_root_card(root: &SheetRoot, chosen: bool, colors: &ColorTokens,
                    state: &Rc<RefCell<DownloadSheet>>)
                    -> impl IntoElement {
    let id = root.id;
    let state = state.clone();
    let free =
        root.free_mb
            .map_or_else(|| t!("download_sheet.free_space_unknown").to_string(),
                         |mb| t!("download_sheet.free_space", size = size_label(mb)).to_string());
    div().id(("download-sheet-root", u64::from(id)))
         .flex()
         .items_center()
         .justify_between()
         .gap(px(12.0))
         .p(px(12.0))
         .rounded_lg()
         .border_1()
         .border_color(if chosen { colors.accent } else { colors.border })
         .when(chosen, |card| card.bg(colors.surface_alt))
         .cursor_pointer()
         .on_click(move |_, window, _cx| {
             state.borrow_mut().choose_root(id);
             window.refresh();
         })
         .child(v_flex().flex_1()
                        .min_w_0()
                        .gap(px(2.0))
                        .child(div().truncate()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(colors.text_primary)
                                    .child(root.name.clone()))
                        .child(div().truncate()
                                    .text_xs()
                                    .font_family("monospace".to_string())
                                    .text_color(colors.text_secondary)
                                    .child(root.path.clone())))
         .child(div().flex_none()
                     .text_sm()
                     .text_color(colors.text_secondary)
                     .child(free))
}

fn render_remember_row(sheet: &DownloadSheet, colors: &ColorTokens,
                       state: &Rc<RefCell<DownloadSheet>>)
                       -> impl IntoElement {
    let locked = sheet.remember_is_locked();
    let state = state.clone();
    v_flex().gap(px(4.0))
            .child(div().flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_sm()
                                    .text_color(colors.text_primary)
                                    .child(t!("download_sheet.remember",
                                              publisher = sheet.publisher.to_string()).to_string()))
                        .child(Switch::new("download-sheet-remember")
                                   .checked(sheet.remember())
                                   .disabled(locked)
                                   .on_click(move |checked, window, _cx| {
                                       state.borrow_mut().set_remember(*checked);
                                       window.refresh();
                                   })))
            .when(locked, |column| {
                column.child(div().text_xs()
                                  .text_color(colors.text_secondary)
                                  .child(t!("download_sheet.remember_locked").to_string()))
            })
}

fn summary_label(sheet: &DownloadSheet) -> String {
    let size = size_label(sheet.selected_size_mb());
    match sheet.selected_count() {
        1 => t!("download_sheet.summary_one", size = size).to_string(),
        count => t!("download_sheet.summary_many", count = count, size = size).to_string(),
    }
}

fn render_content(content: DialogContent, cx: &mut App, controller: &Entity<LibraryController>,
                  state: &Rc<RefCell<DownloadSheet>>)
                  -> DialogContent {
    let colors = cx.global::<LibriTheme>().colors.clone();
    let sheet = state.borrow().clone();

    let all_none = div().flex()
                        .items_center()
                        .gap(px(4.0))
                        .child(Button::new("download-sheet-all").ghost().label(t!("download_sheet.select_all").to_string()).on_click({
                            let state = state.clone();
                            move |_, window, _cx| {
                                state.borrow_mut().select_all();
                                window.refresh();
                            }
                        }))
                        .child(Button::new("download-sheet-none").ghost().label(t!("download_sheet.select_none").to_string()).on_click({
                            let state = state.clone();
                            move |_, window, _cx| {
                                state.borrow_mut().select_none();
                                window.refresh();
                            }
                        }));

    let files = v_flex().w_full()
                        .min_w_0()
                        .children(sheet.files.iter().map(|file| {
                                                        render_file_row(file,
                                                           sheet.is_selected(file.index),
                                                           &colors,
                                                           state).into_any_element()
                                                    }));
    let roots = v_flex().w_full()
                        .min_w_0()
                        .gap(px(8.0))
                        .children(sheet.roots.iter().map(|root| {
                                                        render_root_card(root,
                                                                         root.id
                                                                         == sheet.chosen_root_id(),
                                                                         &colors,
                                                                         state).into_any_element()
                                                    }));

    let shortfall = sheet.space_shortfall().map(|shortfall| {
                                               t!("download_sheet.shortfall",
                                                  root = shortfall.root_name,
                                                  needed = size_label(shortfall.needed_mb),
                                                  free = size_label(shortfall.free_mb)).to_string()
                                           });

    let download = {
        let controller = controller.clone();
        let state = state.clone();
        Button::new("download-sheet-download").primary()
                                              .label(t!("download_sheet.download").to_string())
                                              .disabled(!sheet.can_download())
                                              .on_click(move |_, window, cx| {
                                                  let sheet = state.borrow().clone();
                                                  let plan = sheet.plan();
                                                  controller.update(cx, |controller, cx| {
                                                                controller.start_sheet_download(&sheet.item_id, &plan, cx);
                                                            });
                                                  window.close_dialog(cx);
                                              })
    };

    content.child(DialogHeader::new().px_4()
                                     .pt_4()
                                     .child(DialogTitle::new().child(SharedString::from(
                                         t!("download_sheet.title", title = sheet.title.to_string()).to_string(),
                                     )))
                                     .child(DialogDescription::new().child(SharedString::from(
                                         t!("download_sheet.subtitle").to_string(),
                                     ))))
           .child(v_flex().w_full()
                          .min_w_0()
                          .px_4()
                          .py_3()
                          .gap(px(16.0))
                          .child(v_flex().gap(px(4.0))
                                         .child(div().flex()
                                                     .items_center()
                                                     .justify_between()
                                                     .child(section_heading(t!("download_sheet.files_heading").to_string(), &colors))
                                                     .child(all_none))
                                         .child(files))
                          .child(v_flex().gap(px(8.0))
                                         .child(section_heading(t!("download_sheet.destination_heading").to_string(), &colors))
                                         .child(roots))
                          .child(render_remember_row(&sheet, &colors, state))
                          .when_some(shortfall, |column, message| {
                              column.child(div().text_sm()
                                                .text_color(colors.error)
                                                .child(message))
                          }))
           .child(DialogFooter::new().px_4()
                                     .pb_4()
                                     .justify_between()
                                     .child(div().text_sm()
                                                 .text_color(colors.text_secondary)
                                                 .child(summary_label(&sheet)))
                                     .child(div().flex()
                                                 .gap(px(8.0))
                                                 .child(Button::new("download-sheet-cancel")
                                                            .label(t!("download_sheet.cancel").to_string())
                                                            .on_click(|_, window, cx| window.close_dialog(cx)))
                                                 .child(download)))
}
