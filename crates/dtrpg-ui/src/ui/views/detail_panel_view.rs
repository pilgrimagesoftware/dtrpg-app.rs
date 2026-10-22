//! The inspector: what the app shows about one catalog item, in a single
//! column — a header with the cover and title, the action buttons, a row of
//! tabs, and the selected tab's content.
//!
//! It appears in the pane docked beside the catalog and, with the same layout,
//! in the window an item can be popped out into.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use dtrpg_core::data::enums::ItemStatus;
use dtrpg_core::data::file_openers::FileOpenerEntry;
use dtrpg_core::data::library::{LibraryItem, LibraryItemFile};
use dtrpg_core::data::storage::StorageConfig;
use dtrpg_core::platform::{Platform, PlatformIntegration, PlatformKind};
use dtrpg_core::util::matching::{collection_member_id, member_ids_contain};
use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, App, Context, Entity, Image, InteractiveElement, IntoElement, ObjectFit,
    ParentElement, Pixels, SharedString, StatefulInteractiveElement, Styled, StyledImage, Window,
    div, img, px,
};
use gpui_kit::component::Disableable;
use gpui_kit::component::ElementExt as _;
use gpui_kit::component::IconName;
use gpui_kit::component::Sizable;
use gpui_kit::component::Size;
use gpui_kit::component::WindowExt as _;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::clipboard::Clipboard;
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::spinner::Spinner;
use gpui_kit::component::table::{Column, DataTable, TableDelegate, TableState};
use gpui_kit::component::tooltip::Tooltip;
use rust_i18n::t;

use crate::controllers::inspector::{InspectorController, InspectorPlacement, item_list_table};
use crate::controllers::library::LibraryController;
use crate::controllers::settings::{FILE_OPENERS_PAGE_IX, SettingsController};
use crate::data::theme::{ColorTokens, LibriTheme};
use crate::ui::actions::ShowSettings;
use crate::ui::library::cover::{
    CoverCache, cover_style, detail_cache_key, render_generative_cover,
};
use crate::ui::views::catalog_view::render_checking_indicator;
use crate::ui::views::download_sheet_dialog::request_download_with_sheet;
use crate::ui::views::manage_collections_dialog::open_manage_collections_dialog;
use crate::ui::views::tags_section_view::render_tags_section;
use crate::ui::widgets::{selectable_text, small_caps_text};
use crate::util::datetime::{format_absolute, format_relative};
use crate::util::file_presence::resolved_item_dir;
use crate::util::reveal::reveal_in_file_manager;
use crate::view_models::inspector::{
    InspectorContentsState, InspectorPreviewState, InspectorSummaryState, InspectorTab,
    InspectorViewModel,
};

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

mod actions;
mod contents_tab;
mod cover;
mod header;
mod info_tab;

mod inspector;
mod item_list_delegate;
mod item_tier;
mod metadata_table;
mod other_details;
mod sections;

use actions::*;
use contents_tab::*;
use cover::*;
use header::*;
use info_tab::*;
use inspector::*;
pub(crate) use item_list_delegate::*;
use item_tier::*;
use metadata_table::*;
use other_details::*;
use sections::*;
