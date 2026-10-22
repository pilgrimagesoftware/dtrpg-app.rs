//! The library search box: free text, and `field:value` terms.
//!
//! A term keeps only the items whose field has exactly that value, ignoring
//! case; a `-` in front keeps only those that do not. Several terms for the
//! same field (other than `tag`) are alternatives: `type:core type:adventure`
//! keeps items of either type. Terms for different fields, tag terms, and
//! `-` terms all have to hold together. The fields are `tag`,
//! `publisher`, `system` (the game system), `type` (the content type, such as
//! a core rulebook or an adventure) and `availability` (`downloaded` or
//! `cloud`). A value with spaces is quoted: `tag:"to run"`. Anything else is
//! free text, matched against an item's title, publisher, game line and tag
//! names.
//!
//! A search is parsed into a typed [`SearchQuery`], and a query prints back to
//! text with [`std::fmt::Display`], so the search box, the filter chips and a
//! smart collection's rule line can all show the same query.

use std::fmt;
use std::sync::Arc;

use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::product_filter::{
    CATEGORY_FORMAT, CATEGORY_GENRE, CATEGORY_PRODUCT_TYPE, CATEGORY_RULE_SYSTEM,
};

mod parse;

/// The item property a [`Term`] tests.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Field {
    /// A tag the item has.
    Tag,
    /// The item's publisher.
    Publisher,
    /// A game system the item is for, from its `Rule System` filters. An
    /// item can have several, and each level of a nested system counts.
    System,
    /// A type the item has, from its `Product Type` filters.
    Type,
    /// A format the item has, from its `Format` filters.
    Format,
    /// A genre the item has, from its `Genre` filters.
    Genre,
    /// Whether the item is `downloaded` or still in the `cloud`.
    Availability,
    /// The storage location a downloaded file of the item is in.
    Location,
}

impl Field {
    /// Every field, in the order a search is documented.
    pub const ALL: [Self; 8] = [Self::Tag,
                                Self::Publisher,
                                Self::System,
                                Self::Type,
                                Self::Format,
                                Self::Genre,
                                Self::Availability,
                                Self::Location];

    /// The field whose keyword is `keyword`, ignoring case.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::util::search::Field;
    ///
    /// assert_eq!(Field::from_keyword("Type"), Some(Field::Type));
    /// assert_eq!(Field::from_keyword("title"), None);
    /// ```
    #[must_use]
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        Self::ALL.into_iter()
                 .find(|field| field.keyword().eq_ignore_ascii_case(keyword))
    }

    /// The word before the colon in a term.
    #[must_use]
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Tag => "tag",
            Self::Publisher => "publisher",
            Self::System => "system",
            Self::Type => "type",
            Self::Format => "format",
            Self::Genre => "genre",
            Self::Availability => "availability",
            Self::Location => "location",
        }
    }

    /// The category of product filters this field reads, or `None` for a field
    /// that reads something else.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::util::search::Field;
    ///
    /// assert_eq!(Field::System.category(), Some("Rule System"));
    /// assert_eq!(Field::Publisher.category(), None);
    /// ```
    #[must_use]
    pub const fn category(self) -> Option<&'static str> {
        match self {
            Self::System => Some(CATEGORY_RULE_SYSTEM),
            Self::Type => Some(CATEGORY_PRODUCT_TYPE),
            Self::Format => Some(CATEGORY_FORMAT),
            Self::Genre => Some(CATEGORY_GENRE),
            Self::Tag | Self::Publisher | Self::Availability | Self::Location => None,
        }
    }
}

/// One `field:value` term of a search, such as `-tag:solo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Term {
    /// What the term tests.
    pub field:   Field,
    /// The value the field must have, or must not have if `negated`. Lower
    /// cased, and never empty.
    pub value:   String,
    /// Whether the item must not have the value.
    pub negated: bool,
}

impl Term {
    /// The download state an `availability` term names, or `None` for any
    /// other field or a value that names no state.
    #[must_use]
    pub fn availability(&self) -> Option<ItemStatus> {
        if self.field != Field::Availability {
            return None;
        }
        match self.value.as_str() {
            "downloaded" => Some(ItemStatus::Downloaded),
            "cloud" => Some(ItemStatus::Cloud),
            _ => None,
        }
    }

    /// Whether `item`, which has the tags `tags`, has the term's value,
    /// before the term's negation is applied. An availability value that
    /// names no state is never held.
    pub(crate) fn is_held_by(&self, item: &LibraryItem, tags: &[Arc<str>]) -> bool {
        let equals = |text: &str| text.to_lowercase() == self.value;
        match self.field {
            Field::Tag => tags.iter().any(|tag| equals(tag)),
            Field::Publisher => equals(&item.publisher),
            Field::System | Field::Type | Field::Format | Field::Genre => {
                let category = self.field.category().unwrap_or_default();
                item.filters
                    .iter()
                    .any(|filter| filter.is_in(category) && equals(&filter.name))
            }
            Field::Availability => self.availability() == Some(item.status),
            Field::Location => item.locations.iter().any(|location| equals(location)),
        }
    }
}

impl fmt::Display for Term {
    /// Prints the term as it is typed, quoting a value that has spaces.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.negated { "-" } else { "" };
        let keyword = self.field.keyword();
        if self.value.contains(char::is_whitespace) {
            write!(f, "{sign}{keyword}:\"{}\"", self.value)
        }
        else {
            write!(f, "{sign}{keyword}:{}", self.value)
        }
    }
}

