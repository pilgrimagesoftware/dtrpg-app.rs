//! Tests for the smart collection editor's draft.

use dtrpg_core::data::smart_collection::{MatchMode, Operator};
use dtrpg_core::util::search::{Field, SearchQuery};

use super::SmartCollectionDraft;

#[test]
fn a_new_draft_has_one_empty_rule_and_cannot_be_saved() {
    let draft = SmartCollectionDraft::new();

    assert_eq!(draft.rules().len(), 1);
    assert!(!draft.can_save());
}

#[test]
fn a_draft_needs_a_name_and_a_rule_with_a_value() {
    let mut draft = SmartCollectionDraft::new();

    draft.set_name("Adventures");
    assert!(!draft.can_save(), "the rule has no value yet");
    draft.set_value(0, "adventure");
    assert!(draft.can_save());
    draft.set_name("   ");
    assert!(!draft.can_save(), "a blank name is no name");
}

#[test]
fn rules_are_added_changed_and_removed() {
    let mut draft = SmartCollectionDraft::new();
    draft.add_rule();

    draft.set_field(1, Field::Type);
    draft.set_operator(1, Operator::IsNot);
    draft.set_value(1, "core");
    draft.remove_rule(0);

    assert_eq!(draft.rules().len(), 1);
    assert_eq!(draft.rules()[0].field, Field::Type);
    assert_eq!(draft.rules()[0].operator, Operator::IsNot);
    assert_eq!(draft.rules()[0].value, "core");
}

#[test]
fn an_index_with_no_rule_changes_nothing() {
    let mut draft = SmartCollectionDraft::new();

    draft.set_value(7, "x");
    draft.set_field(7, Field::Type);
    draft.set_operator(7, Operator::IsNot);
    draft.remove_rule(7);

    assert_eq!(draft, SmartCollectionDraft::new());
}

#[test]
fn the_equivalent_query_line_follows_the_rules_and_the_mode() {
    let mut draft = SmartCollectionDraft::new();
    draft.set_value(0, "solo");
    draft.add_rule();
    draft.set_field(1, Field::Type);
    draft.set_value(1, "Adventure");

    assert_eq!(draft.equivalent_query_text().as_deref(),
               Some("tag:solo type:adventure"));
    draft.set_mode(MatchMode::Any);
    assert_eq!(draft.equivalent_query_text(),
               None,
               "the search cannot say a mixed 'any'");
}

#[test]
fn a_search_becomes_a_draft_with_a_rule_for_each_term() {
    let draft =
        SmartCollectionDraft::from_search(&SearchQuery::parse("moria type:adventure -tag:running"));

    assert_eq!(draft.rules().len(), 2);
    assert_eq!(draft.rules()[0].field, Field::Type);
    assert_eq!(draft.rules()[1].operator, Operator::IsNot);
    assert_eq!(draft.rules()[1].value, "running");
}

#[test]
fn a_search_with_no_terms_becomes_a_new_draft() {
    let draft = SmartCollectionDraft::from_search(&SearchQuery::parse("moria"));

    assert_eq!(draft, SmartCollectionDraft::new());
}

#[test]
fn the_saved_collection_has_a_trimmed_name() {
    let mut draft = SmartCollectionDraft::new();
    draft.set_name("  Adventures  ");

    assert_eq!(draft.into_collection().name, "Adventures");
}

#[test]
fn a_draft_made_from_a_manual_collection_is_that_collection_by_name_and_can_be_saved() {
    let draft = SmartCollectionDraft::from_manual_collection("Solo play");

    assert_eq!(draft.name(), "Solo play");
    assert_eq!(draft.rules().len(), 1);
    assert_eq!(draft.rules()[0].field, Field::Collection);
    assert_eq!(draft.rules()[0].operator, Operator::Is);
    assert_eq!(draft.rules()[0].value, "Solo play");
    assert!(draft.can_save());
    assert_eq!(draft.equivalent_query_text().as_deref(),
               Some("collection:\"solo play\""));
}
