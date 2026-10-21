//! Counting items by facet value, with and without a filter.

use super::super::LibraryIndex;
use super::clause::ItemFilter;
use super::facets::FacetCount;
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::tag_color::TagColor;
use crate::util::search::{Field, SearchQuery};

fn item(id: &str, system: &str, kind: &str, status: ItemStatus) -> LibraryItem {
    LibraryItem::new(id,
                     id,
                     "Publisher",
                     system,
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

/// Four items: two systems (one spelled in two cases, one missing), two types,
/// both download states, and two tags.
fn seeded() -> LibraryIndex {
    let mut index = LibraryIndex::open_in_memory().unwrap();
    let items = [item("a", "D&D 5E", "Core", ItemStatus::Cloud),
                 item("b", "d&d 5e", "Adventure", ItemStatus::Downloaded),
                 item("c", "Pathfinder", "Adventure", ItemStatus::Cloud),
                 item("d", "", "Core", ItemStatus::Cloud)];
    index.replace_all_items(&items).unwrap();
    for (id, tags) in [("a", vec!["Solo", "horror"]), ("b", vec!["solo"])] {
        let tags: Vec<_> = tags.into_iter()
                               .map(|name| (name.to_owned(), Option::<TagColor>::None))
                               .collect();
        index.set_item_tags(id, &tags, 1).unwrap();
    }
    index
}

fn counts(index: &LibraryIndex, filter: &ItemFilter<'_>, field: Field) -> Vec<(String, usize)> {
    index.facet_counts(filter, field)
         .unwrap()
         .into_iter()
         .map(|FacetCount { value, count }| (value.to_lowercase(), count))
         .collect()
}

#[test]
fn availability_counts_each_download_state() {
    let index = seeded();

    assert_eq!(counts(&index, &ItemFilter::default(), Field::Availability),
               [("cloud".to_owned(), 3), ("downloaded".to_owned(), 1)]);
}

#[test]
fn a_text_facet_folds_case_and_leaves_out_items_with_no_value() {
    let index = seeded();

    assert_eq!(counts(&index, &ItemFilter::default(), Field::System),
               [("d&d 5e".to_owned(), 2), ("pathfinder".to_owned(), 1)]);
    assert_eq!(counts(&index, &ItemFilter::default(), Field::Type),
               [("adventure".to_owned(), 2), ("core".to_owned(), 2)]);
}

#[test]
fn tags_count_each_item_once_per_tag_whatever_the_case() {
    let index = seeded();

    assert_eq!(counts(&index, &ItemFilter::default(), Field::Tag),
               [("solo".to_owned(), 2), ("horror".to_owned(), 1)]);
}

#[test]
fn counts_follow_the_filter() {
    let index = seeded();
    let query = SearchQuery::parse("type:adventure");
    let filter = ItemFilter { search: Some(&query),
                              ..ItemFilter::default() };

    assert_eq!(counts(&index, &filter, Field::System),
               [("d&d 5e".to_owned(), 1), ("pathfinder".to_owned(), 1)]);
    assert_eq!(counts(&index, &filter, Field::Availability),
               [("cloud".to_owned(), 1), ("downloaded".to_owned(), 1)]);
}

#[test]
fn a_facet_counted_without_its_own_terms_still_shows_the_alternatives() {
    let index = seeded();
    let query = SearchQuery::parse("type:adventure system:pathfinder");
    let without_type = query.without_field(Field::Type);
    let filter = ItemFilter { search: Some(&without_type),
                              ..ItemFilter::default() };

    // Only Pathfinder items remain, and the one of them that is Adventure is
    // not the only type on offer.
    assert_eq!(counts(&index, &filter, Field::Type),
               [("adventure".to_owned(), 1)]);
    let only_system = SearchQuery::parse("system:\"d&d 5e\"");
    let filter = ItemFilter { search: Some(&only_system),
                              ..ItemFilter::default() };
    assert_eq!(counts(&index, &filter, Field::Type),
               [("adventure".to_owned(), 1), ("core".to_owned(), 1)]);
}

#[test]
fn a_field_with_no_facet_gives_no_values() {
    let index = seeded();

    assert!(counts(&index, &ItemFilter::default(), Field::Publisher).is_empty());
}
