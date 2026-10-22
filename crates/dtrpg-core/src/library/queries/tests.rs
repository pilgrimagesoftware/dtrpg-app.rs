//! Core-level tests for the filtered/grouped query cache: ADR 0002 Slice 3.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::time::SystemTime;

use super::ViewState;
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::data::storage::StorageConfig;
use crate::library::Library;

fn item(id: &str, publisher: &str) -> LibraryItem {
    LibraryItem::new(id,
                     "Title",
                     publisher,
                     "",
                     "Core",
                     "PDF",
                     0,
                     0.0,
                     2020,
                     0,
                     ItemStatus::Cloud,
                     "#000000",
                     "",
                     None).with_filter("Product Type", "Core")
}

fn test_storage_config() -> StorageConfig {
    StorageConfig { override_path:                None,
                    max_concurrent_downloads:
                        crate::data::storage::DEFAULT_MAX_CONCURRENT_DOWNLOADS,
                    create_collections:           false,
                    check_free_space:             true,
                    recently_updated_window_days:
                        crate::data::constants::RECENTLY_UPDATED_WINDOW_DEFAULT_DAYS,
                    extra_roots:                  Vec::new(),
                    rules:                        Vec::new(),
                    next_root_id:                 1,
                    default_root_id:              0, }
}

fn open_library() -> Arc<Library> {
    let dir = tempfile::tempdir().expect("tempdir");
    let (library, _work_events, _sync_events) =
        Library::open(dir.keep(), &test_storage_config()).expect("open library");
    Arc::new(library)
}

#[test]
fn invalidate_cache_populates_items_filter_count_and_catalog_ids() {
    let library = open_library();
    library.index
           .lock()
           .unwrap()
           .replace_all_items(&[item("a", "Paizo"), item("b", "Paizo")])
           .unwrap();

    library.invalidate_cache(&ViewState::default(), 0, "Untagged");

    assert_eq!(library.visible_items().len(), 2);
    assert_eq!(library.visible_items_count(), 2);
    assert_eq!(library.filter_count(), 2);
    assert_eq!(library.section_counts().all, 2);
    assert_eq!(library.publishers().len(), 1);
}

#[test]
fn visible_items_grouped_is_empty_without_a_group_by() {
    let library = open_library();
    library.index
           .lock()
           .unwrap()
           .replace_all_items(&[item("a", "Paizo")])
           .unwrap();

    library.invalidate_cache(&ViewState::default(), 0, "Untagged");

    assert!(library.visible_items_grouped().is_empty());
}

#[test]
fn note_thumbnail_attempt_patches_the_cache_without_recomputing_it() {
    let library = open_library();
    library.index
           .lock()
           .unwrap()
           .replace_all_items(&[item("a", "Paizo")])
           .unwrap();
    library.invalidate_cache(&ViewState::default(), 0, "Untagged");

    let before = Arc::as_ptr(&library.visible_items());
    let at = SystemTime::now();
    library.note_thumbnail_attempt(&Arc::from("a"), at);
    let after = Arc::as_ptr(&library.visible_items());

    assert_eq!(before, after,
               "the result set was rebuilt, not just patched");
    let patched = library.visible_items();
    assert_eq!(patched[0].thumbnail_last_attempted, Some(at));
}

#[test]
fn item_by_id_overlays_ephemeral_state() {
    let library = open_library();
    library.index
           .lock()
           .unwrap()
           .replace_all_items(&[item("a", "Paizo")])
           .unwrap();
    let at = SystemTime::now();
    library.note_availability_check(&Arc::from("a"), at);

    let found = library.item_by_id("a").expect("item present");
    assert_eq!(found.availability_last_checked, Some(at));
}

#[test]
fn tags_round_trip_through_set_and_get() {
    let library = open_library();
    assert_eq!(library.tags().entries().len(), 0);

    let tags = crate::data::tag_library::TagLibrary::new([("a".to_owned(),
                                                           vec!["solo".to_owned()])].into(),
                                                         Default::default());
    library.set_tags(tags);

    assert_eq!(library.tags().entries().len(), 1);
}

