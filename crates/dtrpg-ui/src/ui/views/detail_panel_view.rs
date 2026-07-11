//! Expanded detail tab content: full item metadata and actions, filling a
//! tab's content area (opened by double-clicking a catalog item).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, App, Entity, Image, InteractiveElement, IntoElement, ObjectFit, ParentElement,
    SharedString, StatefulInteractiveElement, Styled, StyledImage, div, img, px,
};
use gpui_component::Disableable;
use gpui_component::Icon;
use gpui_component::IconName;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::clipboard::Clipboard;
use gpui_component::collapsible::Collapsible;
use gpui_component::description_list::{DescriptionItem, DescriptionList};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::table::{Table, TableBody, TableCell, TableHeader, TableRow};
use gpui_component::text::Text;
use gpui_component::tooltip::Tooltip;
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::enums::ItemStatus;
use crate::data::library::{LibraryItem, LibraryItemFile};
use crate::data::theme::ColorTokens;
use crate::ui::library::cover::{cover_style, render_generative_cover};
use crate::ui::views::catalog_view::render_checking_indicator;
use crate::ui::views::manage_collections_dialog::open_manage_collections_dialog;
use crate::ui::widgets::selectable_text;
use crate::util::datetime::{format_absolute, format_relative};
use crate::util::matching::{collection_member_id, member_ids_contain};
use crate::util::reveal::reveal_in_file_manager;

