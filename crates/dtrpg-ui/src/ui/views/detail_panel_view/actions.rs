use super::*;

/// Renders the row of buttons under the header: Read, Download, Materialize
/// where the file is a cloud placeholder, and a menu with the rest.
pub(super) fn render_action_row(item: &LibraryItem, storage_root_path: &Path,
                                controller: Entity<LibraryController>, cx: &App)
                                -> AnyElement {
    let is_downloaded = item.status == ItemStatus::Downloaded;
    let can_materialize = item.is_offloaded() && Platform.capabilities().materialize;

    div().flex()
         .items_center()
         .gap(px(8.0))
         .px(px(16.0))
         .pb(px(12.0))
         .child(read_button(item, is_downloaded, controller.clone()))
         .child(download_button(item, is_downloaded, controller.clone(), cx))
         .when(can_materialize, |row| {
             row.child(materialize_button(item, controller.clone()))
         })
         .child(more_menu(item, storage_root_path, is_downloaded, controller))
         .into_any_element()
}

/// Opens the item's file in the application set for it. Disabled until the
/// item is downloaded.
fn read_button(item: &LibraryItem, is_downloaded: bool, controller: Entity<LibraryController>)
               -> Button {
    let read_path = resolved_item_dir(&StorageConfig::load(), item);
    let read_item = item.clone();
    Button::new("inspector-read").primary()
                                 .icon(IconName::BookOpen)
                                 .tooltip(t!("detail.read_button"))
                                 .disabled(!is_downloaded)
                                 .when(!is_downloaded, |button| {
                                     button.tooltip(format!("{}\n{}",
                                                            t!("detail.read_button"),
                                                            t!("detail.tooltip_download_first")))
                                 })
                                 .when(is_downloaded, |button| {
                                     button.on_click(move |_, _, cx| {
                                               open_item_for_reading(&read_path,
                                                                     &read_item,
                                                                     &controller,
                                                                     cx)
                                           })
                                 })
}

/// Opens `item`'s file, logging what went wrong. An entry with several files
/// has no one file to open, so the inspector switches to its Files tab, where
/// the file is chosen.
fn open_item_for_reading(entry_dir: &Path, item: &LibraryItem,
                         controller: &Entity<LibraryController>, cx: &mut App) {
    use crate::util::item_opener::{ItemOpener, OpenError};

    match ItemOpener::open_item(entry_dir, &item.files) {
        Ok(()) => {}
        Err(OpenError::MultipleFilesRequireSelection) => {
            controller.update(cx, |ctrl, cx| {
                          ctrl.set_inspector_tab(Arc::clone(&item.id), InspectorTab::Files, cx)
                      });
        }
        Err(OpenError::FileNotFound(path)) => tracing::warn!("open: file not found: {path}"),
        Err(OpenError::NoDefaultApp) => {
            tracing::warn!("open: no default application configured");
        }
        Err(OpenError::OsFailed(message)) => tracing::warn!("open: OS failed: {message}"),
    }
}

/// Downloads the item, through the download sheet where one applies. Once the
/// item is downloaded the button only shows that; holding Shift while clicking
/// it asks to remove the download.
fn download_button(item: &LibraryItem, is_downloaded: bool,
                   controller: Entity<LibraryController>, cx: &App)
                   -> Button {
    let is_verifying = controller.read(cx).is_verifying_downloads(&item.id);
    let item_id = Arc::clone(&item.id);
    let title = item.title.to_string();
    let tooltip = if is_verifying {
        t!("detail.tooltip_verifying_download")
    }
    else if is_downloaded {
        t!("detail.downloaded_tooltip")
    }
    else {
        t!("detail.download_sheet_button")
    };
    Button::new("inspector-download").ghost()
                                     .outline()
                                     .disabled(is_verifying)
                                     .loading(is_verifying)
                                     .icon(if is_downloaded {
                                               IconName::CircleCheck
                                           }
                                           else {
                                               IconName::ArrowDown
                                           })
                                     .tooltip(tooltip)
                                     .on_click(move |event, window, cx| {
                                         if is_downloaded {
                                             if event.modifiers().shift {
                                                 confirm_remove_download(window,
                                                                         cx,
                                                                         controller.clone(),
                                                                         Arc::clone(&item_id),
                                                                         title.clone());
                                             }
                                         }
                                         else {
                                             request_download_with_sheet(window,
                                                                         cx,
                                                                         &controller,
                                                                         &item_id,
                                                                         title.clone());
                                         }
                                     })
}

