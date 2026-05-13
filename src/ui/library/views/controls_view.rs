//! Top control strip rendering for the library view.

use gpui::{Context, IntoElement, div, prelude::*, rgb};

use crate::ui::library::model::library_data::{FilterScope, LibraryViewMode};

use super::root_view::LibraryRootView;

pub(crate) fn render_controls_row(
    root: &LibraryRootView,
    cx: &mut Context<LibraryRootView>,
) -> impl IntoElement {
    let mode_label = root.controller.mode_label();
    let active_query = root.controller.active_query_label();
    let count = root.controller.filtered_item_count();

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .gap_2()
                .items_center()
                .child("View mode")
                .child(
                    div()
                        .id("view-flat")
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .cursor_pointer()
                        .when(root.controller.view_mode == LibraryViewMode::FlatList, |d| {
                            d.bg(rgb(0x2563eb))
                        })
                        .when(root.controller.view_mode != LibraryViewMode::FlatList, |d| {
                            d.bg(rgb(0x1f2937))
                        })
                        .child("Flat")
                        .on_click(cx.listener(|this, _, _, _| {
                            this.controller.set_view_mode(LibraryViewMode::FlatList)
                        })),
                )
                .child(
                    div()
                        .id("view-publisher")
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .cursor_pointer()
                        .when(
                            root.controller.view_mode == LibraryViewMode::TreeByPublisher,
                            |d| d.bg(rgb(0x2563eb)),
                        )
                        .when(
                            root.controller.view_mode != LibraryViewMode::TreeByPublisher,
                            |d| d.bg(rgb(0x1f2937)),
                        )
                        .child("Tree: Publisher")
                        .on_click(cx.listener(|this, _, _, _| {
                            this.controller.set_view_mode(LibraryViewMode::TreeByPublisher)
                        })),
                )
                .child(
                    div()
                        .id("view-product-type")
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .cursor_pointer()
                        .when(
                            root.controller.view_mode == LibraryViewMode::TreeByProductType,
                            |d| d.bg(rgb(0x2563eb)),
                        )
                        .when(
                            root.controller.view_mode != LibraryViewMode::TreeByProductType,
                            |d| d.bg(rgb(0x1f2937)),
                        )
                        .child("Tree: Product Type")
                        .on_click(cx.listener(|this, _, _, _| {
                            this.controller.set_view_mode(LibraryViewMode::TreeByProductType)
                        })),
                )
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
                        .border_color(rgb(0x374151))
                        .rounded_sm()
                        .child(format!("Search: {active_query}")),
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
                .child("Tree filter scope")
                .child(
                    div()
                        .id("scope-child")
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .cursor_pointer()
                        .when(root.controller.filter_scope == FilterScope::ChildOnly, |d| {
                            d.bg(rgb(0x2563eb))
                        })
                        .when(root.controller.filter_scope != FilterScope::ChildOnly, |d| {
                            d.bg(rgb(0x1f2937))
                        })
                        .child("Child only")
                        .on_click(cx.listener(|this, _, _, _| {
                            this.controller.set_filter_scope(FilterScope::ChildOnly)
                        })),
                )
                .child(
                    div()
                        .id("scope-root-child")
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .cursor_pointer()
                        .when(root.controller.filter_scope == FilterScope::RootAndChild, |d| {
                            d.bg(rgb(0x2563eb))
                        })
                        .when(root.controller.filter_scope != FilterScope::RootAndChild, |d| {
                            d.bg(rgb(0x1f2937))
                        })
                        .child("Root + child")
                        .on_click(cx.listener(|this, _, _, _| {
                            this.controller.set_filter_scope(FilterScope::RootAndChild)
                        })),
                )
                .child(
                    div()
                        .id("scope-root-only")
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .cursor_pointer()
                        .when(root.controller.filter_scope == FilterScope::RootOnly, |d| {
                            d.bg(rgb(0x2563eb))
                        })
                        .when(root.controller.filter_scope != FilterScope::RootOnly, |d| {
                            d.bg(rgb(0x1f2937))
                        })
                        .child("Root only")
                        .on_click(cx.listener(|this, _, _, _| {
                            this.controller.set_filter_scope(FilterScope::RootOnly)
                        })),
                )
                .child(
                    div()
                        .id("scope-cycle")
                        .px_2()
                        .py_1()
                        .bg(rgb(0x1f2937))
                        .rounded_sm()
                        .cursor_pointer()
                        .child("Cycle scope")
                        .on_click(cx.listener(|this, _, _, _| this.controller.cycle_filter_scope())),
                )
                .child(
                    div()
                        .id("flat-sort")
                        .px_2()
                        .py_1()
                        .bg(rgb(0x1f2937))
                        .rounded_sm()
                        .cursor_pointer()
                        .child(format!("Flat sort: {}", root.controller.flat_sort_label()))
                        .on_click(cx.listener(|this, _, _, _| this.controller.cycle_flat_sort())),
                )
                .child(
                    div()
                        .id("outer-sort")
                        .px_2()
                        .py_1()
                        .bg(rgb(0x1f2937))
                        .rounded_sm()
                        .cursor_pointer()
                        .child(format!("Tree outer: {}", root.controller.outer_sort_label()))
                        .on_click(cx.listener(|this, _, _, _| this.controller.cycle_outer_sort())),
                )
                .child(
                    div()
                        .id("inner-sort")
                        .px_2()
                        .py_1()
                        .bg(rgb(0x1f2937))
                        .rounded_sm()
                        .cursor_pointer()
                        .child(format!("Tree inner: {}", root.controller.inner_sort_label()))
                        .on_click(cx.listener(|this, _, _, _| this.controller.cycle_inner_sort())),
                ),
        )
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