/// A search as the user typed it, split into what it asks for.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SearchQuery {
    /// The free text, or empty. Lower cased.
    text:  String,
    /// The terms, in the order they were typed.
    terms: Vec<Term>,
}

impl SearchQuery {
    /// Builds a query from its parts, for a caller that has them already, such
    /// as a smart collection's rules. `text` is lower cased, as a parsed one
    /// is.
    #[must_use]
    pub fn new(text: &str, terms: Vec<Term>) -> Self {
        Self { text: text.to_lowercase(),
               terms }
    }

    /// Whether the search has any tag term, and so needs each item's tags.
    #[must_use]
    pub fn has_tag_terms(&self) -> bool {
        self.terms.iter().any(|term| term.field == Field::Tag)
    }

    /// The free-text part, always lower cased — see [`Self::parse`] for when
    /// it preserves the typed spacing verbatim vs. rejoining words.
    pub(crate) fn free_text(&self) -> &str {
        &self.text
    }

    /// The terms, in the order they were typed.
    #[must_use]
    pub fn terms(&self) -> &[Term] {
        &self.terms
    }

    /// Whether the query asks for items whose `field` has `value`, ignoring
    /// case.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::util::search::{Field, SearchQuery};
    ///
    /// let query = SearchQuery::parse("type:adventure moria");
    /// assert!(query.asks_for(Field::Type, "Adventure"));
    /// assert!(!query.asks_for(Field::Type, "core"));
    /// ```
    #[must_use]
    pub fn asks_for(&self, field: Field, value: &str) -> bool {
        let value = value.to_lowercase();
        self.terms
            .iter()
            .any(|term| !term.negated && term.field == field && term.value == value)
    }

    /// The query with `field:value` added if it did not ask for it, or taken
    /// away if it did. A value with nothing in it leaves the query as it was.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::util::search::{Field, SearchQuery};
    ///
    /// let query = SearchQuery::parse("moria");
    /// let with = query.toggled(Field::Type, "Adventure");
    /// assert_eq!(with.to_string(), "type:adventure moria");
    /// assert_eq!(with.toggled(Field::Type, "adventure"), query);
    /// ```
    #[must_use]
    pub fn toggled(&self, field: Field, value: &str) -> Self {
        let value = value.trim().to_lowercase();
        let mut query = self.clone();
        if value.is_empty() {
            return query;
        }
        if self.asks_for(field, &value) {
            query.terms
                 .retain(|term| term.negated || term.field != field || term.value != value);
        }
        else {
            query.terms.push(Term { field,
                                    value,
                                    negated: false });
        }
        query
    }

    /// The query without `term`, wherever it sits among the others.
    #[must_use]
    pub fn without_term(&self, term: &Term) -> Self {
        let mut query = self.clone();
        query.terms.retain(|other| other != term);
        query
    }

    /// The query without any term for `field`, which is what a facet counts
    /// its own values against: the other choices still narrow the result, but
    /// its own do not.
    #[must_use]
    pub fn without_field(&self, field: Field) -> Self {
        let mut query = self.clone();
        query.terms.retain(|term| term.field != field);
        query
    }

    /// Whether `item`, which has the tags `tags`, matches the search.
    ///
    /// No production caller since the search box became a SQL query
    /// dimension (#353): kept as the specification the index's
    /// `instr(ulower(col), ?)` predicates are tested against
    /// (`data/index/filtered_items/parity_tests.rs`). Do not delete without
    /// retiring that equivalence bar.
    #[must_use]
    pub fn matches(&self, item: &LibraryItem, tags: &[Arc<str>]) -> bool {
        if !self.terms_hold(item, tags) {
            return false;
        }
        self.text.is_empty()
        || item.title.to_lowercase().contains(&self.text)
        || item.publisher.to_lowercase().contains(&self.text)
        || item.line.to_lowercase().contains(&self.text)
        || tags.iter()
               .any(|tag| tag.to_lowercase().contains(&self.text))
    }
}

impl SearchQuery {
    /// Whether the terms hold for `item`: each `-` term and each tag term on
    /// its own, and, for every other field with terms, at least one of them.
    fn terms_hold(&self, item: &LibraryItem, tags: &[Arc<str>]) -> bool {
        let held = |term: &Term| term.is_held_by(item, tags);
        let singly = self.terms
                         .iter()
                         .filter(|term| term.negated || term.field == Field::Tag)
                         .all(|term| held(term) != term.negated);
        let alternatives =
            Field::ALL.into_iter()
                      .filter(|field| *field != Field::Tag)
                      .all(|field| {
                          let mut terms = self.terms
                                              .iter()
                                              .filter(|term| !term.negated && term.field == field)
                                              .peekable();
                          terms.peek().is_none() || terms.any(held)
                      });
        singly && alternatives
    }
}

impl fmt::Display for SearchQuery {
    /// Prints the query as text that parses back to the same query: the terms
    /// first, then the free text.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtrpg_core::util::search::SearchQuery;
    ///
    /// let query = SearchQuery::parse("moria -tag:solo system:\"the one ring\"");
    /// assert_eq!(query.to_string(), "-tag:solo system:\"the one ring\" moria");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts: Vec<String> = self.terms.iter().map(ToString::to_string).collect();
        if !self.text.is_empty() {
            parts.push(self.text.clone());
        }
        f.write_str(&parts.join(" "))
    }
}

#[cfg(test)]
mod round_trip_tests;
#[cfg(test)]
mod tests;
