//! The root view's render tree: composes the title bar, sidebar, active tab
//! content, and status bar from the controllers' snapshots, and wires the
//! window-level menu/keyboard actions.

use gpui_kit::component::Selectable as _;

use super::*;
use crate::ui::views::filters_column::{render_filter_chips, render_filters_column};
use crate::ui::views::sidebar_smart::SmartCollectionRow;

pub(super) fn render_root_view(view: &mut LibraryRootView, window: &mut gpui::Window,
                               cx: &mut Context<LibraryRootView>)
                               -> impl IntoElement {
    // `Root` (gpui-kit, wrapping this view — see `gpui_kit::open_window`
    // in `ui/app/mod.rs`) renders its own dialog/sheet/notification
    // layers automatically via its own `Render` impl; this view does
    // not need to compose them.

    // Adjusting the shared rem size scales every `rems(...)`-based size
    // utility (`.text_sm()`, `.text_xs()`, etc.) in this window, like
    // zooming a page — see `LibraryController::set_ui_text_size`.
    window.set_rem_size(cx.global::<LibriTheme>().fonts.ui_text_size);

    let lib_entity = view.controller.clone();
    let settings_entity = view.settings.clone();
    let activity_entity = view.activity.clone();
    let auth_entity = view.auth_state.clone();

    let snap = view.controller.read(cx).snapshot();
    let tags = snap.tags;
    let (filter,
         counts,
         publishers,
         collections,
         collections_loaded,
         catalog_loading,
         catalog_ids,
         total_count,
         total_mb,
         matched_count,
         sort,
         sort_direction,
         collection_sort,
         collection_sort_direction,
         group_by,
         presentation,
         bulk_selection_mode,
         bulk_selected_ids,
         bulk_selection_count,
         bulk_items) = (snap.filter,
                        snap.counts,
                        snap.publishers,
                        snap.collections,
                        snap.collections_loaded,
                        snap.catalog_loading,
                        snap.catalog_ids,
                        snap.total_count,
                        snap.total_mb,
                        snap.matched_count,
                        snap.sort,
                        snap.sort_direction,
                        snap.collection_sort,
                        snap.collection_sort_direction,
                        snap.group_by,
                        snap.presentation,
                        snap.bulk_selection_mode,
                        snap.bulk_selected_ids,
                        snap.bulk_selection_count,
                        snap.items);

    let settings_snap = view.settings.read(cx).snapshot();
    let activity_snap = view.activity.read(cx).snapshot();
    let notices: Vec<_> = view.auth_state
                              .read(cx)
                              .active_notices()
                              .into_iter()
                              .cloned()
                              .collect();

    let theme = cx.global::<LibriTheme>().clone();
    let colors = &theme.colors;

    let bulk_action_bar =
        bulk_selection_mode.then(|| {
                               render_bulk_action_bar(bulk_selection_count,
                                                      &bulk_selected_ids,
                                                      &bulk_items,
                                                      &collections,
                                                      lib_entity.clone(),
                                                      view.bulk_pattern_input.clone(),
                                                      view.bulk_pattern_field.clone(),
                                                      colors)
                           });

    let publisher_search =
        crate::ui::views::sidebar_view::SidebarSectionSearch { open:  snap.publisher_search_open,
                                                               query: snap.publisher_search_query,
                                                               input: view.publisher_search_input
                                                                          .clone(), };
    let collection_search =
        crate::ui::views::sidebar_view::SidebarSectionSearch { open:  snap.collection_search_open,
                                                               query: snap.collection_search_query,
                                                               input: view.collection_search_input
                                                                          .clone(), };
    let smart_rows = {
        let ctrl = view.controller.read(cx);
        ctrl.smart_collections()
            .iter()
            .map(|collection| SmartCollectionRow { id:    collection.id,
                                                   name:  collection.name.as_str().into(),
                                                   count: ctrl.smart_collection_count(collection.id), })
            .collect::<Vec<_>>()
    };
    let sidebar = render_sidebar(filter.clone(),
                                 counts,
                                 publishers,
                                 collections,
                                 collections_loaded,
                                 catalog_loading,
                                 catalog_ids,
                                 collection_sort,
                                 collection_sort_direction,
                                 lib_entity.clone(),
                                 view.collection_name_input.clone(),
                                 publisher_search,
                                 collection_search,
                                 tags,
                                 smart_rows,
                                 snap.sidebar_open);
    let search_query = dtrpg_core::util::search::SearchQuery::parse(&snap.search_query);
    let filters_toggle =
        render_filters_toggle(view.filters_open, search_query.terms().len(), cx.entity());
    let toolbar = render_toolbar(&filter,
                                 snap.filter_count,
                                 matched_count,
                                 total_count,
                                 &snap.search_query,
                                 view.search_input.clone(),
                                 sort,
                                 sort_direction,
                                 group_by,
                                 presentation,
                                 bulk_selection_mode,
                                 lib_entity.clone(),
                                 filters_toggle,
                                 colors);
    let chips = render_filter_chips(&search_query, &view.search_input, colors);
    let filters_column =
        view.filters_open.then(|| {
                             render_filters_column(&view.controller.read(cx).visible_facets(),
                                                   &search_query,
                                                   &view.search_input,
                                                   colors)
                         });
    let title_bar = render_title_bar(&settings_snap.auth,
                                     settings_entity.clone(),
                                     colors,
                                     view.menu_bar.clone(),
                                     cx);
    let banner = render_notification_banner(notices, auth_entity, cx.entity(), colors);

    let alert_snap = view.activity.read(cx).alert_snapshot();
    let current_locale =
        crate::i18n::Locale::from_code(&rust_i18n::locale()).unwrap_or(crate::i18n::Locale::En);
    let status_bar = render_status_bar(StatusBarSnapshot { total_count,
                                                           total_mb,
                                                           section_label:
                                                               crate::ui::views::toolbar_view::section_title_for(&filter),
                                                           section_count: matched_count,
                                                           theme_key: theme.id.clone(),
                                                           current_locale },
                                       lib_entity.clone(),
                                       ActivityBarData { entity:     activity_entity.clone(),
                                                         snap:       &activity_snap,
                                                         alert_snap: &alert_snap, },
                                       colors);

    let catalog_content =
        div().flex_1()
             .min_h_0()
             .flex()
             .flex_col()
             .child(toolbar)
             .children(chips)
             .children(bulk_action_bar)
             .child(banner)
             .child(div().flex_1()
                         .min_h_0()
                         .flex()
                         .children(filters_column)
                         .child(div().flex_1().min_w_0().child(view.catalog_view.clone())));

    let surface = colors.surface;
    let text_primary = colors.text_primary;

    // Settings renders in its own window (see `settings_window_view`), not
    // as an overlay here, so the main content area no longer
    // branches on `settings_snap.is_open`.
    let main_content = div().flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .relative()
                            .bg(surface)
                            .child(catalog_content);

    let sidebar_col = div().size_full().relative().child(sidebar);

    let this_entity = cx.entity();
    let this_entity_for_about = this_entity.clone();
    let this_entity_for_add = this_entity.clone();
    let this_entity_for_activity = this_entity.clone();
    let this_entity_for_alert_history = this_entity.clone();
    let this_entity_for_report_bug = this_entity.clone();
    let controller_for_reload = view.controller.clone();
    let controller_for_refresh_thumbnails = view.controller.clone();
    let controller_for_check_availability = view.controller.clone();
    let controller_for_view_list = view.controller.clone();
    let controller_for_view_thumbs = view.controller.clone();
    let controller_for_view_grid = view.controller.clone();
    let controller_for_sort_title = view.controller.clone();
    let controller_for_sort_publisher = view.controller.clone();
    let controller_for_sort_date_added = view.controller.clone();
    let controller_for_sort_pages = view.controller.clone();
    let controller_for_sort_asc = view.controller.clone();
    let controller_for_sort_desc = view.controller.clone();
    let controller_for_group_toggle = view.controller.clone();
    let controller_for_group_by_tag = view.controller.clone();
    let search_input_for_focus = view.search_input.clone();
    let sidebar_initial = view.sidebar_width;
    let selected_item = view.controller.read(cx).selected_item();
    let inspector_pane = selected_item.as_ref()
                                      .map(|item| render_inspector_pane(view, item, window, cx));

    div()
        .size_full()
        .bg(surface)
        .text_color(text_primary)
        .relative()
        .track_focus(&view.root_focus)
        .on_action(move |_: &ShowSettings, _, cx| {
            this_entity.update(cx, |view, cx| view.show_settings(cx));
        })
        .on_action(move |_: &About, _, cx| {
            this_entity_for_about.update(cx, |view, cx| view.show_about(cx));
        })
        .on_action(move |_: &ReportFeatureBug, _, cx| {
            this_entity_for_report_bug.update(cx, |view, cx| view.show_report_feature_bug(cx));
        })
        .on_action(move |_: &ReloadCatalog, _, cx| {
            controller_for_reload.update(cx, |ctrl, cx| ctrl.reload_catalog(cx));
        })
        .on_action(move |_: &RefreshThumbnails, _, cx| {
            controller_for_refresh_thumbnails
                .update(cx, |ctrl, cx| ctrl.refresh_all_thumbnails(cx));
        })
        .on_action(move |_: &CheckItemAvailability, _, cx| {
            controller_for_check_availability
                .update(cx, |ctrl, cx| ctrl.request_manual_check_batch(cx));
        })
        .on_action(move |_: &ViewAsList, _, cx| {
            controller_for_view_list.update(cx, |ctrl, cx| {
                ctrl.set_presentation(CatalogPresentation::List, cx)
            });
        })
        .on_action(move |_: &ViewAsThumbs, _, cx| {
            controller_for_view_thumbs.update(cx, |ctrl, cx| {
                ctrl.set_presentation(CatalogPresentation::Thumbs, cx);
            });
        })
        .on_action(move |_: &ViewAsGrid, _, cx| {
            controller_for_view_grid.update(cx, |ctrl, cx| {
                ctrl.set_presentation(CatalogPresentation::Grid, cx)
            });
        })
        .on_action(move |_: &SortByTitle, _, cx| {
            controller_for_sort_title
                .update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::Title, cx));
        })
        .on_action(move |_: &SortByPublisher, _, cx| {
            controller_for_sort_publisher
                .update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::Publisher, cx));
        })
        .on_action(move |_: &SortByDateAdded, _, cx| {
            controller_for_sort_date_added
                .update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::DateAdded, cx));
        })
        .on_action(move |_: &SortByPages, _, cx| {
            controller_for_sort_pages
                .update(cx, |ctrl, cx| ctrl.set_sort(SortMethod::PageCount, cx));
        })
        .on_action(move |_: &SortAscending, _, cx| {
            controller_for_sort_asc.update(cx, |ctrl, cx| {
                ctrl.set_sort_direction(SortDirection::Ascending, cx);
            });
        })
        .on_action(move |_: &SortDescending, _, cx| {
            controller_for_sort_desc.update(cx, |ctrl, cx| {
                ctrl.set_sort_direction(SortDirection::Descending, cx);
            });
        })
        .on_action(move |_: &ToggleGroupByPublisher, _, cx| {
            controller_for_group_toggle.update(cx, |ctrl, cx| {
                ctrl.toggle_group_by(GroupBy::Publisher, cx);
            });
        })
        .on_action(move |_: &ToggleGroupByTag, _, cx| {
            controller_for_group_by_tag.update(cx, |ctrl, cx| {
                ctrl.toggle_group_by(GroupBy::Tag, cx);
            });
        })
        .on_action(move |_: &FocusSearch, window, cx| {
            search_input_for_focus.update(cx, |input, cx| input.focus(window, cx));
        })
        .on_action(move |_: &AddCollection, window, cx| {
            this_entity_for_add.update(cx, |view, cx| {
                view.show_add_collection_dialog(window, cx);
            });
        })
        .on_action(move |_: &ShowActivity, _, cx| {
            this_entity_for_activity.update(cx, |view, cx| view.show_activity(cx));
        })
        .on_action(move |_: &ShowAlertHistory, _, cx| {
            this_entity_for_alert_history.update(cx, |view, cx| view.show_alert_history(cx));
        })

        .child(
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(title_bar)
                .child(
                    div().flex_1().min_h_0().child(
                        h_resizable("main-layout")
                            .with_state(&view.resize_state)
                            .child(
                                resizable_panel()
                                    .size(px(sidebar_initial))
                                    .size_range(px(180.)..px(361.))
                                    .child(sidebar_col),
                            )
                            .child(
                                resizable_panel()
                                    .size_range(px(280.)..Pixels::MAX)
                                    .child(main_content),
                            )
                            .child(
                                resizable_panel()
                                    .visible(inspector_pane.is_some())
                                    .size(px(view.inspector_width))
                                    .size_range(px(INSPECTOR_PANE_MIN_WIDTH)
                                                ..px(INSPECTOR_PANE_MAX_WIDTH))
                                    .children(inspector_pane),
                            ),
                    ),
                )
                .child(status_bar),
        )
}

/// Renders the toolbar button that shows or hides the filters column. A number
/// on its tooltip says how many filters are applied.
fn render_filters_toggle(open: bool, applied: usize, root: Entity<LibraryRootView>) -> AnyElement {
    let tooltip = if applied == 0 {
        t!("filters.toggle_tooltip").to_string()
    }
    else {
        t!("filters.toggle_tooltip_applied", n = applied).to_string()
    };
    let button = Button::new("filters-toggle").icon(gpui_kit::component::Icon::empty()
                                                        .path("icons/sliders-horizontal.svg"))
                                              .ghost()
                                              .selected(open)
                                              .tooltip(tooltip)
                                              .on_click(move |_, _, cx| {
                                                  root.update(cx, |view, cx| {
                                                          view.filters_open = !view.filters_open;
                                                          cx.notify();
                                                      });
                                              });
    if applied == 0 {
        return button.into_any_element();
    }
    gpui_kit::component::badge::Badge::new().count(applied)
                                            .child(button)
                                            .into_any_element()
}
