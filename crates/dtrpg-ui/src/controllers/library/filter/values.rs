//! Which filter values the user wants to see: the choices, the values the
//! catalog has, and what the settings page lists.

use std::collections::HashMap;

use dtrpg_core::data::filter_value_choices::FilterValueChoices;
use dtrpg_core::data::ui_preferences::UiPreferences;
use dtrpg_core::util::search::Field;

use super::super::*;

/// The fields whose values the user can show or hide, in the order the
/// settings page lists them.
pub const CHOOSABLE_FILTER_FIELDS: [Field; 3] = [Field::Format, Field::Genre, Field::Type];

/// One field's values for the settings page.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilterValueSection {
    /// The field the values belong to.
    pub field:  Field,
    /// The values that are shown, sorted alphabetically.
    pub shown:  Vec<String>,
    /// The values that are hidden, sorted alphabetically.
    pub hidden: Vec<String>,
}

impl LibraryController {
    /// The user's choices of which filter values to show.
    #[must_use]
    pub const fn filter_value_choices(&self) -> &FilterValueChoices {
        &self.filter_choices
    }

    /// The values of each choosable field, split into shown and hidden, for the
    /// settings page. A field the catalog has no values for is left out.
    #[must_use]
    pub fn filter_value_sections(&self) -> Vec<FilterValueSection> {
        CHOOSABLE_FILTER_FIELDS.into_iter()
                               .filter_map(|field| {
                                   let values = self.known_filter_values.get(&field)?;
                                   let category = field.category()?;
                                   let (shown, hidden) =
                                       self.filter_choices.split(category, values);
                                   Some(FilterValueSection { field,
                                                             shown,
                                                             hidden })
                               })
                               .collect()
    }

    /// Shows or hides `value` of `field`, remembers the choice, and redraws.
    pub fn set_filter_value_shown(&mut self, field: Field, value: &str, shown: bool,
                                  cx: &mut Context<Self>) {
        let Some(category) = field.category()
        else {
            return;
        };
        self.filter_choices.set_shown(category, value, shown);
        UiPreferences::load().save_filter_value_choices(&self.filter_choices);
        cx.emit(LibraryChanged);
    }

    /// Reads the values the catalog has for each choosable field. Reads the
    /// index, so it runs when the catalog changes, not while drawing.
    pub(crate) fn refresh_known_filter_values(&mut self) {
        self.known_filter_values =
            CHOOSABLE_FILTER_FIELDS.into_iter()
                                   .map(|field| (field, self.library.filter_values(field)))
                                   .filter(|(_, values)| !values.is_empty())
                                   .collect::<HashMap<_, _>>();
    }
}
