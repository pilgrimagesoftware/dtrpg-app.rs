//! The inspector: what the app shows about one catalog item, in a single
//! column — a header with the cover and title, the action buttons, a row of
//! tabs, and the selected tab's content.
//!
//! It appears in the pane docked beside the catalog and, with the same layout,
//! in the window an item can be popped out into.

use std::path::PathBuf;
use std::sync::Arc;

use dtrpg_core::data::file_openers::FileOpenerEntry;
use dtrpg_core::data::library::LibraryItem;
use gpui::{
    AnyElement, App, Entity, Image, InteractiveElement, IntoElement, ParentElement, Styled, Window,
    div,
};
use gpui_kit::component::scroll::ScrollableElement as _;

use crate::controllers::inspector::InspectorController;
use crate::controllers::library::LibraryController;
use crate::controllers::settings::SettingsController;
use crate::data::theme::{ColorTokens, LibriTheme};
use crate::ui::library::cover::{CoverCache, detail_cache_key};
use crate::view_models::inspector::InspectorTab;

/// Everything an inspector surface hands over to draw one item.
pub struct InspectorContext<'a> {
    /// The item to show.
    pub item:              &'a LibraryItem,
    /// The folder downloads are stored in, for locating the item's files.
    pub storage_root_path: PathBuf,
    /// The configured per-extension file openers, for the Opens With section.
    pub file_openers:      Vec<FileOpenerEntry>,
    /// The settings, which the Opens With section links to.
    pub settings:          Entity<SettingsController>,
    /// The library the item belongs to.
    pub controller:        Entity<LibraryController>,
    /// The state of the surface the item is shown in.
    pub inspector:         &'a Entity<InspectorController>,
    /// The theme colors.
    pub colors:            &'a ColorTokens,
    /// The item's cover, once it has been fetched.
    pub cover_image:       Option<Arc<Image>>,
}

/// Renders the inspector for `item`, gathering what it needs from the app's
/// controllers and caches. This is what the docked pane and a popped-out
/// window call.
pub fn render_item_inspector(item: &LibraryItem, controller: &Entity<LibraryController>,
                             settings: &Entity<SettingsController>,
                             inspector: &Entity<InspectorController>, window: &mut Window,
                             cx: &mut App)
                             -> AnyElement {
    let colors = cx.global::<LibriTheme>().colors.clone();
    let covers = cx.global::<CoverCache>();
    let cover_image = covers.get(&detail_cache_key(&item.id))
                            .or_else(|| covers.get(&item.id));
    let settings_snapshot = settings.read(cx).snapshot();
    render_inspector(InspectorContext { item,
                                        storage_root_path: settings_snapshot.storage_root_path,
                                        file_openers: settings_snapshot.file_openers,
                                        settings: settings.clone(),
                                        controller: controller.clone(),
                                        inspector,
                                        colors: &colors,
                                        cover_image },
                     window,
                     cx)
}

/// Renders the inspector for one item: header, actions, tabs, and the active
/// tab's content, filling the space it is given.
///
/// For entries with several files, the Files tab holds the persistent file
/// list; selecting a row there updates the file's details in place (see
/// `catalog-entry-detail-view`).
pub fn render_inspector(context: InspectorContext<'_>, window: &mut Window, cx: &mut App)
                        -> AnyElement {
    let InspectorContext { item,
                           storage_root_path,
                           file_openers,
                           settings,
                           controller,
                           inspector,
                           colors,
                           cover_image, } = context;

    let tab = controller.read(cx).inspector_tab_for(&item.id);
    let body = if tab == InspectorTab::Overview {
        render_info_tab(item,
                        &storage_root_path,
                        &file_openers,
                        &settings,
                        controller.clone(),
                        colors,
                        cx)
    }
    else {
        render_secondary_inspector_tab(item,
                                       tab,
                                       InspectorRenderContext { storage_root_path:
                                                                    &storage_root_path,
                                                                entity: controller.clone(),
                                                                inspector,
                                                                colors,
                                                                window,
                                                                cx })
    };

    div().id("inspector")
         .flex_1()
         .min_h_0()
         .min_w_0()
         .flex()
         .flex_col()
         .bg(colors.surface)
         .child(render_inspector_header(item,
                                        cover_image,
                                        inspector,
                                        controller.clone(),
                                        colors,
                                        cx))
         .child(render_action_row(item, &storage_root_path, controller.clone(), cx))
         .child(render_inspector_tab_bar(item, controller, tab, colors))
         .child(div().id("inspector-body")
                     .flex_1()
                     .min_h_0()
                     .overflow_y_scrollbar()
                     .child(body))
         .into_any_element()
}

use super::actions::*;
use super::header::*;
use super::info_tab::*;
use super::inspector::*;
pub(crate) use super::item_list_delegate::*;
