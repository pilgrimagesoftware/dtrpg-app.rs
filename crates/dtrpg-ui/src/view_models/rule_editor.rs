//! The placement rule editor of the Storage page: what kind of rule is being
//! added or changed, the text of its value, and how that text turns into a
//! rule and into suggestions. No window and no I/O, so it can be tested
//! directly.

use std::collections::HashMap;
use std::sync::Arc;

use dtrpg_core::data::storage::placement::PlacementCondition;
use gpui::Entity;
use gpui_kit::component::input::InputState;

/// The most suggestions kept for the popup under the value field. The popup
/// scrolls, so it holds more than it shows.
pub const MAX_SUGGESTIONS: usize = 20;

/// How many suggestions the popup shows before it scrolls.
pub const VISIBLE_SUGGESTIONS: usize = 6;

/// Which kind of condition a rule has.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleEditorType {
    /// Titles from one publisher.
    Publisher,
    /// Titles in one collection.
    Collection,
    /// Files of at least some size.
    Size,
}

impl RuleEditorType {
    /// Every kind, in the order the type menu lists them.
    pub const ALL: [Self; 3] = [Self::Publisher, Self::Collection, Self::Size];

    /// The kind of `condition`.
    #[must_use]
    pub const fn of(condition: &PlacementCondition) -> Self {
        match condition {
            PlacementCondition::Publisher(_) => Self::Publisher,
            PlacementCondition::Collection(_) => Self::Collection,
            PlacementCondition::SizeAtLeastMb(_) => Self::Size,
        }
    }
}

/// The editor as the Storage page shows it. One editor serves both adding a
/// rule and changing one, and shows in place of the row it changes.
#[derive(Clone, Debug)]
pub struct RuleEditor {
    /// Whether the editor is showing.
    pub open:           bool,
    /// The index of the rule being changed, or `None` when adding one.
    pub editing:        Option<usize>,
    /// The kind of rule being edited.
    pub kind:           RuleEditorType,
    /// The location the rule sends downloads to.
    pub target_root_id: u32,
    /// The text of the value field, kept as it is typed.
    pub value:          String,
    /// The value field, once the root view attaches it.
    pub input:          Option<Entity<InputState>>,
}

impl Default for RuleEditor {
    fn default() -> Self {
        Self { open:           false,
               editing:        None,
               kind:           RuleEditorType::Publisher,
               target_root_id: 0,
               value:          String::new(),
               input:          None, }
    }
}

/// The text that stands for `condition` in the value field: the publisher's
/// name, the collection's name (or its number when the name is not known), or
/// the size in megabytes.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use dtrpg_core::data::storage::placement::PlacementCondition;
/// use dtrpg_ui::view_models::rule_editor::value_text;
///
/// let text = value_text(&PlacementCondition::SizeAtLeastMb(250.0), &HashMap::new());
/// assert_eq!(text, "250");
/// ```
#[must_use]
pub fn value_text(condition: &PlacementCondition, collection_names: &HashMap<u64, Arc<str>>)
                  -> String {
    match condition {
        PlacementCondition::Publisher(name) => name.clone(),
        PlacementCondition::Collection(id) => {
            collection_names.get(id)
                            .map_or_else(|| id.to_string(), ToString::to_string)
        }
        PlacementCondition::SizeAtLeastMb(mb) => mb.to_string(),
    }
}

/// The condition that `text` describes for a rule of `kind`, or `None` when it
/// describes none: an empty publisher, a collection that is not there, or a
/// size that is not a number of at least zero. A collection can be given by
/// its name (capital letters do not matter) or by its number.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use dtrpg_core::data::storage::placement::PlacementCondition;
/// use dtrpg_ui::view_models::rule_editor::{RuleEditorType, condition_from_text};
///
/// let condition = condition_from_text(RuleEditorType::Publisher, " Chaosium ", &HashMap::new());
/// assert_eq!(condition,
///            Some(PlacementCondition::Publisher("Chaosium".to_owned())));
/// ```
#[must_use]
pub fn condition_from_text(kind: RuleEditorType, text: &str,
                           collection_names: &HashMap<u64, Arc<str>>)
                           -> Option<PlacementCondition> {
    let text = text.trim();
    match kind {
        RuleEditorType::Publisher => {
            (!text.is_empty()).then(|| PlacementCondition::Publisher(text.to_owned()))
        }
        RuleEditorType::Collection => {
            let lowered = text.to_lowercase();
            collection_names.iter()
                            .find(|(_, name)| name.to_lowercase() == lowered)
                            .map(|(id, _)| *id)
                            .or_else(|| text.parse::<u64>().ok())
                            .map(PlacementCondition::Collection)
        }
        RuleEditorType::Size => text.parse::<f64>()
                                    .ok()
                                    .filter(|mb| mb.is_finite() && *mb >= 0.0)
                                    .map(PlacementCondition::SizeAtLeastMb),
    }
}

/// The names to offer under the value field for a rule of `kind` while `text`
/// is typed: the publishers or collections whose name starts with the text,
/// then those that contain it, each group in alphabetical order, at most
/// [`MAX_SUGGESTIONS`] of them. A name that
/// is already exactly the text is not offered, and a size has no suggestions.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use dtrpg_ui::view_models::rule_editor::{RuleEditorType, suggestions};
///
/// let publishers = vec!["Chaosium".to_owned(), "Free League".to_owned()];
/// let names = suggestions(RuleEditorType::Publisher,
///                         "cha",
///                         &publishers,
///                         &HashMap::new());
/// assert_eq!(names, vec!["Chaosium".to_owned()]);
/// ```
#[must_use]
pub fn suggestions(kind: RuleEditorType, text: &str, publishers: &[String],
                   collection_names: &HashMap<u64, Arc<str>>)
                   -> Vec<String> {
    let typed = text.trim().to_lowercase();
    if typed.is_empty() {
        return Vec::new();
    }
    let mut names: Vec<String> = match kind {
        RuleEditorType::Publisher => publishers.to_vec(),
        RuleEditorType::Collection => collection_names.values().map(ToString::to_string).collect(),
        RuleEditorType::Size => return Vec::new(),
    };
    names.sort_by_key(|name| name.to_lowercase());
    names.dedup();
    let (starting, containing): (Vec<String>, Vec<String>) =
        names.into_iter()
             .filter(|name| name.to_lowercase() != typed && name.to_lowercase().contains(&typed))
             .partition(|name| name.to_lowercase().starts_with(&typed));
    starting.into_iter()
            .chain(containing)
            .take(MAX_SUGGESTIONS)
            .collect()
}

#[cfg(test)]
mod tests;
