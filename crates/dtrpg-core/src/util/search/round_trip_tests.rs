//! A query prints to text that parses back to the same query, so the search
//! box, the filter chips and a smart collection's rule line always agree.

use proptest::collection::vec;
use proptest::prelude::{Strategy, prop_assert_eq, proptest};

use super::{Field, SearchQuery, Term};

/// Any field.
fn field() -> impl Strategy<Value = Field> {
    proptest::sample::select(Field::ALL.to_vec())
}

/// A term value as a user could type it: lower case, with single spaces
/// inside and none at the ends, and never empty.
fn value() -> impl Strategy<Value = String> {
    "[a-z0-9é]{1,6}( [a-z0-9é]{1,6}){0,2}"
}

/// A term a user could type.
fn term() -> impl Strategy<Value = Term> {
    (field(), value(), proptest::bool::ANY).prop_map(|(field, value, negated)| Term { field,
                                                                                      value,
                                                                                      negated })
}

/// Free text as the parser keeps it when a term is present: lower case words
/// joined by single spaces. A word has no colon, so it cannot read as a term.
fn text() -> impl Strategy<Value = String> {
    vec("[a-z0-9é-]{1,8}", 0..4).prop_map(|words| words.join(" "))
}

proptest! {
    #[test]
    fn a_query_with_terms_parses_back_from_its_text(terms in vec(term(), 1..5), text in text()) {
        let query = SearchQuery::new(&text, terms);

        prop_assert_eq!(SearchQuery::parse(&query.to_string()), query);
    }

    #[test]
    fn a_query_with_only_text_parses_back_from_its_text(text in text()) {
        let query = SearchQuery::new(&text, Vec::new());

        prop_assert_eq!(SearchQuery::parse(&query.to_string()), query);
    }

    #[test]
    fn printing_is_stable_once_a_search_has_been_parsed(typed in "[a-zA-Z0-9é:\" -]{0,40}") {
        let once = SearchQuery::parse(&typed).to_string();
        let twice = SearchQuery::parse(&once).to_string();

        prop_assert_eq!(twice, once);
    }
}
