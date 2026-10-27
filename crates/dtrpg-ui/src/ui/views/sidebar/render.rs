//! Sidebar layout rendering.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

use dtrpg_core::data::collection::entry::CollectionEntry;
use dtrpg_core::data::library::SectionCounts;
use dtrpg_core::data::tag_library::TagEntry;
use dtrpg_core::util::filter::SidebarFilter;
use dtrpg_core::util::matching::name_matches_query;
use dtrpg_core::util::publisher::PublisherEntry;
use dtrpg_core::util::sort::{CollectionSortMethod, SortDirection};
use gpui::prelude::*;
use gpui::{Entity, IntoElement, SharedString, div, px};
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::dialog::{Dialog, DialogButtonProps, DialogHeader, DialogTitle};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::sidebar::{Sidebar, SidebarCollapsible, SidebarMenu, SidebarMenuItem};
use gpui_kit::component::spinner::Spinner;
use gpui_kit::component::{Icon, IconName};
use gpui_kit::component::{Side, Sizable as _, Size};
use rust_i18n::t;

use super::helpers::{nav_item, render_collection_sort_control, render_section_search_suffix};
use super::section::ControlledSection;
use super::view::*;
use crate::controllers::library::{LibraryController, SidebarOpen, SidebarSection};
use crate::ui::views::sidebar::smart::{SmartCollectionRow, smart_collections_menu};
use crate::ui::views::sidebar::tags::tag_rows;

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_sidebar(filter: SidebarFilter, counts: SectionCounts,
                             publishers: Vec<PublisherEntry>, collections: Vec<CollectionEntry>,
                             collections_loaded: bool, catalog_loading: bool,
                             catalog_ids: Arc<HashSet<u64>>,
                             collection_sort: CollectionSortMethod,
                             collection_sort_direction: SortDirection,
                             entity: Entity<LibraryController>,
                             collection_name_input: Entity<InputState>,
                             publisher_search: SidebarSectionSearch,
                             collection_search: SidebarSectionSearch, tags: Vec<TagEntry>,
                             smart_collections: Vec<SmartCollectionRow>,
                             collection_cloud: Arc<HashMap<u64, usize>>,
                             sidebar_open: SidebarOpen)
                             -> impl IntoElement + 'static {
    let active = filter.clone();
    let publishers_open = sidebar_open.publishers;
    let collections_open = sidebar_open.collections;

    // ── Library smart-filter menu
    // ─────────────────────────────────────────────
    let lib_menu = SidebarMenu::new().gap(SIDEBAR_ROW_GAP)
                                     .child(nav_item(&t!("sidebar.all_titles"),
                                                     counts.all,
                                                     active == SidebarFilter::AllTitles,
                                                     SidebarFilter::AllTitles,
                                                     entity.clone()))
                                     .child(nav_item(&t!("sidebar.recently_updated"),
                                                     counts.recently_updated,
                                                     active == SidebarFilter::RecentlyUpdated,
                                                     SidebarFilter::RecentlyUpdated,
                                                     entity.clone()))
                                     .child(nav_item(&t!("sidebar.on_this_device"),
                                                     counts.on_device,
                                                     active == SidebarFilter::OnDevice,
                                                     SidebarFilter::OnDevice,
                                                     entity.clone()))
                                     .child(nav_item(&t!("sidebar.in_the_cloud"),
                                                     counts.in_cloud,
                                                     active == SidebarFilter::InCloud,
                                                     SidebarFilter::InCloud,
                                                     entity.clone()))
                                     .child(nav_item(&t!("sidebar.needs_attention"),
                                                     counts.needs_attention,
                                                     active == SidebarFilter::NeedsAttention,
                                                     SidebarFilter::NeedsAttention,
                                                     entity.clone()));

    let publishers_count = publishers.len();

    // ── Publishers menu
    // ───────────────────────────────────────────────────────
    let mut pub_children: Vec<SidebarMenuItem> =
        publishers.into_iter()
                  .filter(|p| name_matches_query(p.name.as_ref(), &publisher_search.query))
                  .map(|p| {
                      let is_active = active == SidebarFilter::Publisher(Arc::clone(&p.name));
                      let f = SidebarFilter::Publisher(Arc::clone(&p.name));
                      nav_item(p.name.as_ref(), p.count, is_active, f, entity.clone())
                  })
                  .collect();

    // Publishers are derived from the catalog itself, so an empty list while
    // the initial fetch is still in flight means "not yet known", not
    // "confirmed zero publishers" — show a loading indicator instead of a
    // blank section.
    if pub_children.is_empty() && catalog_loading {
        pub_children.push(
            SidebarMenuItem::new(t!("sidebar.loading"))
                .disable(true)
                .suffix(|_window, _cx| Spinner::new().with_size(Size::Small)),
        );
    }

    let entity_for_pub = entity.clone();
    let pub_title: SharedString = if publisher_search.open {
        "".into()
    }
    else {
        t!("sidebar.publishers").into()
    };
    let pub_suffix = render_section_search_suffix("publishers",
                                                  publishers_count,
                                                  publisher_search,
                                                  entity.clone(),
                                                  LibraryController::toggle_publisher_search);
    let pub_section =
        ControlledSection { title:     pub_title,
                            open:      publishers_open,
                            suffix:    Rc::new(move |window, cx| pub_suffix(window, cx)),
                            on_toggle: Rc::new(move |_window, cx| {
                                entity_for_pub.update(cx, |ctrl, cx| {
                                    ctrl.toggle_sidebar_section(SidebarSection::Publishers, cx);
                                });
                            }),
                            items:     pub_children, };

    // ── Sidebar assembly (collections before publishers)
    // ────────────────────── `w_full()` rather than a fixed pixel width:
    // the parent `resizable_panel` in `root_view.rs` already owns width
    // management (its divider IS the drag handle — no separate resize
    // control). A fixed width here would leave the visible sidebar content
    // pinned regardless of the panel's actual dragged width, decoupling
    // what you drag from where the sidebar and catalog actually meet.
    let sidebar_builder = Sidebar::new("sidebar").collapsible(SidebarCollapsible::None)
                                                 .side(Side::Left)
                                                 .w_full()
                                                 .child(SidebarContent::Menu(Box::new(lib_menu)))
                                                 .child(SidebarContent::Separator);

    // ── Collections menu (always present) ────────────────────────────────────
    // Show "?" instead of "0" while the initial collections fetch is still in
    // flight, so an empty list doesn't read as a confirmed zero-collections
    // state.
    let collections_count: SharedString = if collections_loaded {
        collections.len().to_string().into()
    }
    else {
        "?".into()
    };

    let col_rows: Vec<CollectionRow> =
        collections.into_iter()
                   .filter(|c| name_matches_query(c.name.as_ref(), &collection_search.query))
                   .map(|c| {
                       let is_active =
                           matches!(&active, SidebarFilter::Collection(id, _) if *id == c.id);
                       let count = c.member_ids
                                    .iter()
                                    .filter(|id| catalog_ids.contains(id))
                                    .count();
                       CollectionRow { id: c.id,
                                       name: c.name,
                                       count,
                                       in_cloud: collection_cloud.get(&c.id)
                                                                 .copied()
                                                                 .unwrap_or(0),
                                       is_active }
                   })
                   .collect();

    let entity_for_col = entity.clone();
    let col_title: SharedString = if collection_search.open {
        "".into()
    }
    else {
        t!("sidebar.collections").into()
    };
    let collection_search_open = collection_search.open;
    let collection_search_input = collection_search.input.clone();
    let on_toggle: CollectionsToggleFn = Rc::new(move |_window, cx| {
        entity_for_col.update(cx, |ctrl, cx| {
                          ctrl.toggle_sidebar_section(SidebarSection::Collections, cx);
                      });
    });
    let suffix: CollectionsSuffixFn = Rc::new({
        let input = collection_name_input.clone();
        let ctrl = entity.clone();
        let search_input = collection_search_input.clone();
        let entity_for_toggle = entity.clone();
        move |_window, cx| {
            let input = input.clone();
            let ctrl = ctrl.clone();

            if collection_search_open {
                let entity_for_close = entity_for_toggle.clone();
                let search_input_for_close = search_input.clone();
                return div()
                    .id("collections-search-suffix")
                    .on_click(|_, _, cx| cx.stop_propagation())
                    .flex()
                    .items_center()
                    .w_full()
                    .gap(px(4.))
                    .child(div().flex_1().child(Input::new(&search_input).small()))
                    .child(
                        Button::new("collections-search-close")
                            .ghost()
                            .compact()
                            .icon(IconName::Close)
                            .tooltip(t!("sidebar.search_close_tooltip").to_string())
                            .on_click(move |_, window, cx| {
                                entity_for_close.update(cx, |ctrl, cx| {
                                    ctrl.toggle_collection_search(cx);
                                });
                                search_input_for_close.update(cx, |st, cx| {
                                    st.set_value("", window, cx);
                                });
                            }),
                    )
                    .into_any_element();
            }

            div()
                .id("collections-suffix")
                // Stop click propagation so the suffix button does not also
                // fire the SidebarMenuItem header's on_click (which toggles collapse).
                .on_click(|_, _, cx| cx.stop_propagation())
                .flex()
                .items_center()
                .gap(px(4.))
                .child(div().text_xs().child(collections_count.clone()))
                .child(render_collection_sort_control(
                    collection_sort,
                    collection_sort_direction,
                    entity_for_toggle.clone(),
                ))
                .child(
                    Button::new("collections-search-open")
                        .ghost()
                        .compact()
                        .icon(IconName::Search)
                        .tooltip(t!("sidebar.search_tooltip").to_string())
                        .on_click({
                            let entity = entity_for_toggle.clone();
                            let input = search_input.clone();
                            move |_, window, cx| {
                                entity.update(cx, |ctrl, cx| {
                                    ctrl.toggle_collection_search(cx);
                                });
                                input.update(cx, |st, cx| st.focus(window, cx));
                            }
                        }),
                )
                .child(
                    Dialog::new(cx)
                        .trigger(
                            Button::new("add-collection")
                                .ghost()
                                .compact()
                                .icon(IconName::Plus)
                                .tooltip(t!("collections.add_tooltip").to_string()),
                        )
                        .w(px(320.))
                        .close_button(false)
                        .overlay_closable(true)
                        .button_props(
                            DialogButtonProps::default()
                                .ok_text(t!("collections.add_dialog_confirm").to_string())
                                .show_cancel(true)
                                .cancel_text(t!("collections.add_dialog_cancel").to_string()),
                        )
                        .on_ok({
                            let input = input.clone();
                            let ctrl = ctrl.clone();
                            move |_, _, cx| {
                                let name = input.read(cx).value().trim().to_string();
                                if name.is_empty() {
                                    return false;
                                }
                                ctrl.update(cx, |c, cx| c.create_collection(name, cx));
                                true
                            }
                        })
                        .on_cancel(|_, _, _| true)
                        .content({
                            let input = collection_name_input.clone();
                            move |content, _, _| {
                                content
                                    .child(
                                        DialogHeader::new().px_4().pt_4().child(
                                            DialogTitle::new()
                                                .child(t!("collections.add_dialog_title")),
                                        ),
                                    )
                                    .child(div().px_4().py_2().child(Input::new(&input)))
                            }
                        }),
                )
                .into_any_element()
        }
    });

    let collections_section = CollectionsSection { title: col_title,
                                                   open: collections_open,
                                                   suffix,
                                                   on_toggle,
                                                   rows: col_rows,
                                                   loading: !collections_loaded,
                                                   entity: entity.clone() };

    let sidebar_builder =
        sidebar_builder.child(SidebarContent::Collections(Box::new(collections_section)));
    // Nothing to list until a smart collection has been made.
    let sidebar_builder = if smart_collections.is_empty() {
        sidebar_builder
    }
    else {
        sidebar_builder.child(SidebarContent::Separator)
                       .child(SidebarContent::Menu(Box::new(smart_collections_menu(smart_collections,
                                                                                   &active,
                                                                                   &entity))))
    };
    let sidebar_builder = sidebar_builder.child(SidebarContent::Separator)
                                         .child(SidebarContent::Section(Box::new(pub_section)));
    // Nothing to list until something has been tagged.
    let sidebar_builder = if tags.is_empty() {
        sidebar_builder
    }
    else {
        sidebar_builder.child(SidebarContent::Separator)
                       .child(SidebarContent::Section(Box::new(tags_section(tags,
                                                                            sidebar_open.tags,
                                                                            &active,
                                                                            &entity))))
    };
    // Analysis comes last, as in the design.
    sidebar_builder.child(SidebarContent::Separator)
                   .child(SidebarContent::Menu(Box::new(analysis_menu(active
                                                                      == SidebarFilter::Insights,
                                                                      entity))))
}