/// Renders the expanded detail tab's content: a large cover, title,
/// description, actions, and metadata, filling the tab's content area.
///
/// Has no absolute positioning, resize handle, or close button of its own —
/// it's opened as a full tab (double-click on a catalog item, see
/// `main-window-tabs`) and closed via the tab strip.
///
/// For multi-item entries (`item.is_multi_item()`), renders a persistent
/// item list alongside the entry tier; selecting a row updates the item
/// metadata area in place (see `catalog-entry-detail-view`). Single-item
/// entries keep the existing inline entry-tier metadata table.
pub fn render_detail_tab_content(item: &LibraryItem, storage_root_path: PathBuf,
                                 entity: Entity<LibraryController>, colors: &ColorTokens,
                                 cover_image: Option<Arc<Image>>, cx: &App)
                                 -> AnyElement {
    let surface = colors.surface;
    let text_primary = colors.text_primary;
    let text_secondary = colors.text_secondary;

    let item = item.clone();
    let is_checking = entity.read(cx).is_checking(&item.id);
    let entity_download = entity.clone();
    let entity_refresh_thumbnail = entity.clone();
    let entity_other_details = entity.clone();
    let entity_collections = entity.clone();
    let entity_item_tier = entity;
    let item_id = Arc::clone(&item.id);
    let is_downloaded = item.status == ItemStatus::Downloaded;
    let download_title = item.title.to_string();

    let cover_w = crate::data::constants::DETAIL_PANEL_COVER_MAX_WIDTH * 1.5;
    let cover_h = cover_w * 10.0 / 7.0;
    let cover: AnyElement = if let Some(image) = cover_image {
        img(image).w(px(cover_w))
                  .h(px(cover_h))
                  .object_fit(ObjectFit::Cover)
                  .into_any_element()
    }
    else {
        render_generative_cover(&item, cover_w, cover_h, true).into_any_element()
    };
    let cover_url = item.cover_url.clone();

    div()
        .id("detail-tab-content")
        .flex_1()
        .min_h_0()
        .min_w_0()
        .flex()
        .bg(surface)
        .child({
            let mut cover_box =
                div().relative().w(px(cover_w)).flex_none().overflow_hidden().child(cover);
            if let Some(cover_url) = cover_url {
                cover_box = cover_box.child(
                    div()
                        .id("detail-tab-refresh-thumbnail")
                        .absolute()
                        .top(px(8.0))
                        .left(px(8.0))
                        .size(px(24.0))
                        .rounded_full()
                        .bg(colors.scrim)
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .text_sm()
                        .text_color(colors.accent_on)
                        .tooltip(|window, cx| {
                            Tooltip::new(t!("detail.refresh_thumbnail_tooltip").to_string())
                                .build(window, cx)
                        })
                        .on_click(move |_, _, cx| {
                            entity_refresh_thumbnail
                                .update(cx, |ctrl, cx| ctrl.load_thumbnail(cover_url.clone(), cx));
                        })
                        .child("\u{27f3}"),
                );
            }
            div().flex_none().pr(px(16.0)).py(px(16.0)).child(cover_box)
        })
        .child(
            div().flex_1().min_h_0().flex().flex_col().child(
                div()
                    .overflow_y_scrollbar()
                    .p(px(20.0))
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                selectable_text("detail-tab-publisher", item.publisher.to_string())
                                    .text_xs()
                                    .text_color(colors.text_tertiary),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        selectable_text("detail-tab-title", item.title.to_string())
                                            .text_xl()
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_color(text_primary),
                                    )
                                    .child(render_status_icon(is_downloaded, text_secondary))
                                    .children(render_checking_indicator(is_checking)),
                            )
                            .child(
                                selectable_text("detail-tab-line", item.line.to_string())
                                    .text_sm()
                                    .text_color(text_secondary),
                            ),
                    )
                    .child(
                        selectable_text("detail-tab-desc", item.desc.to_string())
                            .text_sm()
                            .text_color(text_secondary),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child({
                                let read_path =
                                    crate::data::storage::publisher_dir(&storage_root_path,
                                                                        &item.publisher);
                                Button::new("detail-tab-read")
                                    .primary()
                                    .icon(IconName::BookOpen)
                                    .disabled(!is_downloaded)
                                    .when(!is_downloaded, |b| {
                                        b.tooltip(format!("{}\n{}",
                                            t!("detail.read_button"),
                                            t!("detail.tooltip_download_first")))
                                    })
                                    .when(is_downloaded, |b| {
                                        let read_item = item.clone();
                                        b.tooltip(t!("detail.read_button")).on_click(
                                            move |_, _, _| {
                                                use crate::util::item_opener::{
                                                    ItemOpener, OpenError,
                                                };

                                                match ItemOpener::open_item(&read_path,
                                                                            &read_item.files)
                                                {
                                                    Ok(()) => {}
                                                    // The item list is already rendered in this
                                                    // same tab (see `render_item_tier`) — no
                                                    // further navigation needed, see
                                                    // `catalog-entry-detail-view`.
                                                    Err(OpenError::MultipleFilesRequireSelection) => {}
                                                    Err(OpenError::FileNotFound(path)) => {
                                                        tracing::warn!(
                                                            "open: file not found: {path}"
                                                        );
                                                    }
                                                    Err(OpenError::NoDefaultApp) => {
                                                        tracing::warn!(
                                                            "open: no default application configured"
                                                        );
                                                    }
                                                    Err(OpenError::OsFailed(msg)) => {
                                                        tracing::warn!("open: OS failed: {msg}");
                                                    }
                                                }
                                            },
                                        )
                                    })
                            })
                            .child(
                                Button::new("detail-tab-download")
                                    .ghost()
                                    .outline()
                                    .icon(if is_downloaded {
                                        IconName::CircleCheck
                                    } else {
                                        IconName::ArrowDown
                                    })
                                    .tooltip(if is_downloaded {
                                        t!("detail.downloaded_button")
                                    } else {
                                        t!("detail.download_button")
                                    })
                                    .on_click(move |_, _, cx| {
                                        let id = Arc::clone(&item_id);
                                        entity_download.update(cx, |ctrl, cx| {
                                            if is_downloaded {
                                                ctrl.remove_download(&id, cx);
                                            }
                                            else {
                                                ctrl.enqueue_download(&id, download_title.clone(), cx);
                                            }
                                        });
                                    }),
                            )
                            .when(is_downloaded, |row| {
                                let item_path =
                                    crate::data::storage::publisher_dir(&storage_root_path,
                                                                        &item.publisher);
                                row.child(
                                    Button::new("detail-tab-reveal")
                                        .ghost()
                                        .outline()
                                        .icon(IconName::FolderOpen)
                                        .tooltip(platform_reveal_label().into_owned())
                                        .on_click(move |_, _, _cx| {
                                            if !item_path.exists() {
                                                tracing::warn!(
                                                    path = %item_path.display(),
                                                    "reveal: file not found — item may need re-download"
                                                );
                                                return;
                                            }
                                            if let Err(e) = reveal_in_file_manager(&item_path) {
                                                tracing::warn!(
                                                    "reveal_in_file_manager failed: {e}"
                                                );
                                            }
                                        }),
                                )
                            }),
                    )
                    .child(render_metadata_table(&item, colors))
                    .when(item.is_multi_item(), |this| {
                        this.child(render_item_tier(&item, &storage_root_path,
                                                    entity_item_tier.clone(), colors, cx))
                    })
                    .child(render_collections_section(&item, entity_collections, colors, cx))
                    .child(render_other_details(&item, entity_other_details, colors, cx)),
            ),
        )
        .into_any_element()
}

