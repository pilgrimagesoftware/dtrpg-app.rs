//! The "Filter values" part of Settings > Advanced: a chip for each value of
//! each filter field (formats, genres, product types), so the user can choose
//! which ones the filters column lists.
//!
//! Shown values are lit and listed first, hidden ones are ghosted and listed
//! after, and each group is sorted alphabetically. Clicking a chip moves it to
//! the other group. The chip is the pre-made `Tag` widget.

use dtrpg_core::util::search::Field;
use gpui::prelude::*;
use gpui::{App, Entity, SharedString, div, px};
use gpui_kit::component::tag::Tag;
use gpui_kit::component::tooltip::Tooltip;
use rust_i18n::t;

use crate::controllers::library::{FilterValueSection, LibraryController};
use crate::data::theme::ColorTokens;
use crate::ui::widgets::selectable_note;

/// How faint a hidden value's chip is drawn. Faint enough to read as off, not
/// so faint that it is hard to find.
const GHOSTED_OPACITY: f32 = 0.7;

/// The height of one row of chips with the gap below it, in pixels.
const CHIP_ROW_HEIGHT: f32 = 30.0;

/// How many rows of chips show before the list scrolls. The half row left
/// showing at the bottom tells the user there is more.
const VISIBLE_CHIP_ROWS: f32 = 3.5;

/// The heading text of a field's group of chips.
fn field_heading(field: Field) -> String {
    match field {
        Field::Format => t!("settings.filter_values_format"),
        Field::Genre => t!("settings.filter_values_genre"),
        Field::Type => t!("settings.filter_values_type"),
        Field::Tag | Field::Publisher | Field::System | Field::Availability => {
            return field.keyword().to_owned();
        }
    }.to_string()
}

/// One value's chip. A click shows the value if it is hidden and hides it if it
/// is shown.
fn render_chip(field: Field, index: usize, value: &str, shown: bool,
               library: &Entity<LibraryController>)
               -> impl IntoElement {
    let library = library.clone();
    let name = value.to_owned();
    let tooltip = if shown {
        t!("settings.filter_values_hide_tooltip", name = value).to_string()
    }
    else {
        t!("settings.filter_values_show_tooltip", name = value).to_string()
    };
    let tag = if shown {
        Tag::secondary()
    }
    else {
        Tag::secondary().outline()
    };
    div().id((SharedString::from(format!("filter-value-{}", field.keyword())), index))
         .cursor_pointer()
         .when(!shown, |chip| chip.opacity(GHOSTED_OPACITY))
         .tooltip(move |window, cx| Tooltip::new(tooltip.clone()).build(window, cx))
         .on_click(move |_, _, cx: &mut App| {
             library.update(cx, |ctrl, cx| {
                        ctrl.set_filter_value_shown(field, &name, !shown, cx)
                    });
         })
         .child(tag.rounded_full().child(value.to_owned()))
}

/// One field's chips: the shown values, then the hidden ones.
fn render_section(section: &FilterValueSection, library: &Entity<LibraryController>,
                  colors: &ColorTokens)
                  -> impl IntoElement {
    let shown =
        section.shown
               .iter()
               .enumerate()
               .map(|(ix, value)| render_chip(section.field, ix, value, true, library));
    let offset = section.shown.len();
    let hidden = section.hidden.iter().enumerate().map(|(ix, value)| {
                                                      render_chip(section.field,
                                                                  offset + ix,
                                                                  value,
                                                                  false,
                                                                  library)
                                                  });
    div().flex()
         .flex_col()
         .gap(px(8.0))
         .child(div().text_xs()
                     .text_color(colors.text_secondary)
                     .child(field_heading(section.field)))
         .child(div().id(SharedString::from(format!("filter-values-{}", section.field.keyword())))
                     .max_h(px(CHIP_ROW_HEIGHT * VISIBLE_CHIP_ROWS))
                     .overflow_y_scroll()
                     .child(div().flex()
                                 .flex_wrap()
                                 .gap(px(6.0))
                                 .children(shown)
                                 .children(hidden)))
}

/// Renders the filter values section, or nothing while the catalog has no
/// filter values to choose from.
pub fn render_filter_values_section(sections: &[FilterValueSection],
                                    library: &Entity<LibraryController>, colors: &ColorTokens)
                                    -> Option<impl IntoElement + use<>> {
    if sections.is_empty() {
        return None;
    }
    Some(div().flex()
              .flex_col()
              .gap(px(12.0))
              .child(div().text_sm()
                          .font_weight(gpui::FontWeight::SEMIBOLD)
                          .text_color(colors.text_primary)
                          .child(t!("settings.filter_values_title").to_string()))
              .child(selectable_note("filter-values-description",
                                     t!("settings.filter_values_description").to_string(),
                                     colors))
              .children(sections.iter()
                                .map(|section| render_section(section, library, colors))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_texts_resolve() {
        for key in ["settings.filter_values_title",
                    "settings.filter_values_description",
                    "settings.filter_values_format",
                    "settings.filter_values_genre",
                    "settings.filter_values_type"]
        {
            assert_ne!(t!(key).to_string(), key, "{key} has no text");
        }
    }

    #[test]
    fn every_choosable_field_has_a_heading() {
        for field in crate::controllers::library::CHOOSABLE_FILTER_FIELDS {
            assert_ne!(field_heading(field), field.keyword());
        }
    }
}
