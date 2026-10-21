//! The filters column beside the catalog and the row of chips under the
//! toolbar.
//!
//! Both only edit the search box. A checked facet is a `field:value` term in
//! the search text, so the search box, the chips and the column always agree,
//! and a search typed by hand shows up as checked boxes.

use dtrpg_core::data::index::FacetCount;
use dtrpg_core::library::Facets;
use dtrpg_core::util::search::{Field, SearchQuery};
use gpui::{
    App, Entity, InteractiveElement, IntoElement, ParentElement, SharedString, Styled, Window, div,
    px,
};
use gpui_kit::component::IconName;
use gpui_kit::component::button::Button;
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::input::InputState;
use gpui_kit::component::scroll::ScrollableElement as _;
use rust_i18n::t;

use crate::data::theme::ColorTokens;

/// The width of the filters column.
const FILTERS_COLUMN_WIDTH: f32 = 220.0;

/// The facets the column lists, in order, with the heading of each.
const FACET_SECTIONS: [(Field, &str); 4] = [(Field::Availability, "filters.availability"),
                                            (Field::Type, "filters.type"),
                                            (Field::System, "filters.system"),
                                            (Field::Tag, "filters.tags")];

/// Replaces the search box text with `query`, which the box reports to the
/// controller like any typing.
fn set_search_text(input: &Entity<InputState>, query: &SearchQuery, window: &mut Window,
                   cx: &mut App) {
    input.update(cx, |state, cx| {
             state.set_value(query.to_string(), window, cx)
         });
}

/// How a facet value is shown. Availability values are words the user did not
/// choose, so they are translated; the rest are shown as the catalog has them.
fn facet_label(field: Field, count: &FacetCount) -> SharedString {
    if field != Field::Availability {
        return count.value.clone().into();
    }
    match count.value.as_str() {
        "downloaded" => t!("filters.downloaded").to_string().into(),
        "cloud" => t!("filters.cloud").to_string().into(),
        other => other.to_owned().into(),
    }
}

/// The counts of `field` in `facets`.
/// Publishers have no facet; they are chosen in the sidebar.
fn counts_for(facets: &Facets, field: Field) -> &[FacetCount] {
    match field {
        Field::Availability => &facets.availability,
        Field::Type => &facets.types,
        Field::System => &facets.systems,
        Field::Tag => &facets.tags,
        Field::Publisher => &[],
    }
}

/// Renders one facet: a heading and a checkbox with a count for each value.
fn render_facet_section(field: Field, heading_key: &str, facets: &Facets, query: &SearchQuery,
                        search_input: &Entity<InputState>, colors: &ColorTokens)
                        -> Option<impl IntoElement + use<>> {
    let counts = counts_for(facets, field);
    if counts.is_empty() {
        return None;
    }
    let rows = counts.iter().enumerate().map(|(ix, count)| {
        let value = count.value.clone();
        let query = query.clone();
        let input = search_input.clone();
        div().flex()
             .items_center()
             .justify_between()
             .gap(px(8.0))
             .child(Checkbox::new((field.keyword(), ix)).label(facet_label(field, count))
                                                        .checked(query.asks_for(field, &value))
                                                        .on_click(move |_, window, cx| {
                                                            let next = query.toggled(field, &value);
                                                            set_search_text(&input, &next, window, cx);
                                                        }))
             .child(div().text_xs()
                         .text_color(colors.text_tertiary)
                         .child(count.count.to_string()))
    });
    Some(div().flex()
              .flex_col()
              .gap(px(6.0))
              .child(div().text_xs()
                          .text_color(colors.text_secondary)
                          .child(t!(heading_key).to_string().to_uppercase()))
              .children(rows))
}

/// Renders the filters column: the facets of the items on show, each with a
/// count, ready to be checked.
pub fn render_filters_column(facets: &Facets, query: &SearchQuery,
                             search_input: &Entity<InputState>, colors: &ColorTokens)
                             -> impl IntoElement + use<> {
    let sections = FACET_SECTIONS.iter().filter_map(|(field, heading_key)| {
                                            render_facet_section(*field,
                                                                 heading_key,
                                                                 facets,
                                                                 query,
                                                                 search_input,
                                                                 colors)
                                        });
    div().id("filters-column")
         .w(px(FILTERS_COLUMN_WIDTH))
         .flex_none()
         .h_full()
         .border_r_1()
         .border_color(colors.border)
         .bg(colors.surface)
         .child(div().id("filters-column-body")
                     .size_full()
                     .overflow_y_scrollbar()
                     .child(div().flex()
                                 .flex_col()
                                 .gap(px(16.0))
                                 .p(px(14.0))
                                 .children(sections)))
}

/// Renders the chips for the `field:value` terms of the search, each of which
/// takes its term out of the search when pressed. Nothing is rendered while
/// the search has no terms.
pub fn render_filter_chips(query: &SearchQuery, search_input: &Entity<InputState>,
                           colors: &ColorTokens)
                           -> Option<impl IntoElement + use<>> {
    if query.terms().is_empty() {
        return None;
    }
    let chips = query.terms().iter().enumerate().map(|(ix, term)| {
                                                    let term = term.clone();
                                                    let query = query.clone();
                                                    let input = search_input.clone();
                                                    Button::new(("filter-chip", ix)).outline()
                                        .compact()
                                        .label(term.to_string())
                                        .icon(IconName::Close)
                                        .tooltip(t!("filters.remove_chip_tooltip"))
                                        .on_click(move |_, window, cx| {
                                            let next = query.without_term(&term);
                                            set_search_text(&input, &next, window, cx);
                                        })
                                                });
    Some(div().flex()
              .flex_wrap()
              .items_center()
              .gap(px(6.0))
              .px(px(18.0))
              .py(px(8.0))
              .border_b_1()
              .border_color(colors.border)
              .bg(colors.surface)
              .children(chips))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_translated_texts_resolve() {
        for key in ["filters.toggle_tooltip",
                    "filters.remove_chip_tooltip",
                    "filters.downloaded",
                    "filters.cloud",
                    "filters.availability",
                    "filters.type",
                    "filters.system",
                    "filters.tags"]
        {
            assert_ne!(t!(key).to_string(), key, "{key} has no text");
        }
    }

    #[test]
    fn availability_values_are_translated_and_others_are_kept() {
        let cloud = FacetCount { value: "cloud".to_owned(),
                                 count: 1, };
        let adventure = FacetCount { value: "Adventure".to_owned(),
                                     count: 1, };
        assert_ne!(facet_label(Field::Availability, &cloud).to_string(),
                   "cloud");
        assert_eq!(facet_label(Field::Type, &adventure).to_string(),
                   "Adventure");
    }
}