/// Renders the item tier for a multi-item entry: a persistent, selectable
/// item list and an item metadata area that updates in place on selection.
///
/// See `catalog-entry-detail-view`'s persistent-item-list and
/// update-in-place requirements.
fn render_item_tier(item: &LibraryItem, storage_root_path: &Path,
                    entity: Entity<LibraryController>, colors: &ColorTokens, cx: &App)
                    -> impl IntoElement + 'static {
    let entry_id = Arc::clone(&item.id);
    let selected_ix = entity.read(cx).selected_item_file(&entry_id);

    let mut header_row = TableRow::new().child(
        TableCell::new().child(
            div().text_xs()
                 .font_weight(gpui::FontWeight::SEMIBOLD)
                 .text_color(colors.text_secondary)
                 .child(t!("detail.item_list_column_name").to_string()),
        ),
    );
    header_row = header_row.child(
        TableCell::new().child(
            div().text_xs()
                 .font_weight(gpui::FontWeight::SEMIBOLD)
                 .text_color(colors.text_secondary)
                 .child(t!("detail.item_list_column_type").to_string()),
        ),
    );
    header_row = header_row.child(
        TableCell::new().child(
            div().text_xs()
                 .font_weight(gpui::FontWeight::SEMIBOLD)
                 .text_color(colors.text_secondary)
                 .child(t!("detail.item_list_column_status").to_string()),
        ),
    );

    let mut body = TableBody::new();
    for (row_ix, file) in item.files.iter().enumerate() {
        // Selection is keyed by row position, not `file.id` — the API has
        // been observed to reuse the same download id across genuinely
        // distinct files within a bundle (see `LibraryItem::dedupe_files`),
        // so comparing by id would select/deselect multiple unrelated rows
        // together and make it impossible to pick between them.
        let is_selected = selected_ix == Some(row_ix);

        // `TableRow` itself has no click hook, so the whole row's clickable
        // area is a single wrapping div inside one `col_span(2)` `TableCell`
        // (mirroring the two-column header width via its own internal flex
        // split) rather than one on_click per column — attaching separate
        // click listeners to adjacent sibling cells of the same logical row
        // left later rows unresponsive after the first selection, since each
        // sibling tracks its own independent mouse-down/mouse-up state (see
        // this crate's `gpui-component` usage policy for why `Table` stays
        // the base component here instead of a hand-rolled flex layout).
        let entity_row = entity.clone();
        let entry_id_row = Arc::clone(&entry_id);
        // Includes `row_ix` (not just `file.id`) so that rows stay uniquely
        // identifiable — and therefore individually clickable — even if a
        // stale catalog cache (written before file records were deduplicated
        // by download id, see `map_order_product`) still has two rows
        // sharing an id; GPUI needs distinct element ids to hit-test each
        // row separately.
        let row_id = SharedString::from(format!("item-row-{row_ix}-{}", file.id));

        let row_content =
            div().id(row_id)
                 .flex()
                 .w_full()
                 .cursor_pointer()
                 .hover(|d| d.bg(colors.hover))
                 .on_click(move |_: &gpui::ClickEvent, _: &mut gpui::Window, cx: &mut App| {
                               entity_row.update(cx, |ctrl, cx| {
                                             ctrl.select_item_file(Arc::clone(&entry_id_row),
                                                                   row_ix,
                                                                   cx);
                                         });
                           })
                 // File name
                 .child(div().flex_1().child(
                     selectable_text(SharedString::from(format!("item-row-{row_ix}-name")),
                                     file.name.to_string())
                         .text_sm()
                         .text_color(colors.text_primary),
                 ))
                 // File type
                 .child(div().flex_1().child(
                     selectable_text(SharedString::from(format!("item-row-{row_ix}-format")),
                                     file.format.to_string())
                         .text_sm()
                         .text_color(colors.text_secondary),
                 ))
                 // Status
                 .child(div().flex_1()
                             .text_sm()
                             .text_color(colors.text_secondary)
                             .child(t!("detail.item_status_").to_string() /* TODO: replace
                                                                           * with cloud/
                                                                           * downloaded icon
                                                                           * and tooltip */));

        let row = TableRow::new().when(is_selected, |row| row.bg(colors.accent_soft))
                                 .child(TableCell::new().col_span(2).child(row_content));

        body = body.child(row);
    }

    let item_list = Table::new().child(TableHeader::new().child(header_row))
                                .child(body);

    let selected_file = selected_ix.and_then(|ix| item.files.get(ix).map(|file| (ix, file)));

    div().flex()
         .flex_col()
         .gap(px(12.0))
         .child(div().text_sm()
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .text_color(colors.text_primary)
                     .child(t!("detail.items_heading").to_string()))
         .child(div().text_sm()
                     .text_color(colors.text_tertiary)
                     // .py(px(12.0))
                     .child(t!("detail.item_prompt_select").to_string())
                     .into_any_element())
         .child(item_list)
         .child(match selected_file {
                    Some((ix, file)) => render_item_metadata(item,
                                                             file,
                                                             ix,
                                                             storage_root_path,
                                                             entity.clone(),
                                                             colors,
                                                             cx).into_any_element(),
                    None => div().into_any_element(),
                })
}

