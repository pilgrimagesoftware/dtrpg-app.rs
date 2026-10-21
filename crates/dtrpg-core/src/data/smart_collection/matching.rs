//! Which items belong to a smart collection, and the search that says the same.

use std::sync::Arc;

use super::model::{MatchMode, Operator, Rule, SmartCollection};
use crate::data::library::LibraryItem;
use crate::util::search::{Field, SearchQuery, Term};

impl Rule {
    /// Whether the rule has a value to compare with.
    #[must_use]
    pub fn is_filled_in(&self) -> bool {
        !self.value.trim().is_empty()
    }

    /// The rule as a search term, which is how the search compares values.
    fn term(&self) -> Term {
        Term { field:   self.field,
               value:   self.value.trim().to_lowercase(),
               negated: self.operator == Operator::IsNot, }
    }

    /// Whether `item`, which has the tags `tags`, meets the rule.
    #[must_use]
    pub fn holds_for(&self, item: &LibraryItem, tags: &[Arc<str>]) -> bool {
        let term = self.term();
        term.is_held_by(item, tags) != term.negated
    }
}

impl SmartCollection {
    /// The rules that have a value. The others are still being typed.
    fn active_rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter().filter(|rule| rule.is_filled_in())
    }

    /// Whether `item`, which has the tags `tags`, belongs to the collection.
    ///
    /// A collection with no rule to go by has no members, so a new collection
    /// does not start out holding the whole library.
    #[must_use]
    pub fn contains(&self, item: &LibraryItem, tags: &[Arc<str>]) -> bool {
        let mut rules = self.active_rules().peekable();
        if rules.peek().is_none() {
            return false;
        }
        match self.mode {
            MatchMode::All => rules.all(|rule| rule.holds_for(item, tags)),
            MatchMode::Any => rules.any(|rule| rule.holds_for(item, tags)),
        }
    }

    /// The search that finds the same items, or `None` when the search cannot
    /// say it.
    ///
    /// The search combines terms differently from the rules: terms for one
    /// field (other than tags) are alternatives, and everything else must all
    /// hold. So the search matches the rules only when no two positive rules
    /// name the same field in "all" mode, and when "any" mode has a single
    /// rule or only positive rules for one field (other than tags).
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::data::smart_collection::{MatchMode, Operator, Rule, SmartCollection};
    /// use dtrpg_core::util::search::Field;
    ///
    /// let rule = |field, operator, value: &str| Rule { field,
    ///                                                  operator,
    ///                                                  value: value.to_owned() };
    /// let collection = SmartCollection { mode: MatchMode::All,
    ///                                    rules: vec![rule(Field::Type, Operator::Is, "Adventure"),
    ///                                                rule(Field::Tag, Operator::IsNot, "Running")],
    ///                                    ..SmartCollection::default() };
    /// let query = collection.equivalent_query()
    ///                       .expect("the search can say it");
    /// assert_eq!(query.to_string(), "type:adventure -tag:running");
    /// ```
    #[must_use]
    pub fn equivalent_query(&self) -> Option<SearchQuery> {
        let rules: Vec<&Rule> = self.active_rules().collect();
        if rules.is_empty() {
            return None;
        }
        let expressible = match self.mode {
            MatchMode::All => no_field_repeats_positively(&rules),
            MatchMode::Any => rules.len() == 1 || all_positive_for_one_field(&rules),
        };
        expressible.then(|| SearchQuery::new("", rules.iter().map(|rule| rule.term()).collect()))
    }
}

/// Whether no field other than tags has two positive rules, which the search
/// would read as alternatives.
fn no_field_repeats_positively(rules: &[&Rule]) -> bool {
    Field::ALL.into_iter()
              .filter(|field| *field != Field::Tag)
              .all(|field| {
                  let positive =
                      rules.iter()
                           .filter(|rule| rule.field == field && rule.operator == Operator::Is);
                  positive.count() < 2
              })
}

/// Whether every rule is positive and names the same field, other than tags,
/// so the search reads them as alternatives.
fn all_positive_for_one_field(rules: &[&Rule]) -> bool {
    let field = rules[0].field;
    field != Field::Tag
    && rules.iter()
            .all(|rule| rule.field == field && rule.operator == Operator::Is)
}

#[cfg(test)]
mod tests;
