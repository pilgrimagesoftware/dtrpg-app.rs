//! Tests for the library search box.

use super::*;
use crate::data::enums::ItemStatus;

fn item(title: &str, publisher: &str, line: &str) -> LibraryItem {
    LibraryItem::new("a",
                     title,
                     publisher,
                     line,
                     "Core",
                     "PDF",
                     100,
                     10.0,
                     2024,
                     0,
                     ItemStatus::Cloud,
                     "#000000",
                     "",
                     None)
}

fn tags(names: &[&str]) -> Vec<Arc<str>> {
    names.iter().map(|name| Arc::from(*name)).collect()
}

fn matches(query: &str, item: &LibraryItem, names: &[&str]) -> bool {
    SearchQuery::parse(query).matches(item, &tags(names))
}

#[test]
fn an_empty_search_matches_everything() {
    let moria = item("Moria", "Free League", "The One Ring");

    assert!(matches("", &moria, &[]));
}

#[test]
fn plain_text_matches_title_publisher_and_line_ignoring_case() {
    let moria = item("Moria", "Free League", "The One Ring");

    assert!(matches("moria", &moria, &[]));
    assert!(matches("FREE", &moria, &[]));
    assert!(matches("one ring", &moria, &[]));
    assert!(!matches("starfinder", &moria, &[]));
}

#[test]
fn plain_text_is_kept_exactly_as_typed() {
    let moria = item("Moria", "Free  League", "");

    // Two spaces in the publisher, matched by two spaces in the search.
    assert!(matches("free  league", &moria, &[]));
}

#[test]
fn plain_text_also_matches_a_tag_name() {
    let moria = item("Moria", "Free League", "");

    assert!(matches("solo", &moria, &["Solo play"]));
    assert!(!matches("solo", &moria, &["co-op"]));
}

#[test]
fn a_tag_term_keeps_only_the_items_with_the_tag() {
    let moria = item("Moria", "Free League", "");

    assert!(matches("tag:solo", &moria, &["Solo"]));
    assert!(!matches("tag:solo", &moria, &["co-op"]));
    assert!(!matches("tag:solo", &moria, &[]));
}

#[test]
fn a_tag_term_matches_the_whole_tag_name_not_part_of_it() {
    let moria = item("Moria", "Free League", "");

    assert!(!matches("tag:sol", &moria, &["solo"]));
}

#[test]
fn a_negated_tag_term_drops_the_items_with_the_tag() {
    let moria = item("Moria", "Free League", "");

    assert!(!matches("-tag:solo", &moria, &["solo"]));
    assert!(matches("-tag:solo", &moria, &["co-op"]));
    assert!(matches("-tag:solo", &moria, &[]));
}

#[test]
fn every_tag_term_must_hold() {
    let moria = item("Moria", "Free League", "");

    assert!(matches("tag:solo tag:printed", &moria, &["printed", "solo"]));
    assert!(!matches("tag:solo tag:printed", &moria, &["solo"]));
    assert!(matches("tag:solo -tag:printed", &moria, &["solo"]));
    assert!(!matches("tag:solo -tag:printed", &moria, &["solo", "printed"]));
}

#[test]
fn a_tag_name_with_spaces_is_quoted() {
    let moria = item("Moria", "Free League", "");

    assert!(matches("tag:\"to run\"", &moria, &["To Run"]));
    assert!(matches("-tag:\"to run\"", &moria, &["solo"]));
    assert!(!matches("tag:\"to run\"", &moria, &["to"]));
}

#[test]
fn a_quote_that_is_never_closed_runs_to_the_end() {
    let moria = item("Moria", "Free League", "");

    assert!(matches("tag:\"to run", &moria, &["to run"]));
}

#[test]
fn a_tag_term_is_read_ignoring_case_and_combines_with_text() {
    let moria = item("Moria", "Free League", "");
    let ring = item("The One Ring", "Free League", "");

    assert!(matches("TAG:Solo moria", &moria, &["solo"]));
    assert!(!matches("tag:solo moria", &ring, &["solo"]));
    assert!(matches("free tag:solo", &ring, &["solo"]));
}

#[test]
fn a_tag_term_with_no_name_asks_for_nothing() {
    let moria = item("Moria", "Free League", "");

    assert!(!SearchQuery::parse("tag:").has_tag_terms());
    assert!(matches("tag: moria", &moria, &[]));
    assert!(matches("tag:\"\"", &moria, &[]));
}

#[test]
fn only_a_search_with_a_tag_term_needs_the_items_tags() {
    assert!(SearchQuery::parse("tag:solo").has_tag_terms());
    assert!(SearchQuery::parse("moria -tag:solo").has_tag_terms());
    assert!(!SearchQuery::parse("moria").has_tag_terms());
    assert!(!SearchQuery::parse("tagged").has_tag_terms());
}

