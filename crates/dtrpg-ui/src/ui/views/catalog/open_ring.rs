//! The ring drawn around the card or row whose item is open in the inspector,
//! so the selection can be found again in a grid of covers.

use gpui::Div;
use gpui::{Styled, div, px};

use crate::data::theme::ColorTokens;

/// A ring and a faint wash over the card or row it is added to. It is laid over
/// the item without taking any room, so the layout does not move when the item
/// is opened or closed, and it lets clicks through to the item.
pub(super) fn open_ring(colors: &ColorTokens) -> Div {
    div().absolute()
         .top_0()
         .left_0()
         .right_0()
         .bottom_0()
         .rounded(px(6.0))
         .border_2()
         .border_color(colors.accent)
         .bg(colors.accent_soft)
}
