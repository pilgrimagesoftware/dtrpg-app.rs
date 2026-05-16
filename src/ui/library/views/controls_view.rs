//! Top control strip rendering for the library view.

use std::hash::{Hash, Hasher};

use gpui::{Context, IntoElement, div, prelude::*, rgb};

use crate::ui::library::controller::library_controller::{LibraryController, SortPopup};
use crate::ui::library::model::library_data::{
    FilterScope, LibraryViewMode, MatchPresentation, SortMethod,
};

use super::root_view::LibraryRootView;

pub(crate) fn render_controls_row(
    root: &LibraryRootView,
    cx: &mut Context<LibraryRootView>,
) -> impl IntoElement {
    let mode_label = root.controller.mode_label();
    let active_query = root.controller.active_query_label();
    let count = root.controller.filtered_item_count();
    let summary = root.controller.controls_summary();

    let search_mode = match root.controller.match_presentation {
        MatchPresentation::HideNonMatching => "Hide non-matching",
        MatchPresentation::HighlightMatching => "Highlight matches",
    };

    let mut controls = div().flex().flex_col().gap_2().child(
        div()
            .flex()
            .gap_2()
            .items_center()
            .child(
                div()
                    .id("toggle-filter-disclosure")
                    .px_2()
                    .py_1()
                    .bg(rgb(0x1d4ed8))
                    .rounded_sm()
                    .cursor_pointer()
                    .child(if root.controller.controls_disclosed {
                        "Hide filters"
                    } else {
                        "Show filters"
                    })
                    .on_click(
                        cx.listener(|this, _, _, _| this.controller.toggle_controls_disclosure()),
                    ),
            )
            .child(
                div()
                    .px_2()
                    .py_1()
                    .bg(rgb(0x0f172a))
                    .rounded_sm()
                    .child(summary),
            ),
    );

    if !root.controller.controls_disclosed {
        return controls;
    }

    controls = controls
        .child(
            div()
                .flex()
                .gap_2()
                .items_center()
                .child("View mode")
                .child(view_mode_button(
                    1,
                    "Flat",
                    root.controller.view_mode == LibraryViewMode::FlatList,
                    cx.listener(|this, _, _, _| {
                        this.controller.set_view_mode(LibraryViewMode::FlatList)
                    }),
                ))
                .child(view_mode_button(
                    2,
                    "Tree: Publisher",
                    root.controller.view_mode == LibraryViewMode::TreeByPublisher,
                    cx.listener(|this, _, _, _| {
                        this.controller
                            .set_view_mode(LibraryViewMode::TreeByPublisher)
                    }),
                ))
                .child(view_mode_button(
                    3,
                    "Tree: Type",
                    root.controller.view_mode == LibraryViewMode::TreeByProductType,
                    cx.listener(|this, _, _, _| {
                        this.controller
                            .set_view_mode(LibraryViewMode::TreeByProductType)
                    }),
                ))
                .child(view_mode_button(
                    4,
                    "Grid: Publisher",
                    root.controller.view_mode == LibraryViewMode::GridByPublisher,
                    cx.listener(|this, _, _, _| {
                        this.controller
                            .set_view_mode(LibraryViewMode::GridByPublisher)
                    }),
                ))
                .child(view_mode_button(
                    5,
                    "Grid: Type",
                    root.controller.view_mode == LibraryViewMode::GridByProductType,
                    cx.listener(|this, _, _, _| {
                        this.controller
                            .set_view_mode(LibraryViewMode::GridByProductType)
                    }),
                ))
                .child(
                    div()
                        .id("view-mode-cycle")
                        .px_2()
                        .py_1()
                        .bg(rgb(0x1f2937))
                        .rounded_sm()
                        .cursor_pointer()
                        .child(format!("Cycle ({mode_label})"))
                        .on_click(cx.listener(|this, _, _, _| this.controller.cycle_view_mode())),
                ),
        )
        .child(
            div()
                .flex()
                .gap_2()
                .items_center()
                .child(
                    div()
                        .id("filter-box")
                        .px_2()
                        .py_1()
                        .bg(rgb(0x111827))
                        .border_1()
                        .border_color(if root.controller.search_editing {
                            rgb(0x2563eb)
                        } else {
                            rgb(0x374151)
                        })
                        .rounded_sm()
                        .child(format!(
                            "Search: {active_query} (type; Esc to stop; Cmd/Ctrl+F focus)"
                        ))
                        .on_click(cx.listener(|this, _, _, _| {
                            this.controller.begin_search_editing()
                        }))
                        .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, _, _| {
                            this.controller
                                .handle_global_key(&event.keystroke.key, &event.keystroke.modifiers);
                        })),
                )
                .child(
                    div()
                        .id("clear-filter")
                        .px_2()
                        .py_1()
                        .bg(rgb(0x334155))
                        .rounded_sm()
                        .cursor_pointer()
                        .child("Clear")
                        .on_click(cx.listener(|this, _, _, _| this.controller.clear_filter_query())),
                )
                .child(search_chip(1, "atlas", root, cx))
                .child(search_chip(2, "sandbox", root, cx))
                .child(search_chip(3, "urban", root, cx))
                .child(search_chip(4, "tabletop", root, cx))
                .child(
                    div()
                        .id("cycle-filter")
                        .px_2()
                        .py_1()
                        .bg(rgb(0x1f2937))
                        .rounded_sm()
                        .cursor_pointer()
                        .child("Cycle presets")
                        .on_click(cx.listener(|this, _, _, _| this.controller.cycle_filter_query())),
                )
                .child(
                    div()
                        .id("search-mode")
                        .px_2()
                        .py_1()
                        .bg(rgb(0x1f2937))
                        .rounded_sm()
                        .cursor_pointer()
                        .child(format!("Search mode: {search_mode}"))
                        .on_click(
                            cx.listener(|this, _, _, _| this.controller.toggle_match_presentation()),
                        ),
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .bg(rgb(0x0f172a))
                        .rounded_sm()
                        .child(format!("Visible items: {count}")),
                ),
        )
        .child(
            div()
                .flex()
                .gap_2()
                .items_center()
                .child("Parent matching")
                .child(scope_button(
                    "Child only",
                    root.controller.filter_scope == FilterScope::ChildOnly,
                    cx.listener(|this, _, _, _| {
                        this.controller.set_filter_scope(FilterScope::ChildOnly)
                    }),
                ))
                .child(scope_button(
                    "Root + child",
                    root.controller.filter_scope == FilterScope::RootAndChild,
                    cx.listener(|this, _, _, _| {
                        this.controller.set_filter_scope(FilterScope::RootAndChild)
                    }),
                ))
                .child(scope_button(
                    "Root only",
                    root.controller.filter_scope == FilterScope::RootOnly,
                    cx.listener(|this, _, _, _| {
                        this.controller.set_filter_scope(FilterScope::RootOnly)
                    }),
                ))
                .child(render_sort_controls(root, cx)),
        )
        .child(
            div()
                .text_color(rgb(0x93c5fd))
                .child("Keyboard: / or Cmd/Ctrl+F focuses search, Esc exits search editing, Cmd/Ctrl+L clears search."),
        );

    controls
}