/// Asks before removing an item's download.
fn confirm_remove_download(window: &mut Window, cx: &mut App,
                           controller: Entity<LibraryController>, item_id: Arc<str>,
                           title: String) {
    window.open_alert_dialog(cx, move |alert, _, _| {
              let controller = controller.clone();
              let item_id = Arc::clone(&item_id);
              alert.confirm()
                   .title(t!("catalog.remove_download_confirm_title",
                             title = title.clone()).to_string())
                   .description(t!("catalog.remove_download_confirm_description").to_string())
                   .on_ok(move |_, _window, cx| {
                       controller.update(cx, |ctrl, cx| ctrl.remove_download(&item_id, cx));
                       true
                   })
          });
}

/// Asks the system for the local copy of a file that was moved out to the
/// cloud.
fn materialize_button(item: &LibraryItem, controller: Entity<LibraryController>) -> Button {
    let item_id = Arc::clone(&item.id);
    Button::new("inspector-materialize").ghost()
                                        .outline()
                                        .icon(IconName::ArrowDown)
                                        .label(t!("detail.materialize_button").to_string())
                                        .tooltip(t!("detail.materialize_tooltip").to_string())
                                        .on_click(move |_, _, cx| {
                                            controller.update(cx, |ctrl, cx| {
                                                          ctrl.materialize_item(&item_id, cx);
                                                      });
                                        })
}

/// The menu at the end of the row: show the files in the file manager, remove
/// the download, and fetch the cover again.
fn more_menu(item: &LibraryItem, storage_root_path: &Path, is_downloaded: bool,
             controller: Entity<LibraryController>)
             -> impl IntoElement {
    let item_path = dtrpg_core::data::storage::publisher_dir(storage_root_path, &item.publisher);
    let has_cover = item.detail_cover_url.is_some() || item.cover_url.is_some();
    let item_id = Arc::clone(&item.id);
    let title = item.title.to_string();
    Button::new("inspector-more").ghost()
                                 .outline()
                                 .icon(IconName::EllipsisVertical)
                                 .tooltip(t!("detail.more_tooltip"))
                                 .dropdown_menu(move |menu, _, _| {
                                     let reveal_path = item_path.clone();
                                     let refresh_controller = controller.clone();
                                     let refresh_id = Arc::clone(&item_id);
                                     let remove_controller = controller.clone();
                                     let remove_id = Arc::clone(&item_id);
                                     let remove_title = title.clone();
                                     menu.item(PopupMenuItem::new(platform_reveal_label())
                                               .disabled(!is_downloaded)
                                               .on_click(move |_, _, _| reveal(&reveal_path)))
                                         .item(PopupMenuItem::new(t!("catalog.action_remove_download"))
                                               .disabled(!is_downloaded)
                                               .on_click(move |_, window, cx| {
                                                   confirm_remove_download(window,
                                                                           cx,
                                                                           remove_controller.clone(),
                                                                           Arc::clone(&remove_id),
                                                                           remove_title.clone());
                                               }))
                                         .item(PopupMenuItem::new(t!("detail.refresh_thumbnail_tooltip"))
                                               .disabled(!has_cover)
                                               .on_click(move |_, _, cx| {
                                                   refresh_controller.update(cx, |ctrl, cx| {
                                                       ctrl.refresh_detail_cover(Arc::clone(&refresh_id), cx);
                                                   });
                                               }))
                                 })
}

/// Shows `path` in the system's file manager.
fn reveal(path: &Path) {
    if !path.exists() {
        tracing::warn!(path = %path.display(), "reveal: file not found — item may need re-download");
        return;
    }
    if let Err(error) = reveal_in_file_manager(path) {
        tracing::warn!("reveal_in_file_manager failed: {error}");
    }
}
