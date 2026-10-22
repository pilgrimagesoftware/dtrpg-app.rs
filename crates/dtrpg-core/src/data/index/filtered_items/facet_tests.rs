//! Counting items by facet value, with and without a filter.

use super::super::LibraryIndex;
use super::clause::ItemFilter;
use super::facets::FacetCount;
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::tag_color::TagColor;
use crate::util::search::{Field, SearchQuery};

fn item(id: &str, system: &str, kind: &str, status: ItemStatus) -> LibraryItem {
    let item = LibraryItem::new(id,
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
                                Some(1_700_000_000));
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

#[test]
fn format_and_genre_facets_count_each_value_once_per_item() {
    let mut index = LibraryIndex::open_in_memory().unwrap();
    let items = [item("a", "", "", ItemStatus::Cloud).with_filter("Format", "PDF")
                                                     .with_filter("Genre", "Fantasy")
                                                     .with_filter("Genre", "Horror"),
                 item("b", "", "", ItemStatus::Cloud).with_filter("Format", "pdf")
                                                     .with_filter("Genre", "Fantasy"),
                 item("c", "", "", ItemStatus::Cloud)];
    index.replace_all_items(&items).unwrap();
    let none = ItemFilter::default();

    assert_eq!(counts(&index, &none, Field::Format),
               [("pdf".to_owned(), 2)]);
    assert_eq!(counts(&index, &none, Field::Genre),
               [("fantasy".to_owned(), 2), ("horror".to_owned(), 1)]);
}

#[test]
fn the_filters_of_an_item_survive_a_round_trip_through_the_index() {
    let mut index = LibraryIndex::open_in_memory().unwrap();
    let stored = item("a", "D&D 5E", "Core", ItemStatus::Cloud).with_filter("Genre", "Horror");
    index.replace_all_items(std::slice::from_ref(&stored))
         .unwrap();

    let read = index.get_item("a").unwrap().unwrap();
    let listed = index.list_items().unwrap();

    assert_eq!(read.filters, stored.filters);
    assert_eq!(listed[0].filters, stored.filters);
    assert_eq!(read.filters.len(), 3);
}

#[test]
fn replacing_one_item_replaces_its_filters() {
    let mut index = LibraryIndex::open_in_memory().unwrap();
    let before = item("a", "D&D 5E", "", ItemStatus::Cloud);
    index.replace_all_items(std::slice::from_ref(&before))
         .unwrap();
    let after = item("a", "Pathfinder", "", ItemStatus::Cloud);

    index.replace_item(&after).unwrap();

    let read = index.get_item("a").unwrap().unwrap();
    assert_eq!(read.filters.len(), 1);
    assert_eq!(read.filters[0].name.as_ref(), "Pathfinder");
}

/// Records where the files of items `a`, `b` and `c` are: `a` in two places,
/// `b` and `c` in one each.
fn with_locations(mut index: LibraryIndex) -> LibraryIndex {
    let file = |name: &str, location: &str| (name.to_owned(), location.to_owned());
    index.set_file_locations("a", &[file("x.pdf", "Vault"), file("y.pdf", "Local")])
         .unwrap();
    index.set_file_locations("b", &[file("z.pdf", "Vault")])
         .unwrap();
    index.set_file_locations("c", &[file("w.pdf", "Local")])
         .unwrap();
    index
}

#[test]
fn locations_count_each_item_once_for_each_place_it_has_files() {
    let index = with_locations(seeded());

    assert_eq!(counts(&index, &ItemFilter::default(), Field::Location),
               [("local".to_owned(), 2), ("vault".to_owned(), 2)]);
}

#[test]
fn location_counts_follow_the_filter() {
    let index = with_locations(seeded());
    let query = SearchQuery::parse("availability:downloaded");
    let filter = ItemFilter { search: Some(&query),
                              ..ItemFilter::default() };

    assert_eq!(counts(&index, &filter, Field::Location),
               [("vault".to_owned(), 1)]);
}

#[test]
fn a_location_term_keeps_the_items_with_files_there_and_a_negated_one_the_rest() {
    let index = with_locations(seeded());
    let ids = |text: &str| -> Vec<String> {
        let query = SearchQuery::parse(text);
        let filter = ItemFilter { search: Some(&query),
                                  ..ItemFilter::default() };
        let mut ids: Vec<String> =
            index.list_items_filtered(&filter,
                                      crate::util::sort::SortMethod::Title,
                                      crate::util::sort::SortDirection::Ascending)
                 .unwrap()
                 .into_iter()
                 .map(|item| item.id.to_string())
                 .collect();
        ids.sort();
        ids
    };

    assert_eq!(ids("location:vault"), ["a", "b"]);
    assert_eq!(ids("location:VAULT location:local"), ["a", "b", "c"]);
    assert_eq!(ids("-location:vault"), ["c", "d"]);
}
