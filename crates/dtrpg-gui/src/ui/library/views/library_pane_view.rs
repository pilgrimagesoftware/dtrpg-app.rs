//! Left library pane rendering for the library view.

use std::hash::{Hash, Hasher};

use gpui::{Context, IntoElement, div, prelude::*, rgb};

use crate::ui::library::model::library_data::{LibraryViewMode, MatchPresentation};

use crate::ui::library::controller::library_controller::Selection;

use super::root_view::LibraryRootView;

pub(crate) fn render_library_pane(
    root: &LibraryRootView,
    cx: &mut Context<LibraryRootView>,
) -> impl IntoElement {
    let left_content = match root.controller.view_mode {
        LibraryViewMode::FlatList => {
            let items = root.controller.flat_items();
            div()
                .flex()
                .flex_col()
                .gap_1()
                .children(items.into_iter().map(|item| {
                let item_id = item.id;
                let selected =
                    matches!(root.controller.selection, Some(Selection::Item(id)) if id == item_id);
                let matches = root.controller.is_item_match(&item);
                div()
                    .id(("flat-item", item_id))
                    .flex()
                    .justify_between()
                    .p_2()
                    .when(selected, |d| d.bg(rgb(0x334155)))
                    .when(
                        !selected
                            && matches
                            && root.controller.match_presentation
                                == MatchPresentation::HighlightMatching,
                        |d| d.bg(rgb(0x1e3a8a)),
                    )
                    .when(
                        !selected
                            && !(matches
                                && root.controller.match_presentation
                                    == MatchPresentation::HighlightMatching),
                        |d| d.bg(rgb(0x111827)),
                    )
                    .border_1()
                    .border_color(rgb(0x374151))
                    .rounded_sm()
                    .cursor_pointer()
                    .child(item.title)
                    .child(item.publisher)
                    .on_click(cx.listener(move |this, _, _, _| {
                        this.controller.set_item_selection(item_id);
                    }))
            }))
        }
        LibraryViewMode::TreeByPublisher | LibraryViewMode::TreeByProductType => {
            let nodes = root.controller.tree_items();
            div().flex().flex_col().gap_2().children(nodes.into_iter().map(|node| {
                let root_label = node.root_label.clone();
                let root_selected = match (&root.controller.selection, root.controller.view_mode) {
                    (Some(Selection::Publisher(value)), LibraryViewMode::TreeByPublisher) => {
                        value == &root_label
                    }
                    (Some(Selection::ProductType(value)), LibraryViewMode::TreeByProductType) => {
                        value == &root_label
                    }
                    _ => false,
                };

                let root_matches = root.controller.is_root_match(&root_label);
                let root_id = stable_root_id(&root_label);
                let root_row = div()
                    .id(("tree-root", root_id))
                    .p_2()
                    .rounded_sm()
                    .cursor_pointer()
                    .when(root_selected, |d| d.bg(rgb(0x334155)))
                    .when(!root_selected && root_matches && root.controller.match_presentation == MatchPresentation::HighlightMatching, |d| {
                        d.bg(rgb(0x1e3a8a))
                    })
                    .when(!root_selected && !(root_matches && root.controller.match_presentation == MatchPresentation::HighlightMatching), |d| d.bg(rgb(0x1f2937)))
                    .child(format!("{} ({})", root_label, node.children.len()))
                    .on_click(cx.listener(move |this, _, _, _| match this.controller.view_mode {
                        LibraryViewMode::TreeByPublisher => {
                            this.controller.set_publisher_selection(root_label.clone())
                        }
                        LibraryViewMode::TreeByProductType => {
                            this.controller.set_product_type_selection(root_label.clone())
                        }
                        LibraryViewMode::FlatList
                        | LibraryViewMode::GridByPublisher
                        | LibraryViewMode::GridByProductType => {}
                    }));

                let child_rows =
                    div().pl_4().flex().flex_col().gap_1().children(node.children.into_iter().map(
                        |item| {
                            let item_id = item.id;
                            let selected =
                                matches!(root.controller.selection, Some(Selection::Item(id)) if id == item_id);
                            let matches = root.controller.is_item_match(&item);
                            div()
                                .id(("tree-item", item_id))
                                .p_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .when(selected, |d| d.bg(rgb(0x475569)))
                                .when(!selected && matches && root.controller.match_presentation == MatchPresentation::HighlightMatching, |d| {
                                    d.bg(rgb(0x1e3a8a))
                                })
                                .when(!selected && !(matches && root.controller.match_presentation == MatchPresentation::HighlightMatching), |d| d.bg(rgb(0x0f172a)))
                                .child(item.title)
                                .on_click(cx.listener(move |this, _, _, _| {
                                    this.controller.set_item_selection(item_id);
                                }))
                        },
                    ));

                div().flex().flex_col().gap_1().child(root_row).child(child_rows)
            }))
        }
        LibraryViewMode::GridByPublisher | LibraryViewMode::GridByProductType => {
            let sections = root.controller.grid_sections();
            div()
                .flex()
                .flex_col()
                .gap_3()
                .children(sections.into_iter().map(|section| {
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .bg(rgb(0x1f2937))
                                .rounded_sm()
                                .child(format!(
                                    "{} ({})",
                                    section.root_label,
                                    section.children.len()
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .gap_2()
                                .children(section.children.into_iter().map(|item| {
                                    let item_id = item.id;
                                    let selected = matches!(
                                        root.controller.selection,
                                        Some(Selection::Item(id)) if id == item_id
                                    );
                                    let matches = root.controller.is_item_match(&item);

                                    div()
                                        .id(("grid-item", item_id))
                                        .w_1_4()
                                        .p_2()
                                        .rounded_sm()
                                        .border_1()
                                        .border_color(rgb(0x374151))
                                        .cursor_pointer()
                                        .when(selected, |d| d.bg(rgb(0x334155)))
                                        .when(
                                            !selected
                                                && matches
                                                && root.controller.match_presentation
                                                    == MatchPresentation::HighlightMatching,
                                            |d| d.bg(rgb(0x1e3a8a)),
                                        )
                                        .when(
                                            !selected
                                                && !(matches
                                                    && root.controller.match_presentation
                                                        == MatchPresentation::HighlightMatching),
                                            |d| d.bg(rgb(0x0f172a)),
                                        )
                                        .child(
                                            div()
                                                .h_16()
                                                .rounded_sm()
                                                .bg(rgb(0x1e293b))
                                                .child("Thumbnail"),
                                        )
                                        .child(div().pt_2().child(item.title))
                                        .child(div().text_color(rgb(0x93c5fd)).child(format!(
                                            "{} | updated {}",
                                            item.product_type, item.updated_order
                                        )))
                                        .on_click(cx.listener(move |this, _, _, _| {
                                            this.controller.set_item_selection(item_id);
                                        }))
                                })),
                        )
                }))
        }
    };

    div()
        .w_3_5()
        .h_full()
        .p_2()
        .bg(rgb(0x111827))
        .rounded_md()
        .border_1()
        .border_color(rgb(0x374151))
        .flex()
        .flex_col()
        .gap_2()
        .child(if root.controller.renders_grid() {
            "Library grid"
        } else {
            "Library"
        })
        .child(left_content)
}

fn stable_root_id(label: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    label.hash(&mut hasher);
    hasher.finish()
}
