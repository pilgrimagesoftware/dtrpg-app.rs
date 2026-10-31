//! The Locations section of the Storage page: one card per location.

use dtrpg_core::constants::ITEMS_SUB_PATH;
use dtrpg_core::data::storage::DEFAULT_ROOT_ID;
use dtrpg_core::data::storage::roots::StorageRoot;
use dtrpg_core::data::storage::roots::{RootKind, root_kind};
use dtrpg_core::data::storage::validate_writable;
use gpui::Entity;
use gpui::IntoElement;
use gpui::ParentElement;
use gpui::SharedString;
use gpui::Styled;
use gpui::div;
use gpui::prelude::*;
use gpui::px;
use gpui_kit::component::Icon;
use gpui_kit::component::IconName;
use gpui_kit::component::button::Button;
use gpui_kit::component::button::ButtonVariants;
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_kit::component::tag::Tag;
use gpui_kit::component::tooltip::Tooltip;
use rust_i18n::t;

use crate::controllers::settings::SettingsController;
use crate::data::theme::ColorTokens;
use crate::ui::views::settings::card::section_heading;
use crate::ui::views::settings::page::storage::platform_reveal_label;
use crate::util::reveal::reveal_in_file_manager;
use crate::view_models::storage_display::{StorageDisplay, free_space_label, titles_here_label};

/// Renders the Locations heading, a card for each location, and the button
/// that adds another.
pub(crate) fn render_storage_roots(roots: Vec<StorageRoot>, display: &StorageDisplay,
                                   entity: Entity<SettingsController>, colors: &ColorTokens)
                                   -> impl IntoElement + 'static + use<> {
    div().flex()
         .flex_col()
         .gap(px(10.0))
         .child(section_heading(t!("settings.storage_roots_title").to_string(), colors))
        .child(div().flex_1()
            .text_xs()
            .text_color(colors.text_secondary)
            .child(create_location_root_note_string())
        )
         .children(roots.into_iter()
                        .map(|root| render_location(&root, display, &entity, colors)))
         .child(div().flex().child(add_location_button(entity)))
}

/// Create a note telling the user that files will be stored in a subfolder of
/// the chosen location, so they aren't surprised to find downloaded files are not
/// in the root of the location.
fn create_location_root_note_string() -> String {
     t!("settings.storage_roots_note", folder = ITEMS_SUB_PATH).to_string()
}

/// One location: its name and kind, where it is, how many titles it holds, and
/// how full its volume is, with a menu of things to do with it.
fn render_location(root: &StorageRoot, display: &StorageDisplay,
                   entity: &Entity<SettingsController>, colors: &ColorTokens)
                   -> impl IntoElement + 'static + use<> {
    let root_id = root.id;
    let (free, total) = display.volumes.get(&root_id).copied().unwrap_or_default();
    let titles = titles_here_label(display.title_counts.get(&root_id).copied().unwrap_or(0));
    div().flex()
         .items_center()
         .gap(px(12.0))
         .px(px(16.0))
         .py(px(12.0))
         .rounded(px(8.0))
         .border_1()
         .border_color(colors.border)
         .child(Icon::empty().path("icons/hard-drive.svg")
                             .text_color(colors.text_secondary))
         .child(div().flex_1()
                     .min_w_0()
                     .flex()
                     .flex_col()
                     .gap(px(2.0))
                     .child(div().flex()
                                 .items_center()
                                 .gap(px(6.0))
                                 .child(div().text_sm()
                                             .font_weight(gpui::FontWeight::SEMIBOLD)
                                             .text_color(colors.text_primary)
                                             .child(root.name.clone()))
                                 .children(kind_badge(root_kind(&root.path))))
                     .child(div().text_xs()
                                 .font_family("monospace")
                                 .text_color(colors.text_secondary)
                                 .truncate()
                                 .child(create_path_string(root.path.to_string_lossy().into_owned())))
                     .child(div().text_xs()
                                 .text_color(colors.text_secondary)
                                 .child(titles)))
         .children(free_space_label(free, total).map(|label| {
                                                    div().flex_none()
                                                         .text_xs()
                                                         .text_color(colors.text_secondary)
                                                         .child(label)
                                                }))
         .child(location_menu(root, entity))
}

/// Build a path string that takes the path value and appends the "items"
/// path where files will ultimately end up.
fn create_path_string(path: String) -> String {
    format!("{} -> `{}`", path, ITEMS_SUB_PATH)
}