/// Renders the metadata area for a single selected item within a multi-item
/// entry: name, type, format, file size, the entry's download state
/// (individual files share the entry's on-disk download state today; see
/// `define-rust-catalog-entry-detail-view`'s open questions for future
/// per-file tracking), and a per-file "Other details" disclosure (file id,
/// download location).
///
/// `row_ix` is the file's position within the entry's `files` list — used to
/// key element ids and disclosure state, since the API has been observed to
/// reuse the same download id across genuinely distinct files within a
/// bundle (see `LibraryItem::dedupe_files`).
fn render_item_metadata(item: &LibraryItem, file: &LibraryItemFile, row_ix: usize,
                        storage_root_path: &Path, entity: Entity<LibraryController>,
                        colors: &ColorTokens, cx: &App)
                        -> impl IntoElement + 'static {
    let name_value = copyable_value(SharedString::from(format!("file-name-{row_ix}")),
                                    file.name.to_string());

    let metadata = DescriptionList::vertical()
        .columns(2)
        .bordered(false)
        .child(DescriptionItem::new(t!("detail.item_list_column_name").to_string())
                   .value(name_value)
                   .span(2))
        .child(DescriptionItem::new(t!("detail.field_format").to_string())
                   .value(file.format.to_string()))
        .child(DescriptionItem::new(t!("detail.field_file_size").to_string())
                   .value(format!("{:.1} MB", file.size_mb)))
        // .child(DescriptionItem::new(t!("detail.field_status").to_string())
        //            .value(if item.status == ItemStatus::Downloaded {
        //                t!("detail.status_on_device").to_string()
        //            } else {
        //                t!("detail.status_in_cloud").to_string()
        //            })
        //            .span(2))
    ;

    div().flex()
         .flex_col()
         .gap(px(12.0))
         .child(metadata)
         .child(render_file_other_details(FileOtherDetailsContext { entry_id: &item.id,
                                                                    publisher: &item.publisher,
                                                                    row_ix,
                                                                    is_downloaded:
                                                                        item.status
                                                                        == ItemStatus::Downloaded,
                                                                    storage_root_path },
                                          file,
                                          entity,
                                          colors,
                                          cx))
}

/// Builds the shared toggle-state key for a single file's "Other details"
/// disclosure within a multi-item entry's item tier.
fn file_other_details_key(entry_id: &str, row_ix: usize) -> Arc<str> {
    Arc::from(format!("{entry_id}:{row_ix}"))
}

/// Grouped context for [`render_file_other_details`], kept below Rust's
/// preferred argument count.
struct FileOtherDetailsContext<'a> {
    entry_id:          &'a str,
    publisher:         &'a str,
    row_ix:            usize,
    is_downloaded:     bool,
    storage_root_path: &'a Path,
}