fn render_sort_controls(
    root: &LibraryRootView,
    cx: &mut Context<LibraryRootView>,
) -> impl IntoElement {
    match root.controller.view_mode {
        LibraryViewMode::FlatList => div().child(sort_selector(
            "Sort",
            root.controller.flat_sort,
            root.controller.open_sort_popup == Some(SortPopup::Flat),
            cx.listener(|this, _, _, _| this.controller.toggle_sort_popup(SortPopup::Flat)),
            |controller, sort| controller.set_flat_sort(sort),
            root,
            cx,
        )),
        _ => div()
            .child(sort_selector(
                "Tree outer",
                root.controller.outer_sort,
                root.controller.open_sort_popup == Some(SortPopup::Outer),
                cx.listener(|this, _, _, _| this.controller.toggle_sort_popup(SortPopup::Outer)),
                |controller, sort| controller.set_outer_sort(sort),
                root,
                cx,
            ))
            .child(sort_selector(
                "Tree inner",
                root.controller.inner_sort,
                root.controller.open_sort_popup == Some(SortPopup::Inner),
                cx.listener(|this, _, _, _| this.controller.toggle_sort_popup(SortPopup::Inner)),
                |controller, sort| controller.set_inner_sort(sort),
                root,
                cx,
            )),
    }
}

