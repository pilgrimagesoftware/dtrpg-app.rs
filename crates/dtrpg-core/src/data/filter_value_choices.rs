//! Which filter values the user wants to see.
//!
//! DriveThruRPG's filters include values that mean a lot to some people and
//! nothing to others (`Human-Created Without AI`, say). The app ships a short
//! list of values it hides to begin with, and the user can show or hide any
//! value. Only what the user changed from those defaults is kept, so a value
//! the user never touched follows the app's defaults, and the defaults can
//! improve later without undoing anyone's choice.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::data::product_filter::CATEGORY_FORMAT;

/// Values the app hides until the user shows them, by category. Names are
/// compared ignoring case.
const HIDDEN_BY_DEFAULT: [(&str, &[&str]); 1] =
    [(CATEGORY_FORMAT, &["Human-Created Without AI", "Creation Method"])];

/// Whether the app hides `value` of `category` when the user has not chosen.
///
/// # Examples
///
/// ```
/// use dtrpg_core::data::filter_value_choices::hidden_by_default;
///
/// assert!(hidden_by_default("Format", "human-created without ai"));
/// assert!(!hidden_by_default("Format", "PDF"));
/// ```
#[must_use]
pub fn hidden_by_default(category: &str, value: &str) -> bool {
    HIDDEN_BY_DEFAULT.iter()
                     .filter(|(name, _)| name.eq_ignore_ascii_case(category))
                     .flat_map(|(_, values)| values.iter())
                     .any(|hidden| hidden.eq_ignore_ascii_case(value))
}

/// What the user changed in one category, by lower-cased value.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct CategoryChoices {
    /// Values the user shows that the app hides by default.
    #[serde(default)]
    shown:  BTreeSet<String>,
    /// Values the user hides that the app shows by default.
    #[serde(default)]
    hidden: BTreeSet<String>,
}

impl CategoryChoices {
    fn is_empty(&self) -> bool {
        self.shown.is_empty() && self.hidden.is_empty()
    }
}

/// The user's choices of which filter values to show, kept in the UI
/// preferences.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FilterValueChoices {
    /// The changes from the defaults, by lower-cased category.
    #[serde(default)]
    changes: BTreeMap<String, CategoryChoices>,
}

impl FilterValueChoices {
    /// Whether `value` of `category` is shown: the user's choice if they made
    /// one, the app's default if not.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::data::filter_value_choices::FilterValueChoices;
    ///
    /// let mut choices = FilterValueChoices::default();
    /// assert!(choices.is_shown("Format", "PDF"));
    /// assert!(!choices.is_shown("Format", "Creation Method"));
    ///
    /// choices.set_shown("Format", "PDF", false);
    /// choices.set_shown("Format", "Creation Method", true);
    /// assert!(!choices.is_shown("Format", "PDF"));
    /// assert!(choices.is_shown("Format", "Creation Method"));
    /// ```
    #[must_use]
    pub fn is_shown(&self, category: &str, value: &str) -> bool {
        let value_key = value.to_lowercase();
        let changes = self.changes.get(&category.to_lowercase());
        if changes.is_some_and(|changes| changes.hidden.contains(&value_key)) {
            return false;
        }
        if changes.is_some_and(|changes| changes.shown.contains(&value_key)) {
            return true;
        }
        !hidden_by_default(category, value)
    }

    /// Shows or hides `value` of `category`. Choosing what the app would do
    /// anyway keeps nothing, so the saved choices stay the changes only.
    pub fn set_shown(&mut self, category: &str, value: &str, shown: bool) {
        let category_key = category.to_lowercase();
        let value_key = value.to_lowercase();
        let changes = self.changes.entry(category_key.clone()).or_default();
        changes.shown.remove(&value_key);
        changes.hidden.remove(&value_key);
        let shown_by_default = !hidden_by_default(category, value);
        if shown != shown_by_default {
            if shown {
                changes.shown.insert(value_key);
            }
            else {
                changes.hidden.insert(value_key);
            }
        }
        if changes.is_empty() {
            self.changes.remove(&category_key);
        }
    }

    /// Splits `values` of `category` into the ones shown and the ones hidden,
    /// each sorted alphabetically ignoring case, with a value listed once.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::data::filter_value_choices::FilterValueChoices;
    ///
    /// let values = ["Print", "PDF", "Creation Method", "audio"].map(String::from);
    /// let (shown, hidden) = FilterValueChoices::default().split("Format", &values);
    /// assert_eq!(shown, ["audio", "PDF", "Print"]);
    /// assert_eq!(hidden, ["Creation Method"]);
    /// ```
    #[must_use]
    pub fn split(&self, category: &str, values: &[String]) -> (Vec<String>, Vec<String>) {
        let mut sorted: Vec<&String> = values.iter().collect();
        sorted.sort_by_key(|value| value.to_lowercase());
        sorted.dedup_by_key(|value| value.to_lowercase());
        let (shown, hidden): (Vec<&String>, Vec<&String>) =
            sorted.into_iter()
                  .partition(|value| self.is_shown(category, value));
        (shown.into_iter().cloned().collect(), hidden.into_iter().cloned().collect())
    }
}

#[cfg(test)]
mod tests;
