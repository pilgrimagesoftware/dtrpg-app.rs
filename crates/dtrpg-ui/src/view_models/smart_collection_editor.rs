//! State of the smart collection editor: the collection being made or changed
//! and what the editor says about it.
//!
//! Pure data and rules, no I/O and no window, so the editor's behavior can be
//! tested directly. The text boxes and the live match count live in the view;
//! this holds the choices.

use dtrpg_core::data::smart_collection::{MatchMode, Operator, Rule, SmartCollection};
use dtrpg_core::util::search::{Field, SearchQuery};

/// The fields a rule can test, in the order the field menu lists them.
pub const RULE_FIELDS: [Field; 8] = [Field::Tag,
                                     Field::Publisher,
                                     Field::System,
                                     Field::Type,
                                     Field::Format,
                                     Field::Genre,
                                     Field::Availability,
                                     Field::Collection];

/// A smart collection as it is being edited.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SmartCollectionDraft {
    /// The collection, which has the id of the one being changed, or `0` for a
    /// new one.
    collection: SmartCollection,
}

impl SmartCollectionDraft {
    /// A draft for a new collection, with one empty rule to start on.
    #[must_use]
    pub fn new() -> Self {
        let mut draft = Self::default();
        draft.add_rule();
        draft
    }

    /// A draft that finds the members of the manual collection `name`: its
    /// name, and one rule, `collection is <name>`. Used to make a smart
    /// collection from a manual one. The smart collection follows the manual
    /// one, and more rules can narrow it.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::util::search::Field;
    /// use dtrpg_ui::view_models::smart_collection_editor::SmartCollectionDraft;
    ///
    /// let draft = SmartCollectionDraft::from_manual_collection("Solo play");
    /// assert_eq!(draft.name(), "Solo play");
    /// assert_eq!(draft.rules()[0].field, Field::Collection);
    /// assert_eq!(draft.rules()[0].value, "Solo play");
    /// ```
    #[must_use]
    pub fn from_manual_collection(name: &str) -> Self {
        let mut draft = Self::new();
        draft.set_name(name);
        draft.set_field(0, Field::Collection);
        draft.set_value(0, name);
        draft
    }

    /// A draft for changing `collection`.
    #[must_use]
    pub fn editing(collection: SmartCollection) -> Self {
        Self { collection }
    }

    /// A draft for a new collection that finds what `query` finds: one rule
    /// for each of its `field:value` terms. The free text of a search has no
    /// rule, so it is left out.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::util::search::SearchQuery;
    /// use dtrpg_ui::view_models::smart_collection_editor::SmartCollectionDraft;
    ///
    /// let draft = SmartCollectionDraft::from_search(&SearchQuery::parse("type:adventure -tag:solo"));
    /// assert_eq!(draft.rules().len(), 2);
    /// ```
    #[must_use]
    pub fn from_search(query: &SearchQuery) -> Self {
        let rules = query.terms()
                         .iter()
                         .map(|term| Rule { field:    term.field,
                                            operator: if term.negated {
                                                Operator::IsNot
                                            }
                                            else {
                                                Operator::Is
                                            },
                                            value:    term.value.clone(), })
                         .collect();
        let collection = SmartCollection { rules,
                                           ..SmartCollection::default() };
        if collection.rules.is_empty() {
            return Self::new();
        }
        Self { collection }
    }

    /// The collection as it stands.
    #[must_use]
    pub const fn collection(&self) -> &SmartCollection {
        &self.collection
    }

    /// The rules, in the order they are shown.
    #[must_use]
    pub fn rules(&self) -> &[Rule] {
        &self.collection.rules
    }

    /// The name typed so far.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.collection.name
    }

    /// Sets the name.
    pub fn set_name(&mut self, name: &str) {
        name.clone_into(&mut self.collection.name);
    }

    /// Whether every rule or any rule has to be met.
    #[must_use]
    pub const fn mode(&self) -> MatchMode {
        self.collection.mode
    }

    /// Sets whether every rule or any rule has to be met.
    pub const fn set_mode(&mut self, mode: MatchMode) {
        self.collection.mode = mode;
    }

    /// Adds an empty rule after the others.
    pub fn add_rule(&mut self) {
        self.collection.rules.push(Rule { field:    Field::Tag,
                                          operator: Operator::Is,
                                          value:    String::new(), });
    }

    /// Removes the rule at `index`. Does nothing if there is no such rule.
    pub fn remove_rule(&mut self, index: usize) {
        if index < self.collection.rules.len() {
            self.collection.rules.remove(index);
        }
    }

    /// Sets what the rule at `index` tests.
    pub fn set_field(&mut self, index: usize, field: Field) {
        if let Some(rule) = self.collection.rules.get_mut(index) {
            rule.field = field;
        }
    }

    /// Sets how the rule at `index` compares.
    pub fn set_operator(&mut self, index: usize, operator: Operator) {
        if let Some(rule) = self.collection.rules.get_mut(index) {
            rule.operator = operator;
        }
    }

    /// Sets the value the rule at `index` compares with.
    pub fn set_value(&mut self, index: usize, value: &str) {
        if let Some(rule) = self.collection.rules.get_mut(index) {
            value.clone_into(&mut rule.value);
        }
    }

    /// Whether the on-match switch for downloading new matches is on.
    #[must_use]
    pub const fn download_new_matches(&self) -> bool {
        self.collection.download_new_matches
    }

    /// Whether the on-match switch for mirroring as symlinks is on.
    #[must_use]
    pub const fn mirror_as_symlinks(&self) -> bool {
        self.collection.mirror_as_symlinks
    }

    /// Whether the collection can be saved: it has a name and at least one
    /// rule with a value.
    #[must_use]
    pub fn can_save(&self) -> bool {
        !self.collection.name.trim().is_empty()
        && self.collection.rules.iter().any(Rule::is_filled_in)
    }

    /// The search that finds the same items, as text, for the line at the
    /// foot of the editor; `None` when the search cannot say it.
    #[must_use]
    pub fn equivalent_query_text(&self) -> Option<String> {
        self.collection
            .equivalent_query()
            .map(|query| query.to_string())
    }

    /// The collection to save, with its name trimmed.
    #[must_use]
    pub fn into_collection(mut self) -> SmartCollection {
        self.collection.name = self.collection.name.trim().to_owned();
        self.collection
    }
}

#[cfg(test)]
mod tests;
