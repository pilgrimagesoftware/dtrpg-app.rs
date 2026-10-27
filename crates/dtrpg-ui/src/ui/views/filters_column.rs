//! The filters column beside the catalog and the search box with its chips.
//!
//! Both only edit the search. A checked facet is a `field:value` term in the
//! search query, so the search box, its chips and the column always agree, and
//! a search typed by hand shows up as checked boxes. The box holds the free
//! text; the terms show as chips inside it.

use dtrpg_core::data::filter_value_choices::FilterValueChoices;
use dtrpg_core::data::index::FacetCount;
use dtrpg_core::library::Facets;
use dtrpg_core::util::search::{Field, SearchQuery, Term, accept_key, join};
use gpui::{
    App, Context, Entity, InteractiveElement, IntoElement, MouseButton, ParentElement,
    SharedString, Styled, Window, deferred, div, px,
};
use gpui_kit::component::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::input::{Escape, IndentInline, Input, InputState};
use gpui_kit::component::scroll::ScrollableElement as _;
use rust_i18n::t;

use crate::controllers::library::LibraryController;
use crate::data::theme::ColorTokens;
use crate::ui::views::smart_collection_editor_dialog::open_smart_collection_editor;
use crate::view_models::smart_collection_editor::SmartCollectionDraft;

/// The facets the column lists, in order, with the heading of each.
const FACET_SECTIONS: [(Field, &str); 7] = [(Field::Availability, "filters.availability"),
                                            (Field::System, "filters.system"),
                                            (Field::Type, "filters.type"),
                                            (Field::Location, "filters.location"),
                                            (Field::Format, "filters.format"),
                                            (Field::Genre, "filters.genre"),
                                            (Field::Tag, "filters.tags")];

