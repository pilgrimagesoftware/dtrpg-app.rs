//! Tests for this module. A failed unwrap is a failed test, so they may use it.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use super::super::model::{MatchMode, Operator, Rule, SmartCollection};
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::util::search::{Field, SearchQuery};

fn item(publisher: &str, kind: &str, status: ItemStatus) -> LibraryItem {
    LibraryItem::new("id",
                     "Title",
                     publisher,
                     "5E",
                     kind,
                     "PDF",
                     10,
                     1.0,
                     2024,
                     0,
                     status,
                     "#1C2A44",
                     "",
                     Some(1_700_000_000))
}

fn rule(field: Field, operator: Operator, value: &str) -> Rule {
    Rule { field,
           operator,
           value: value.to_owned() }
}

fn collection(mode: MatchMode, rules: Vec<Rule>) -> SmartCollection {
    SmartCollection { mode,
                      rules,
                      ..SmartCollection::default() }
}

fn tags(names: &[&str]) -> Vec<Arc<str>> {
    names.iter().map(|name| Arc::from(*name)).collect()
}

#[test]
fn all_of_needs_every_rule() {
    let adventures_without_running = collection(MatchMode::All,
                                                vec![rule(Field::Type, Operator::Is, "adventure"),
                                                     rule(Field::Tag, Operator::IsNot, "running")]);
    let adventure = item("Paizo", "Adventure", ItemStatus::Cloud);

    assert!(adventures_without_running.contains(&adventure, &tags(&["solo"])));
    assert!(!adventures_without_running.contains(&adventure, &tags(&["Running"])));
    assert!(!adventures_without_running.contains(&item("Paizo", "Core", ItemStatus::Cloud), &[]));
}

#[test]
fn any_of_needs_one_rule() {
    let either = collection(MatchMode::Any,
                            vec![rule(Field::Publisher, Operator::Is, "Paizo"),
                                 rule(Field::Availability, Operator::Is, "downloaded")]);

    assert!(either.contains(&item("Paizo", "Core", ItemStatus::Cloud), &[]));
    assert!(either.contains(&item("Other", "Core", ItemStatus::Downloaded), &[]));
    assert!(!either.contains(&item("Other", "Core", ItemStatus::Cloud), &[]));
}

#[test]
fn a_collection_with_no_rules_has_no_members() {
    let item = item("Paizo", "Core", ItemStatus::Cloud);

    assert!(!collection(MatchMode::All, vec![]).contains(&item, &[]));
    assert!(!collection(MatchMode::Any, vec![]).contains(&item, &[]));
}

#[test]
fn a_rule_with_no_value_is_ignored() {
    let half_typed = collection(MatchMode::All,
                                vec![rule(Field::Type, Operator::Is, "core"),
                                     rule(Field::Tag, Operator::Is, "  ")]);
    let only_blank = collection(MatchMode::All, vec![rule(Field::Tag, Operator::Is, "")]);
    let core = item("Paizo", "Core", ItemStatus::Cloud);

    assert!(half_typed.contains(&core, &[]));
    assert!(!only_blank.contains(&core, &[]));
}

#[test]
fn values_are_compared_ignoring_case() {
    let shouting = collection(MatchMode::All,
                              vec![rule(Field::Publisher, Operator::Is, "PAIZO")]);

    assert!(shouting.contains(&item("paizo", "Core", ItemStatus::Cloud), &[]));
}

#[test]
fn an_availability_that_names_no_state_holds_for_nothing() {
    let is = collection(MatchMode::All,
                        vec![rule(Field::Availability, Operator::Is, "lost")]);
    let is_not = collection(MatchMode::All,
                            vec![rule(Field::Availability, Operator::IsNot, "lost")]);
    let any = item("Paizo", "Core", ItemStatus::Cloud);

    assert!(!is.contains(&any, &[]));
    assert!(is_not.contains(&any, &[]));
}

#[test]
fn the_equivalent_query_is_given_only_when_the_search_can_say_it() {
    let query =
        |collection: &SmartCollection| collection.equivalent_query().map(|query| query.to_string());
    let tag = |operator, value| rule(Field::Tag, operator, value);

    // Different fields, and two tags, are all combined with "and" in the
    // search.
    let all = collection(MatchMode::All,
                         vec![rule(Field::Type, Operator::Is, "Adventure"),
                              tag(Operator::Is, "solo"),
                              tag(Operator::IsNot, "running")]);
    assert_eq!(query(&all).as_deref(),
               Some("type:adventure tag:solo -tag:running"));

    // Two positive rules for one field would be read as alternatives.
    let clash = collection(MatchMode::All,
                           vec![rule(Field::Type, Operator::Is, "core"),
                                rule(Field::Type, Operator::Is, "adventure")]);
    assert_eq!(query(&clash), None);

    // "Any" is a search only for alternatives of one field, or a single rule.
    let alternatives = collection(MatchMode::Any,
                                  vec![rule(Field::Type, Operator::Is, "core"),
                                       rule(Field::Type, Operator::Is, "adventure")]);
    assert_eq!(query(&alternatives).as_deref(),
               Some("type:core type:adventure"));
    let mixed = collection(MatchMode::Any,
                           vec![rule(Field::Type, Operator::Is, "core"),
                                tag(Operator::Is, "solo")]);
    assert_eq!(query(&mixed), None);
    let two_tags = collection(MatchMode::Any,
                              vec![tag(Operator::Is, "solo"), tag(Operator::Is, "horror")]);
    assert_eq!(query(&two_tags), None);
    let single = collection(MatchMode::Any, vec![tag(Operator::IsNot, "solo")]);
    assert_eq!(query(&single).as_deref(), Some("-tag:solo"));

    assert_eq!(query(&collection(MatchMode::All, vec![])), None);
}

#[test]
fn the_equivalent_query_finds_what_the_collection_holds() {
    let items = [item("Paizo", "Core", ItemStatus::Cloud),
                 item("Paizo", "Adventure", ItemStatus::Downloaded),
                 item("Other", "Adventure", ItemStatus::Cloud),
                 item("Other", "Core", ItemStatus::Downloaded)];
    let tag_sets = [tags(&[]),
                    tags(&["solo"]),
                    tags(&["running"]),
                    tags(&["solo", "running"])];
    let cases = [collection(MatchMode::All,
                            vec![rule(Field::Publisher, Operator::Is, "paizo"),
                                 rule(Field::Tag, Operator::IsNot, "running")]),
                 collection(MatchMode::Any,
                            vec![rule(Field::Type, Operator::Is, "core"),
                                 rule(Field::Type, Operator::Is, "adventure")]),
                 collection(MatchMode::Any,
                            vec![rule(Field::Availability, Operator::IsNot, "cloud")])];

    for case in &cases {
        let query: SearchQuery = case.equivalent_query().expect("expressible");
        for item in &items {
            for tags in &tag_sets {
                assert_eq!(case.contains(item, tags),
                           query.matches(item, tags),
                           "{case:?} vs {query} for {item:?} with {tags:?}");
            }
        }
    }
}