fn sort_selector(
    label: &str,
    current: SortMethod,
    open: bool,
    toggle: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
    set_sort: impl Fn(&mut LibraryController, SortMethod) + Copy + 'static,
    root: &LibraryRootView,
    cx: &mut Context<LibraryRootView>,
) -> impl IntoElement {
    let mut menu = div().flex().flex_col().gap_1().child(
        div()
            .id(("sort-selector", stable_id(label)))
            .px_2()
            .py_1()
            .bg(rgb(0x1f2937))
            .rounded_sm()
            .cursor_pointer()
            .child(format!("{label}: {}", sort_label(current)))
            .on_click(toggle),
    );

    if open {
        menu = menu
            .child(sort_option(SortMethod::AtoZ, current, set_sort, root, cx))
            .child(sort_option(SortMethod::ZtoA, current, set_sort, root, cx))
            .child(sort_option(
                SortMethod::MostRecentlyAdded,
                current,
                set_sort,
                root,
                cx,
            ))
            .child(sort_option(
                SortMethod::MostRecentlyUpdated,
                current,
                set_sort,
                root,
                cx,
            ))
            .child(
                div()
                    .id(("sort-close", 1_u64))
                    .px_2()
                    .py_1()
                    .bg(rgb(0x0f172a))
                    .rounded_sm()
                    .cursor_pointer()
                    .child("Close")
                    .on_click(cx.listener(|this, _, _, _| this.controller.close_sort_popup())),
            );
    }

    menu
}

fn sort_option(
    sort: SortMethod,
    current: SortMethod,
    set_sort: impl Fn(&mut LibraryController, SortMethod) + Copy + 'static,
    _root: &LibraryRootView,
    cx: &mut Context<LibraryRootView>,
) -> impl IntoElement {
    div()
        .id(("sort-option", sort as u64))
        .px_2()
        .py_1()
        .rounded_sm()
        .cursor_pointer()
        .when(sort == current, |d| d.bg(rgb(0x2563eb)))
        .when(sort != current, |d| d.bg(rgb(0x1e293b)))
        .child(sort_label(sort))
        .on_click(cx.listener(move |this, _, _, _| set_sort(&mut this.controller, sort)))
}

fn sort_label(sort: SortMethod) -> &'static str {
    match sort {
        SortMethod::AtoZ => "A-Z",
        SortMethod::ZtoA => "Z-A",
        SortMethod::MostRecentlyAdded => "Most recently added",
        SortMethod::MostRecentlyUpdated => "Most recently updated",
    }
}

fn view_mode_button(
    id: u64,
    label: &str,
    active: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    div()
        .id(("view-mode-button", id))
        .px_2()
        .py_1()
        .rounded_sm()
        .cursor_pointer()
        .when(active, |d| d.bg(rgb(0x2563eb)))
        .when(!active, |d| d.bg(rgb(0x1f2937)))
        .child(label.to_string())
        .on_click(on_click)
}

fn scope_button(
    label: &str,
    active: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    div()
        .id(("scope-button", stable_id(label)))
        .px_2()
        .py_1()
        .rounded_sm()
        .cursor_pointer()
        .when(active, |d| d.bg(rgb(0x2563eb)))
        .when(!active, |d| d.bg(rgb(0x1f2937)))
        .child(label.to_string())
        .on_click(on_click)
}

fn stable_id(label: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    label.hash(&mut hasher);
    hasher.finish()
}

fn search_chip(
    id_suffix: u64,
    query: &'static str,
    root: &LibraryRootView,
    cx: &mut Context<LibraryRootView>,
) -> impl IntoElement {
    let active = root.controller.filter_query.eq_ignore_ascii_case(query);

    div()
        .id(("search-chip", id_suffix))
        .px_2()
        .py_1()
        .rounded_sm()
        .cursor_pointer()
        .when(active, |d| d.bg(rgb(0x0284c7)))
        .when(!active, |d| d.bg(rgb(0x1f2937)))
        .child(query)
        .on_click(cx.listener(move |this, _, _, _| this.controller.set_filter_query(query)))
}
