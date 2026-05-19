//! Right detail pane rendering for the library view.

use gpui::{IntoElement, div, prelude::*, rgb};

use super::root_view::LibraryRootView;

pub(crate) fn render_detail_pane(root: &LibraryRootView) -> impl IntoElement {
    let detail = root.controller.detail_lines();

    div()
        .w_2_5()
        .h_full()
        .p_2()
        .bg(rgb(0x0f172a))
        .rounded_md()
        .border_1()
        .border_color(rgb(0x334155))
        .flex()
        .flex_col()
        .gap_1()
        .child("Details")
        .children(detail.into_iter().map(|line| div().child(line)))
}