/// Renders a single file's "Other details" disclosure section: a
/// clickable header that toggles a collapsed-by-default panel showing the
/// file's id and its on-disk download location.
///
/// The download location is the entry's shared item folder — per-file paths
/// aren't tracked separately (see `LibraryItemFile`), so every file in a
/// bundle reports the same location.
fn render_file_other_details(ctx: FileOtherDetailsContext<'_>, file: &LibraryItemFile,
                             entity: Entity<LibraryController>, colors: &ColorTokens, cx: &App)
                             -> impl IntoElement + 'static {
    let FileOtherDetailsContext { entry_id,
                                  publisher,
                                  row_ix,
                                  is_downloaded,
                                  storage_root_path, } = ctx;
    let toggle_key = file_other_details_key(entry_id, row_ix);
    let open = entity.read(cx).is_file_other_details_open(&toggle_key);

    let toggle_entity = entity;
    let toggle_key_for_click = Arc::clone(&toggle_key);

    let header = div().id(SharedString::from(format!("file-other-details-header-{toggle_key}")))
                      .flex()
                      .items_center()
                      .gap(px(6.0))
                      .cursor_pointer()
                      .text_sm()
                      .font_weight(gpui::FontWeight::SEMIBOLD)
                      .text_color(colors.text_primary)
                      .on_click(move |_, _, cx| {
                          toggle_entity.update(cx, |ctrl, cx| {
                              ctrl.toggle_file_other_details(Arc::clone(&toggle_key_for_click), cx);
                          });
                      })
                      .child(Icon::new(if open {
                                           IconName::ChevronUp
                                       }
                                       else {
                                           IconName::ChevronDown
                                       }).text_color(colors.text_secondary))
                      .child(t!("detail.other_details_heading").to_string());

    let path_value = if is_downloaded {
        crate::data::storage::publisher_dir(storage_root_path, publisher).display()
                                                                         .to_string()
    }
    else {
        value_or_dash("")
    };

    let content = DescriptionList::vertical()
        .columns(2)
        .bordered(false)
        .child(DescriptionItem::new(t!("detail.field_file_id").to_string())
                   .value(copyable_value(SharedString::from(format!("file-id-{toggle_key}")),
                                         file.id.to_string()))
                   .span(2))
        .child(DescriptionItem::new(t!("detail.field_download_location").to_string())
                   .value(copyable_value(SharedString::from(format!("file-path-{toggle_key}")),
                                         path_value))
                   .span(2));

    Collapsible::new().gap(px(8.0))
                      .open(open)
                      .child(header)
                      .content(content)
}

/// Renders the "Other details" disclosure section: a clickable header that
/// toggles a collapsed-by-default panel of fields not already shown in the
/// primary metadata table or item tier (stable id, numeric id, order product
/// id, product id, added-order value, and the generative cover color).
///
/// See `catalog-entry-detail-advanced-disclosure`.
fn render_other_details(item: &LibraryItem, entity: Entity<LibraryController>,
                        colors: &ColorTokens, cx: &App)
                        -> impl IntoElement + 'static {
    let entry_id = Arc::clone(&item.id);
    let open = entity.read(cx).is_other_details_open(&entry_id);

    let toggle_entity = entity;
    let toggle_entry_id = Arc::clone(&entry_id);

    let header = div().id("other-details-header")
                      .flex()
                      .items_center()
                      .gap(px(6.0))
                      .cursor_pointer()
                      .text_sm()
                      .font_weight(gpui::FontWeight::SEMIBOLD)
                      .text_color(colors.text_primary)
                      .on_click(move |_, _, cx| {
                          toggle_entity.update(cx, |ctrl, cx| {
                                           ctrl.toggle_other_details(Arc::clone(&toggle_entry_id),
                                                                     cx);
                                       });
                      })
                      .child(Icon::new(if open {
                                           IconName::ChevronUp
                                       }
                                       else {
                                           IconName::ChevronDown
                                       }).text_color(colors.text_secondary))
                      .child(t!("detail.other_details_heading").to_string());

    let swatch_color = cover_style(item).background;
    let content = DescriptionList::vertical()
        .columns(2)
        .bordered(false)
        .child(DescriptionItem::new(t!("detail.field_stable_id").to_string())
                   .value(copyable_value(SharedString::from("other-details-stable-id"),
                                         item.id.to_string())))
        .child(DescriptionItem::new(t!("detail.field_numeric_id").to_string())
                   .value(copyable_value(SharedString::from("other-details-numeric-id"),
                                         item.numeric_id.to_string())))
        .child(DescriptionItem::new(t!("detail.field_order_product_id").to_string())
                   .value(copyable_value(SharedString::from("other-details-order-product-id"),
                                         item.order_product_id.to_string())))
        .child(DescriptionItem::new(t!("detail.field_product_id").to_string())
                   .value(copyable_value(SharedString::from("other-details-product-id"),
                                         item.product_id.to_string())))
        .child(DescriptionItem::new(t!("detail.field_added_order").to_string())
                   .value(item.added_order.to_string()))
        .child(DescriptionItem::new(t!("detail.field_cover_color").to_string())
                   .value(
                       div().flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(div().size(px(12.0)).rounded_full().bg(swatch_color))
                            .child(copyable_value(SharedString::from("other-details-cover-color"),
                                                  item.color.to_string()))
                            .into_any_element(),
                   )
                   .span(2));

    Collapsible::new().gap(px(8.0))
                      .open(open)
                      .child(header)
                      .content(content)
}

