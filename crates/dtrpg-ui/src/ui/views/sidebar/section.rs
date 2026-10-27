//! A collapsible sidebar section whose open state belongs to the controller.
//!
//! `SidebarMenuItem` keeps its own open state, and its caret button changes
//! that state without calling the item's click handler, so a section built from
//! it cannot save that it was collapsed. This section is told whether it is
//! open, and reports each click on its header, as the Collections section does.

use gpui::prelude::*;
use gpui::{AnyElement, App, ElementId, IntoElement, SharedString, Window, div};
use gpui_kit::component::sidebar::{SidebarItem, SidebarMenuItem};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{ActiveTheme, Collapsible, Sizable as _};
use gpui_kit::component::{Icon, IconName};
use rust_i18n::t;

use super::view::*;

/// The caret at the right of a section header, which points down while the
/// section is open and right while it is closed. The header itself takes the
/// click; the caret has a tooltip because it is only an image.
pub(super) fn section_caret(id: &SharedString, open: bool) -> impl IntoElement {
    let (icon, tip) = if open {
        (IconName::ChevronDown, t!("sidebar.section_hide"))
    }
    else {
        (IconName::ChevronRight, t!("sidebar.section_show"))
    };
    let tip = tip.to_string();
    div().id(SharedString::from(format!("{id}-caret")))
         .flex_none()
         .child(Icon::new(icon).small())
         .tooltip(move |window, cx| Tooltip::new(tip.clone()).build(window, cx))
}

/// A header over a list of rows. The header shows `title` and the `suffix`, and
/// a click on it calls `on_toggle`; the rows show only while `open`.
#[derive(Clone)]
pub(super) struct ControlledSection {
    /// The header's text.
    pub(super) title:     SharedString,
    /// Whether the rows show.
    pub(super) open:      bool,
    /// The header's trailing element, such as a count and a search button.
    pub(super) suffix:    CollectionsSuffixFn,
    /// Called when the header is clicked, to flip and save the open state.
    pub(super) on_toggle: CollectionsToggleFn,
    /// The rows under the header.
    pub(super) items:     Vec<SidebarMenuItem>,
}

impl Collapsible for ControlledSection {
    // The app's `Sidebar` never collapses to the icon-only rail.
    fn collapsed(self, _collapsed: bool) -> Self {
        self
    }

    fn is_collapsed(&self) -> bool {
        false
    }
}

impl SidebarItem for ControlledSection {
    fn render(self, id: impl Into<ElementId>, window: &mut Window, cx: &mut App)
              -> impl IntoElement {
        let id = id.into();
        let on_toggle = self.on_toggle.clone();
        let suffix = (self.suffix)(window, cx);
        let open = self.open;

        let header_id = SharedString::from(format!("{id:?}-header"));
        let header =
            div().id(header_id.clone())
                 .w_full()
                 .flex()
                 .items_center()
                 .overflow_x_hidden()
                 .flex_shrink_0()
                 .p_2()
                 .gap_x_2()
                 .rounded(cx.theme().radius)
                 .text_sm()
                 .h_7()
                 .hover(|this| {
                     this.bg(cx.theme().sidebar_accent.opacity(0.8))
                         .text_color(cx.theme().sidebar_accent_foreground)
                 })
                 .on_click(move |_, window, cx| on_toggle(window, cx))
                 .child(div().flex_1()
                             .flex()
                             .items_center()
                             .justify_between()
                             .gap_x_2()
                             .overflow_x_hidden()
                             .child(div().flex_1().overflow_x_hidden().child(self.title.clone()))
                             .child(suffix))
                 .child(section_caret(&header_id, open));

        let rows: Vec<AnyElement> = self.items
                                        .into_iter()
                                        .enumerate()
                                        .map(|(ix, item)| {
                                            item.render(format!("{id:?}-item-{ix}"), window, cx)
                                                .into_any_element()
                                        })
                                        .collect();

        div().id(id)
             .w_full()
             .flex()
             .flex_col()
             .child(header)
             .when(open, |this| {
                 this.child(div().id("submenu")
                                 .border_l_1()
                                 .border_color(cx.theme().sidebar_border)
                                 .flex()
                                 .flex_col()
                                 .gap_1()
                                 .ml_3p5()
                                 .pl_2p5()
                                 .py_0p5()
                                 .children(rows))
             })
    }
}

#[cfg(test)]
mod tests {
    use rust_i18n::t;

    #[test]
    fn the_caret_tooltips_resolve() {
        for key in ["sidebar.section_hide", "sidebar.section_show"] {
            assert_ne!(t!(key).to_string(), key, "{key} has no text");
        }
    }
}
