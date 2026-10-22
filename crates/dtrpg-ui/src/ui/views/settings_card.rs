//! The small pieces the settings pages share: a heading with a rule beside
//! it, and a bordered card whose rows are separated by lines.

use gpui::prelude::*;
use gpui::{Div, div, px};

use crate::data::theme::ColorTokens;
use crate::ui::widgets::small_caps_text;

/// A section heading in small caps, with a rule running out to the right edge.
pub(crate) fn section_heading(text: String, colors: &ColorTokens) -> impl IntoElement {
    div().flex()
         .items_center()
         .gap(px(10.0))
         .child(div().flex_none()
                     .text_sm()
                     .text_color(colors.text_secondary)
                     .child(small_caps_text(text)))
         .child(div().flex_1().h(px(1.0)).bg(colors.border))
}

/// An empty bordered card; its children are the rows, separated with
/// [`card_row`].
pub(crate) fn card(colors: &ColorTokens) -> Div {
    div().flex()
         .flex_col()
         .rounded(px(8.0))
         .border_1()
         .border_color(colors.border)
}

/// One row of a [`card`]: padded, with a line above it unless it is the first.
pub(crate) fn card_row(first: bool, colors: &ColorTokens) -> Div {
    let row = div().flex()
                   .items_center()
                   .gap(px(12.0))
                   .px(px(16.0))
                   .py(px(12.0));
    if first {
        row
    }
    else {
        row.border_t_1().border_color(colors.border)
    }
}