/// Renders a summary of the collections this entry currently belongs to, with a
/// "Manage…" button opening the Manage Collections dialog scoped to this entry.
///
/// See `detail-view-collection-membership`.
fn render_collections_section(item: &LibraryItem, entity: Entity<LibraryController>,
                              colors: &ColorTokens, cx: &App)
                              -> impl IntoElement + 'static {
    let member_id = collection_member_id(item);
    let product_id = item.product_id;
    let member_names: Vec<Arc<str>> =
        entity.read(cx)
              .collections
              .iter()
              .filter(|c| member_ids_contain(&c.member_ids, member_id, product_id))
              .map(|c| Arc::clone(&c.name))
              .collect();

    let summary: AnyElement = if member_names.is_empty() {
        div().text_sm()
             .text_color(colors.text_tertiary)
             .child(t!("detail.collections_empty").to_string())
             .into_any_element()
    }
    else {
        selectable_text("detail-tab-collections-summary",
                        member_names.iter()
                                    .map(|n| n.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")).text_sm()
                                                .text_color(colors.text_secondary)
                                                .into_any_element()
    };

    let item_title = Arc::clone(&item.title);

    div().flex()
        .flex_col()
        .gap(px(6.0))
        .child(div().text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(colors.text_primary)
                    .child(t!("detail.collections_heading").to_string()))
        .child(
            div().flex()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .child(summary)
                .child(Button::new("detail-tab-manage-collections").ghost().outline().label(
            t!("detail.collections_manage_button").to_string(),
        ).on_click(move |_, window, cx| {
                       open_manage_collections_dialog(window, cx, entity.clone(),
                                                      Arc::clone(&item_title), member_id,
                                                      product_id);
                   })),
        )
}

/// Renders a text value with an appear-on-hover copy button.
///
/// `field_id` must be unique within the surrounding view — it doubles as the
/// hover group name and the copy button's element id.
fn copyable_value(field_id: SharedString, value: impl Into<SharedString>) -> AnyElement {
    let value: SharedString = value.into();

    div().id(SharedString::from(format!("{field_id}-row")))
         .group(field_id.clone())
         .flex()
         .items_center()
         .gap(px(6.0))
         .child(selectable_text(SharedString::from(format!("{field_id}-text")), value.clone()))
         .child(
             div()
                 .invisible()
                 .group_hover(field_id.clone(), |d| d.visible())
                 .child(
                     Clipboard::new(SharedString::from(format!("{field_id}-copy")))
                         .value(value)
                         .tooltip(t!("detail.copy_tooltip").to_string()),
                 ),
         )
         .into_any_element()
}

fn platform_reveal_label() -> std::borrow::Cow<'static, str> {
    #[cfg(target_os = "macos")]
    return t!("detail.show_in_finder");
    #[cfg(target_os = "windows")]
    return t!("detail.show_in_explorer");
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return t!("detail.show_in_files");
}