fn typed_item(publisher: &str, system: &str, kind: &str, status: ItemStatus) -> LibraryItem {
    let item = LibraryItem::new("a", "Moria", publisher, system, kind, "PDF", 100, 10.0, 2024, 0,
                                status, "#000000", "", None);
    with_system_and_type(item, system, kind)
}

/// The item with a rule system and a product type filter for each of `system`
/// and `kind` that is not empty.
fn with_system_and_type(item: LibraryItem, system: &str, kind: &str) -> LibraryItem {
    let item = if system.is_empty() {
        item
    }
    else {
        item.with_filter("Rule System", system)
    };
    if kind.is_empty() {
        item
    }
    else {
        item.with_filter("Product Type", kind)
    }
}

#[test]
fn a_publisher_term_matches_the_whole_publisher_ignoring_case() {
    let moria = typed_item("Free League", "", "Core", ItemStatus::Cloud);

    assert!(matches("publisher:\"free league\"", &moria, &[]));
    assert!(matches("PUBLISHER:\"FREE LEAGUE\"", &moria, &[]));
    assert!(!matches("publisher:free", &moria, &[]));
    assert!(matches("-publisher:paizo", &moria, &[]));
    assert!(!matches("-publisher:\"free league\"", &moria, &[]));
}

#[test]
fn a_system_term_tests_the_game_system() {
    let moria = typed_item("Free League", "The One Ring", "Core", ItemStatus::Cloud);

    assert!(matches("system:\"the one ring\"", &moria, &[]));
    assert!(!matches("system:\"pathfinder 2e\"", &moria, &[]));
    assert!(matches("-system:\"pathfinder 2e\"", &moria, &[]));
}

#[test]
fn a_type_term_tests_the_content_type() {
    let moria = typed_item("Free League", "", "Adventure", ItemStatus::Cloud);

    assert!(matches("type:adventure", &moria, &[]));
    assert!(!matches("type:core", &moria, &[]));
    assert!(matches("-type:core", &moria, &[]));
}

#[test]
fn an_availability_term_tests_the_download_state() {
    let cloud = typed_item("", "", "Core", ItemStatus::Cloud);
    let downloaded = typed_item("", "", "Core", ItemStatus::Downloaded);

    assert!(matches("availability:cloud", &cloud, &[]));
    assert!(!matches("availability:cloud", &downloaded, &[]));
    assert!(matches("-availability:cloud", &downloaded, &[]));
}

#[test]
fn an_availability_that_names_no_state_holds_for_no_item() {
    let cloud = typed_item("", "", "Core", ItemStatus::Cloud);

    assert!(!matches("availability:lost", &cloud, &[]));
    assert!(matches("-availability:lost", &cloud, &[]));
}

#[test]
fn every_term_must_hold_and_the_text_must_match() {
    let moria = typed_item("Free League", "The One Ring", "Core", ItemStatus::Cloud);

    assert!(matches("moria type:core -tag:solo system:\"the one ring\"",
                    &moria,
                    &["horror"]));
    assert!(!matches("moria type:adventure system:\"the one ring\"", &moria, &[]));
    assert!(!matches("starfinder type:core", &moria, &[]));
}

#[test]
fn a_word_before_a_colon_that_is_not_a_field_is_free_text() {
    let query = SearchQuery::parse("colour:cloud");

    assert_eq!(query.free_text(), "colour:cloud");
    assert!(query.terms().is_empty());
}

#[test]
fn a_query_prints_its_terms_in_the_order_typed_then_its_text() {
    let query = SearchQuery::parse("moria -tag:solo Publisher:\"Free League\" type:core");

    assert_eq!(query.to_string(),
               "-tag:solo publisher:\"free league\" type:core moria");
}

#[test]
fn a_term_with_no_value_is_not_printed() {
    assert_eq!(SearchQuery::parse("tag: system:\"\" moria").to_string(),
               "moria");
}

#[test]
fn terms_for_the_same_field_are_alternatives() {
    let core = typed_item("", "", "Core", ItemStatus::Cloud);
    let adventure = typed_item("", "", "Adventure", ItemStatus::Cloud);
    let setting = typed_item("", "", "Setting", ItemStatus::Cloud);

    assert!(matches("type:core type:adventure", &core, &[]));
    assert!(matches("type:core type:adventure", &adventure, &[]));
    assert!(!matches("type:core type:adventure", &setting, &[]));
}

#[test]
fn alternatives_for_one_field_still_have_to_agree_with_the_other_terms() {
    let moria = typed_item("Free League", "", "Core", ItemStatus::Cloud);

    assert!(matches("type:core type:adventure publisher:\"free league\"",
                    &moria,
                    &[]));
    assert!(!matches("type:core type:adventure publisher:paizo", &moria, &[]));
    assert!(!matches("type:core type:adventure -type:core", &moria, &[]));
}