/// Replaces the search box text with the free text of `query`, whose terms
/// show as chips, and pushes the whole query into the controller. `set_value`
/// does not emit `InputEvent::Change`.
fn set_search_text(input: &Entity<InputState>, controller: &Entity<LibraryController>,
                   query: &SearchQuery, window: &mut Window, cx: &mut App) {
    input.update(cx, |state, cx| {
             state.set_value(query.free_text(), window, cx)
         });
    let next = query.to_string();
    controller.update(cx, |ctrl, cx| ctrl.set_search_query(next, cx));
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
/// Publishers and collections have no facet; they are chosen in the sidebar.
fn counts_for(facets: &Facets, field: Field) -> &[FacetCount] {
    match field {
        Field::Availability => &facets.availability,
        Field::Type => &facets.types,
        Field::Format => &facets.formats,
        Field::Genre => &facets.genres,
        Field::System => &facets.systems,
        Field::Location => &facets.locations,
        Field::Tag => &facets.tags,
        Field::Publisher | Field::Collection => &[],
    }
}

/// The facets without the values the user hides. A hidden value still works
/// in a search and still shows as a chip when it is applied; it is only left
/// out of the column.
#[must_use]
pub fn without_hidden_values(facets: &Facets, choices: &FilterValueChoices) -> Facets {
    let kept = |field: Field, counts: &[FacetCount]| -> Vec<FacetCount> {
        counts.iter()
              .filter(|count| {
                  field.category()
                       .is_none_or(|category| choices.is_shown(category, &count.value))
              })
              .cloned()
              .collect()
    };
    Facets { availability: facets.availability.clone(),
             systems:      kept(Field::System, &facets.systems),
             types:        kept(Field::Type, &facets.types),
             formats:      kept(Field::Format, &facets.formats),
             genres:       kept(Field::Genre, &facets.genres),
             locations:    facets.locations.clone(),
             tags:         facets.tags.clone(), }
}

/// Renders one facet: a heading and a checkbox with a count for each value.
fn render_facet_section(field: Field, heading_key: &str, facets: &Facets, query: &SearchQuery,
                        search_input: &Entity<InputState>,
                        controller: &Entity<LibraryController>, colors: &ColorTokens)
                        -> Option<impl IntoElement + use<>> {
    let counts = counts_for(facets, field);
    if counts.is_empty() {
        return None;
    }
    let largest = counts.iter()
                        .map(|count| count.count)
                        .max()
                        .unwrap_or(1)
                        .max(1);
    let rows = counts.iter().enumerate().map(|(ix, count)| {
        let value = count.value.clone();
        let query = query.clone();
        let input = search_input.clone();
        let controller = controller.clone();
        // The bar is a share of the largest count in this list.
        let share = count.count as f32 / largest as f32;
        div().flex()
             .flex_col()
             .gap(px(2.0))
             .child(div().flex()
                         .items_center()
                         .justify_between()
                         .gap(px(8.0))
                         .child(Checkbox::new((field.keyword(), ix)).label(facet_label(field, count))
                                                                    .checked(query.asks_for(field, &value))
                                                                    .on_click(move |_, window, cx| {
                                                                        let next = query.toggled(field, &value);
                                                                        set_search_text(&input, &controller, &next, window, cx);
                                                                    }))
                         .child(div().text_xs()
                                     .text_color(colors.text_tertiary)
                                     .child(count.count.to_string())))
             .child(div().h(px(3.0))
                         .w_full()
                         .rounded(px(2.0))
                         .bg(colors.surface_alt)
                         .child(div().h_full()
                                     .w(gpui::relative(share))
                                     .rounded(px(2.0))
                                     .bg(colors.accent_soft)))
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
                             search_input: &Entity<InputState>,
                             controller: &Entity<LibraryController>, colors: &ColorTokens)
                             -> impl IntoElement + use<> {
    let sections = FACET_SECTIONS.iter().filter_map(|(field, heading_key)| {
                                            render_facet_section(*field,
                                                                 heading_key,
                                                                 facets,
                                                                 query,
                                                                 search_input,
                                                                 controller,
                                                                 colors)
                                        });
    div().id("filters-column")
         .w_full()
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

/// The filter key the word being typed in the search box could be completed
/// to, kept as an entity so the box redraws only when it changes.
#[derive(Default)]
pub struct SearchSuggestion {
    field: Option<Field>,
}

impl SearchSuggestion {
    /// The field suggested, if any.
    #[must_use]
    pub const fn field(&self) -> Option<Field> {
        self.field
    }

    /// Replaces the suggestion, redrawing only if it differs.
    pub fn set(&mut self, field: Option<Field>, cx: &mut Context<Self>) {
        if self.field != field {
            self.field = field;
            cx.notify();
        }
    }
}

/// Renders the row under the toolbar that holds the search box: the entry for
/// the free text with a chip inside it for each committed `field:value` term
/// of `chips`, each of which takes its term out of the search when pressed, and
/// a button to save the search as a smart collection while there are chips.
pub fn render_search_box(chips: &[Term], search_input: &Entity<InputState>,
                         controller: &Entity<LibraryController>,
                         suggestion: &Entity<SearchSuggestion>, colors: &ColorTokens, cx: &App)
                         -> impl IntoElement + use<> {
    let mut input = Input::new(search_input).w_full();
    let mut save_button = None;
    if !chips.is_empty() {
        let buttons = chips.iter().enumerate().map(|(ix, term)| {
            let term = term.clone();
            let all = chips.to_vec();
            let input = search_input.clone();
            let controller = controller.clone();
            Button::new(("filter-chip", ix)).outline()
                                            .compact()
                                            .label(term.to_string())
                                            .icon(IconName::Close)
                                            .tooltip(t!("filters.remove_chip_tooltip"))
                                            .on_click(move |_, _, cx| {
                                                let kept: Vec<Term> =
                                                    all.iter().filter(|other| **other != term)
                                                       .cloned()
                                                       .collect();
                                                let typed = input.read(cx).value().to_string();
                                                controller.update(cx, |ctrl, cx| {
                                                              ctrl.set_search_chips(kept, &typed, cx)
                                                          });
                                            })
        });
        let (save_chips, save_input, save_controller) =
            (chips.to_vec(), search_input.clone(), controller.clone());
        let save = Button::new("filter-chips-save").ghost()
                                                   .compact()
                                                   .icon(IconName::Plus)
                                                   .tooltip(t!("filters.save_as_smart_collection_tooltip"))
                                                   .on_click(move |_, window, cx| {
                                                       let typed =
                                                           save_input.read(cx).value().to_string();
                                                       let query = SearchQuery::parse(&join(&save_chips,
                                                                                            &typed));
                                                       open_smart_collection_editor(window,
                                                                                    cx,
                                                                                    save_controller.clone(),
                                                                                    SmartCollectionDraft::from_search(&query));
                                                   });
        input = input.prefix(div().flex()
                                  .items_center()
                                  .gap(px(4.0))
                                  .pl(px(4.0))
                                  .children(buttons));
        save_button = Some(save);
    }
    // Clear all takes the chips and the text, which the input's own clear
    // button (text only) would leave half done, so that one is not used.
    let has_text = !search_input.read(cx).value().is_empty();
    let clear_all =
        (has_text || !chips.is_empty()).then(|| {
                                           let (clear_input, clear_controller, clear_suggestion) =
                                               (search_input.clone(),
                                                controller.clone(),
                                                suggestion.clone());
                                           Button::new("search-clear-all").ghost()
                                           .compact()
                                           .icon(IconName::Close)
                                           .tooltip(t!("search.clear_all_tooltip"))
                                           .on_click(move |_, window, cx| {
                                               clear_input.update(cx, |state, cx| {
                                                              state.set_value("", window, cx)
                                                          });
                                               clear_controller.update(cx, |ctrl, cx| {
                                                                   ctrl.set_search_chips(Vec::new(),
                                                                                         "",
                                                                                         cx)
                                                               });
                                               clear_suggestion.update(cx, |state, cx| {
                                                                   state.set(None, cx)
                                                               });
                                           })
                                       });
    input = input.suffix(div().flex()
                              .items_center()
                              .gap(px(4.0))
                              .children(clear_all)
                              .children(save_button));
    let suggested = suggestion.read(cx).field();
    let (key_input, key_chips, key_controller, key_suggestion) =
        (search_input.clone(), chips.to_vec(), controller.clone(), suggestion.clone());
    let escape_suggestion = suggestion.clone();
    let (pop_input, pop_chips, pop_controller, pop_suggestion) =
        (search_input.clone(), chips.to_vec(), controller.clone(), suggestion.clone());
    // The completion popup is custom: gpui-kit's completion menu belongs to the
    // multi-line code editor and the search box is a single-line input. Tab
    // reaches the box as the input's unhandled `IndentInline` (a raw key
    // listener runs after the focus-next binding, so it is too late), and
    // Escape as the input's propagated `Escape`. Both are passed on when
    // there is nothing suggested. Right is left to the input: it moves the
    // caret and does not propagate.
    div().flex_none()
         .relative()
         .px(px(18.0))
         .py(px(8.0))
         .border_b_1()
         .border_color(colors.border)
         .bg(colors.surface)
         .on_action(move |_: &IndentInline, window, cx| {
             let Some(field) = key_suggestion.read(cx).field()
             else {
                 cx.propagate();
                 return;
             };
             accept_suggestion(&key_input,
                               &key_chips,
                               &key_controller,
                               &key_suggestion,
                               field,
                               window,
                               cx);
         })
         .on_action(move |_: &Escape, _, cx| {
             if escape_suggestion.read(cx).field().is_some() {
                 escape_suggestion.update(cx, |state, cx| state.set(None, cx));
             }
             else {
                 cx.propagate();
             }
         })
         .child(input)
         .children(suggested.map(|field| {
                                // `occlude` and the mouse-down handler keep the
                                // click from
                                // reaching the catalog drawn under the popup.
                                deferred(div().id("search-suggestion")
                                              .occlude()
                                              .cursor_pointer()
                                              .on_mouse_down(MouseButton::Left,
                                                             move |_, window, cx| {
                                                                 accept_suggestion(&pop_input,
                                                                                   &pop_chips,
                                                                                   &pop_controller,
                                                                                   &pop_suggestion,
                                                                                   field,
                                                                                   window,
                                                                                   cx);
                                                                 pop_input.update(cx, |state, cx| {
                                                                     state.focus(window, cx)
                                                                 });
                                                                 cx.stop_propagation();
                                                             })
                                              .absolute()
                                              .top(px(46.0))
                                              .left(px(18.0))
                                              .px(px(10.0))
                                              .py(px(6.0))
                                              .flex()
                                              .items_center()
                                              .gap(px(10.0))
                                              .rounded(px(6.0))
                                              .border_1()
                                              .border_color(colors.border)
                                              .bg(colors.surface)
                                              .shadow_md()
                                              .child(div().text_sm()
                                                          .text_color(colors.text_primary)
                                                          .child(format!("{}:", field.keyword())))
                                              .child(div().text_xs()
                                                          .text_color(colors.text_tertiary)
                                                          .child(t!("search.complete_hint"))))
                            }))
}

/// Completes the word being typed in the search box to `field`'s key, searches
/// for the result and clears the suggestion.
fn accept_suggestion(input: &Entity<InputState>, chips: &[Term],
                     controller: &Entity<LibraryController>,
                     suggestion: &Entity<SearchSuggestion>, field: Field, window: &mut Window,
                     cx: &mut App) {
    let typed = input.read(cx).value().to_string();
    let completed = accept_key(&typed, field);
    input.update(cx, |state, cx| {
             state.set_value(completed.as_str(), window, cx)
         });
    let query = join(chips, &completed);
    controller.update(cx, |ctrl, cx| ctrl.set_search_query_debounced(query, cx));
    suggestion.update(cx, |state, cx| state.set(None, cx));
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
                    "filters.format",
                    "filters.genre",
                    "filters.tags"]
        {
            assert_ne!(t!(key).to_string(), key, "{key} has no text");
        }
    }

    #[test]
    fn hidden_values_are_left_out_of_the_column() {
        let counts = |names: &[&str]| -> Vec<FacetCount> {
            names.iter()
                 .map(|name| FacetCount { value: (*name).to_owned(),
                                          count: 1, })
                 .collect()
        };
        let facets = Facets { formats: counts(&["PDF", "Creation Method"]),
                              genres: counts(&["Horror"]),
                              ..Facets::default() };
        let mut choices = FilterValueChoices::default();
        choices.set_shown("Genre", "Horror", false);

        let shown = without_hidden_values(&facets, &choices);

        assert_eq!(shown.formats,
                   counts(&["PDF"]),
                   "the app hides Creation Method by default");
        assert!(shown.genres.is_empty());
        choices.set_shown("Format", "Creation Method", true);
        assert_eq!(without_hidden_values(&facets, &choices).formats.len(), 2);
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