/// Renders a small status icon next to the item title: a checkmark when
/// downloaded, a cloud glyph otherwise. Replaces the old text-only "Status" row
/// in the metadata table.
fn render_status_icon(is_downloaded: bool, color: gpui::Hsla) -> impl IntoElement + 'static {
    let (glyph, tooltip_text) = if is_downloaded {
        ("\u{2713}", t!("detail.status_on_device").to_string())
    }
    else {
        ("\u{2601}", t!("detail.status_in_cloud").to_string())
    };

    div().id("detail-status-icon")
         .text_sm()
         .text_color(color)
         .tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))
         .child(glyph)
}

/// Renders a metadata value, falling back to an em dash when the underlying
/// data is empty (e.g. the API did not report a game system/line for this
/// item).
fn value_or_dash(value: &str) -> String {
    if value.trim().is_empty() {
        "\u{2014}".to_string()
    }
    else {
        value.to_string()
    }
}

fn render_metadata_table(item: &LibraryItem, colors: &ColorTokens)
                         -> impl IntoElement + 'static + use<> {
    let category_label = div().flex()
                              .items_center()
                              .gap(px(4.0))
                              .child(Icon::new(IconName::Folder).text_color(colors.text_secondary))
                              .child(t!("detail.field_category").to_string())
                              .into_any_element();

    let mut list = DescriptionList::vertical()
        .columns(2)
        .bordered(false)
        .child(
            DescriptionItem::new(t!("detail.field_system").to_string())
                .value(Text::from(selectable_text("detail-field-system",
                                                  value_or_dash(&item.line)))),
        )
        .child(
            DescriptionItem::new(t!("detail.field_released").to_string())
                .value(Text::from(selectable_text("detail-field-released",
                                                  item.year.to_string()))),
        )
        .child(
            DescriptionItem::new(t!("detail.field_format").to_string())
                .value(Text::from(selectable_text("detail-field-format",
                                                  item.format.to_string()))),
        )
        .child(
            DescriptionItem::new(t!("detail.field_file_size").to_string())
                .value(Text::from(selectable_text("detail-field-file-size",
                                                  format!("{:.0} MB", item.size_mb)))),
        )
        .child(DescriptionItem::new(category_label)
                   .value(Text::from(selectable_text("detail-field-kind", item.kind.to_string())))
                   .span(2));

    // The DriveThruRPG order-product API does not always report a page count; omit
    // the row entirely rather than showing a misleading "0".
    if item.pages > 0 {
        list = list.child(
            DescriptionItem::new(t!("detail.field_pages").to_string())
                .value(Text::from(selectable_text("detail-field-pages", item.pages.to_string())))
                .span(2),
        );
    }

    if let Some(ts) = item.date_added {
        let value = render_relative_date_value(&item.id, "added", ts);
        list = list.child(DescriptionItem::new(t!("detail.field_added").to_string()).value(value)
                                                                                    .span(2));
    }

    if let Some(ts) = item.date_updated {
        let value = render_relative_date_value(&item.id, "updated", ts);
        list =
            list.child(DescriptionItem::new(t!("detail.field_updated").to_string()).value(value)
                                                                                   .span(2));
    }

    list
}

/// Renders a stateful div showing a relative date label, with a tooltip that
/// reveals the absolute human-readable date/time on hover.
///
/// `slot` disambiguates the element id (e.g. `"added"`, `"updated"`) so
/// multiple date rows on the same item don't collide.
fn render_relative_date_value(item_id: &str, slot: &str, ts: i64) -> AnyElement {
    let relative = format_relative(ts);
    let absolute = format_absolute(ts);
    let id = SharedString::from(format!("detail-{slot}-{item_id}"));
    div().id(id)
         .child(selectable_text(SharedString::from(format!("detail-{slot}-{item_id}-text")),
                                relative))
         .tooltip(move |window, cx| Tooltip::new(absolute.clone()).build(window, cx))
         .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_or_dash_passes_through_non_empty_value() {
        assert_eq!(value_or_dash("Pathfinder"), "Pathfinder");
    }

    #[test]
    fn value_or_dash_falls_back_to_em_dash_on_empty_string() {
        assert_eq!(value_or_dash(""), "\u{2014}");
    }

    #[test]
    fn value_or_dash_falls_back_to_em_dash_on_whitespace_only() {
        assert_eq!(value_or_dash("   "), "\u{2014}");
    }
}