#[test]
fn tag_terms_all_have_to_hold_unlike_other_fields() {
    let moria = typed_item("", "", "Core", ItemStatus::Cloud);

    assert!(!matches("tag:solo tag:printed", &moria, &["solo"]));
}

#[test]
fn a_value_is_toggled_on_and_off() {
    let query = SearchQuery::parse("moria");
    let on = query.toggled(Field::System, "D&D 5E");

    assert!(on.asks_for(Field::System, "d&d 5e"));
    assert_eq!(on.to_string(), "system:\"d&d 5e\" moria");
    assert_eq!(on.toggled(Field::System, "D&D 5E"), query);
}

#[test]
fn toggling_a_value_leaves_a_negated_term_for_it_alone() {
    let query = SearchQuery::parse("-type:core");

    let toggled = query.toggled(Field::Type, "core");

    assert!(toggled.asks_for(Field::Type, "core"));
    assert_eq!(toggled.terms().len(), 2);
}

#[test]
fn toggling_nothing_changes_nothing() {
    let query = SearchQuery::parse("moria");

    assert_eq!(query.toggled(Field::Tag, "  "), query);
}

#[test]
fn a_term_or_a_whole_field_can_be_taken_out() {
    let query = SearchQuery::parse("type:core type:adventure tag:solo");
    let core = query.terms()[0].clone();

    assert_eq!(query.without_term(&core).to_string(),
               "type:adventure tag:solo");
    assert_eq!(query.without_field(Field::Type).to_string(), "tag:solo");
}

#[test]
fn a_product_with_several_systems_matches_each_of_them() {
    let item = typed_item("Paizo", "", "Adventure", ItemStatus::Cloud)
        .with_filter("Rule System", "Pathfinder / Starfinder")
        .with_filter("Rule System", "Pathfinder OGL 1E");

    assert!(matches("system:\"pathfinder / starfinder\"", &item, &[]));
    assert!(matches("system:\"pathfinder ogl 1e\"", &item, &[]));
    assert!(!matches("system:\"5e-compatible\"", &item, &[]));
    assert!(!matches("-system:\"pathfinder ogl 1e\"", &item, &[]));
}

#[test]
fn format_and_genre_terms_read_their_own_filters() {
    let item = typed_item("Paizo", "", "", ItemStatus::Cloud).with_filter("Format", "PDF")
                                                             .with_filter("Genre", "Horror");

    assert!(matches("format:pdf genre:horror", &item, &[]));
    assert!(!matches("format:horror", &item, &[]),
            "a value is only read in its own category");
    assert!(!matches("genre:pdf", &item, &[]));
    assert!(matches("-genre:fantasy", &item, &[]));
}

#[test]
fn a_filter_in_another_category_does_not_answer_a_term() {
    let item = typed_item("Paizo", "", "", ItemStatus::Cloud).with_filter("Genre", "Fantasy");

    assert!(!matches("system:fantasy", &item, &[]));
    assert!(!matches("type:fantasy", &item, &[]));
}

#[test]
fn a_location_term_is_held_by_an_item_with_files_there() {
    let mut moria = item("Moria", "Free League", "The One Ring");
    moria.locations = vec!["Vault".into(), "Local".into()];

    assert!(matches("location:vault", &moria, &[]));
    assert!(matches("location:\"local\"", &moria, &[]));
    assert!(!matches("location:cloud", &moria, &[]));
    assert!(!matches("-location:vault", &moria, &[]));
    assert!(matches("-location:cloud", &moria, &[]));
}

#[test]
fn an_item_with_no_files_is_in_no_location() {
    let moria = item("Moria", "Free League", "The One Ring");

    assert!(!matches("location:vault", &moria, &[]));
    assert!(matches("-location:vault", &moria, &[]));
}

#[test]
fn a_collection_term_is_held_by_an_item_in_that_collection() {
    let mut moria = item("Moria", "Free League", "The One Ring");
    moria.collections = vec!["Solo Play".into()];

    assert!(matches("collection:\"solo play\"", &moria, &[]));
    assert!(!matches("collection:horror", &moria, &[]));
    assert!(!matches("-collection:\"SOLO PLAY\"", &moria, &[]));
    assert!(matches("-collection:horror", &moria, &[]));
}

#[test]
fn an_item_in_no_collection_is_held_by_no_collection_term() {
    let moria = item("Moria", "Free League", "The One Ring");

    assert!(!matches("collection:\"solo play\"", &moria, &[]));
    assert!(matches("-collection:\"solo play\"", &moria, &[]));
}