/// The Analysis group, holding Insights. The entry has no count, since it shows
/// figures, not titles.
fn analysis_menu(insights_active: bool, entity: Entity<LibraryController>) -> SidebarMenu {
    let insights = SidebarMenuItem::new(t!("sidebar.insights")).icon(Icon::empty().path("icons/chart-no-axes-column.svg"))
                                                               .active(insights_active)
                                                               .on_click(move |_, _, cx| {
                                                                   entity.update(cx, |ctrl, cx| {
                                                                             ctrl.set_filter(SidebarFilter::Insights,
                                                                                             cx);
                                                                         });
                                                               });
    SidebarMenu::new().gap(SIDEBAR_ROW_GAP)
                      .child(SidebarMenuItem::new(t!("sidebar.analysis")).default_open(true)
                                                                         .click_to_toggle(true)
                                                                         .children([insights]))
}

/// The Tags section: a header with the number of tags, over a row for each tag.
fn tags_section(tags: Vec<TagEntry>, open: bool, active: &SidebarFilter,
                entity: &Entity<LibraryController>)
                -> ControlledSection {
    let count = tags.len();
    let items = tag_rows(tags, active, entity);
    let toggle_entity = entity.clone();
    ControlledSection { title: t!("sidebar.tags").into(),
                        open,
                        suffix: Rc::new(move |_, _| {
                            div().text_xs().child(count.to_string()).into_any_element()
                        }),
                        on_toggle: Rc::new(move |_window, cx| {
                            toggle_entity.update(cx, |ctrl, cx| {
                                             ctrl.toggle_sidebar_section(SidebarSection::Tags, cx);
                                         });
                        }),
                        items }
}