#[test]
fn the_facets_count_the_items_on_show_without_the_choices_made_in_that_facet() {
    let library = open_library();
    library.index
           .lock()
           .unwrap()
           .replace_all_items(&[item("a", "Paizo"), item("b", "Paizo")])
           .unwrap();
    let view = ViewState { search_query: "type:core".to_owned(),
                           ..ViewState::default() };

    library.invalidate_cache(&view, 0, "Untagged");
    let facets = library.visible_facets();

    // Both items are "Core"; the type facet is counted without the type choice,
    // and availability with it.
    assert_eq!(facets.types.len(), 1);
    assert_eq!(facets.types[0].count, 2);
    assert_eq!(facets.availability.len(), 1);
    assert_eq!(facets.availability[0].count, 2);
    assert!(facets.systems.is_empty());
}

#[test]
fn a_smart_collection_counts_and_lists_the_items_that_match_its_rules() {
    use crate::data::smart_collection::{MatchMode, Operator, Rule, SmartCollection};
    use crate::util::search::Field;

    let library = open_library();
    let mut paizo = item("a", "Paizo");
    paizo.order_product_id = 11;
    paizo.product_id = 21;
    let mut other = item("b", "Other");
    other.order_product_id = 12;
    other.product_id = 22;
    library.index
           .lock()
           .unwrap()
           .replace_all_items(&[paizo, other])
           .unwrap();
    let collection = SmartCollection { mode: MatchMode::All,
                                       rules: vec![Rule { field:    Field::Publisher,
                                                          operator: Operator::Is,
                                                          value:    "paizo".to_owned(), }],
                                       ..SmartCollection::default() };

    assert_eq!(library.smart_collection_match_count(&collection), (1, 2));
    let ids = library.smart_collection_member_ids(&collection);
    assert_eq!(ids, [11, 21].into_iter().collect());
}

#[test]
fn the_filter_values_of_a_field_are_listed_once_across_the_whole_catalog() {
    use crate::util::search::Field;

    let library = open_library();
    let items = [item("a", "Paizo").with_filter("Format", "PDF")
                                   .with_filter("Format", "Audio"),
                 item("b", "Other").with_filter("Format", "pdf")
                                   .with_filter("Genre", "Horror")];
    library.index
           .lock()
           .unwrap()
           .replace_all_items(&items)
           .unwrap();

    let mut formats = library.filter_values(Field::Format);
    formats.sort_by_key(|value| value.to_lowercase());

    assert_eq!(formats.len(), 2);
    assert_eq!(formats[0], "Audio");
    assert_eq!(formats[1].to_lowercase(), "pdf");
    assert_eq!(library.filter_values(Field::Genre), ["Horror"]);
    assert!(library.filter_values(Field::Publisher).is_empty());
}

#[test]
fn a_collection_counts_the_members_that_are_only_in_the_cloud() {
    use crate::data::collection::CollectionEntry;

    let library = open_library();
    let mut downloaded = item("a", "Paizo");
    downloaded.status = ItemStatus::Downloaded;
    downloaded.product_id = 21;
    let mut cloud = item("b", "Paizo");
    cloud.status = ItemStatus::Cloud;
    cloud.product_id = 22;
    let mut outsider = item("c", "Paizo");
    outsider.status = ItemStatus::Cloud;
    outsider.product_id = 23;
    library.index
           .lock()
           .unwrap()
           .replace_all_items(&[downloaded, cloud, outsider])
           .unwrap();
    library.set_collections(vec![CollectionEntry { id:         7,
                                                   name:       "Shelf".into(),
                                                   member_ids: Arc::from([21, 22]),
                                                   created_at: 0, }]);

    let counts = library.collection_cloud_counts();

    assert_eq!(counts.get(&7), Some(&1));
}