/// The ⋮ button of a location, and the menu it opens: show it in the file
/// manager, change its folder (the default location only, as the others can be
/// removed and added again), or remove it (every location but the default).
fn location_menu(root: &StorageRoot, entity: &Entity<SettingsController>) -> impl IntoElement {
    let root_id = root.id;
    let path = root.path.clone();
    let entity = entity.clone();
    Button::new(SharedString::from(format!("location-menu-{root_id}")))
        .ghost()
        .icon(Icon::empty().path("icons/ellipsis-vertical.svg"))
        .tooltip(t!("settings.storage_location_menu_tooltip").to_string())
        .dropdown_menu(move |menu, _, _| {
            let reveal_path = path.clone();
            let change_entity = entity.clone();
            let remove_entity = entity.clone();
            let menu = menu.item(PopupMenuItem::new(platform_reveal_label().into_owned())
                                     .on_click(move |_, _, _| reveal(&reveal_path)));
            if root_id == DEFAULT_ROOT_ID {
                menu.item(PopupMenuItem::new(t!("settings.storage_change_folder").to_string())
                              .on_click(move |_, _, cx| {
                                  change_default_folder(change_entity.clone(), cx);
                              }))
            }
            else {
                menu.item(PopupMenuItem::new(t!("settings.storage_remove_location").to_string())
                              .on_click(move |_, _, cx| {
                                  remove_entity.update(cx, |ctrl, cx| {
                                                   ctrl.remove_storage_root(root_id, cx);
                                               });
                              }))
            }
        })
}

/// Shows `path` in the platform's file manager, logging if it is not there.
fn reveal(path: &std::path::Path) {
    if !path.exists() {
        tracing::warn!(path = %path.display(), "reveal: location not found");
        return;
    }
    if let Err(error) = reveal_in_file_manager(path) {
        tracing::warn!(%error, "reveal_in_file_manager failed");
    }
}

/// Asks for a folder and makes it the default location.
fn change_default_folder(entity: Entity<SettingsController>, cx: &mut gpui::App) {
    cx.spawn(async move |cx| {
          // The picker is asynchronous so it does not hold gpui's `App`
          // borrowed while a native modal loop runs; see issue #257.
          let Some(handle) = rfd::AsyncFileDialog::new().pick_folder().await
          else {
              return;
          };
          let path = handle.path().to_path_buf();
          if let Err(error) = validate_writable(&path) {
              tracing::warn!(%error, "storage path not writable");
              return;
          }
          entity.update(cx, |ctrl, cx| {
                    if let Err(error) = ctrl.apply_storage_path(path, cx) {
                        tracing::warn!(%error, "storage path rejected");
                    }
                });
      })
      .detach();
}

/// The "Add location…" button: asks for a folder and adds it, named after it.
fn add_location_button(entity: Entity<SettingsController>) -> impl IntoElement {
    Button::new("add-storage-root").outline()
                                   .icon(IconName::Plus)
                                   .label(t!("settings.storage_add_location").to_string())
                                   .on_click(move |_, _, cx| {
                                       let entity = entity.clone();
                                       cx.spawn(async move |cx| {
                                             let Some(handle) =
                                                 rfd::AsyncFileDialog::new().pick_folder().await
                                             else {
                                                 return;
                                             };
                                             let path = handle.path().to_path_buf();
                                             let name = path.file_name()
                                                            .and_then(|name| name.to_str())
                                                            .filter(|name| !name.is_empty())
                                                            .unwrap_or("Storage")
                                                            .to_owned();
                                             if let Err(error) = entity.update(cx, |ctrl, cx| {
                                                                     ctrl.add_storage_root(name,
                                                                                           path, cx)
                                                                 })
                                             {
                                                 tracing::warn!(%error, "storage root rejected");
                                             }
                                         })
                                         .detach();
                                   })
}

/// The EXTERNAL or CLOUD marker for a location, or nothing for a local one.
fn kind_badge(kind: RootKind) -> Option<impl IntoElement> {
    let (label, tooltip) = match kind {
        RootKind::Local => return None,
        RootKind::External => {
            (t!("settings.storage_badge_external"), t!("settings.storage_badge_external_tooltip"))
        }
        RootKind::Cloud => {
            (t!("settings.storage_badge_cloud"), t!("settings.storage_badge_cloud_tooltip"))
        }
    };
    let tooltip = tooltip.to_string();
    Some(div().id(SharedString::from(format!("root-kind-{label}")))
              .tooltip(move |window, cx| Tooltip::new(tooltip.clone()).build(window, cx))
              .child(Tag::secondary().outline().child(label.to_string())))
}
