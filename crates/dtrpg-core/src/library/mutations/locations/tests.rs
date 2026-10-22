//! Tests for recording which storage location each downloaded file is in.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;
use std::sync::Arc;

use crate::data::enums::ItemStatus;
use crate::data::library::{LibraryItem, LibraryItemFile};
use crate::data::storage::StorageConfig;
use crate::data::storage::placement::{PlacementCondition, PlacementRule};
use crate::data::storage::roots::StorageRoot;
use crate::library::Library;

/// A downloaded item from `publisher` with one downloaded file and one not.
fn downloaded_item(id: &str, publisher: &str) -> LibraryItem {
    let mut item = LibraryItem::new(id,
                                    "Title",
                                    publisher,
                                    "",
                                    "Core",
                                    "PDF",
                                    0,
                                    0.0,
                                    2020,
                                    0,
                                    ItemStatus::Downloaded,
                                    "#000000",
                                    "",
                                    None);
    let file = |name: &str, downloaded: bool| LibraryItemFile { id: name.into(),
                                                                index: 0,
                                                                name: name.into(),
                                                                format: "PDF".into(),
                                                                size_mb: 1.0,
                                                                downloaded,
                                                                offloaded: false };
    item.files = vec![file("have.pdf", true), file("not-yet.pdf", false)];
    item
}

/// A configuration with a second location, "Vault", that Paizo's titles go to.
fn storage() -> StorageConfig {
    let mut storage = StorageConfig::for_test(PathBuf::from("/tmp/dtrpg-default"))
        .with_extra_roots(vec![StorageRoot { id:   1,
                                             name: "Vault".to_owned(),
                                             path: PathBuf::from("/tmp/dtrpg-vault"), }]);
    storage.rules = vec![PlacementRule { condition:
                                             PlacementCondition::Publisher("Paizo".to_owned()),
                                         target_root_id: 1, }];
    storage
}

fn open_library(items: &[LibraryItem]) -> Arc<Library> {
    let dir = tempfile::tempdir().expect("tempdir");
    let (library, _work_events, _sync_events) =
        Library::open(dir.keep(), &storage()).expect("open library");
    library.index
           .lock()
           .unwrap()
           .replace_all_items(items)
           .unwrap();
    Arc::new(library)
}

#[test]
fn a_downloaded_file_is_recorded_in_the_location_the_rules_choose() {
    let library = open_library(&[downloaded_item("a", "Paizo"), downloaded_item("b", "Other")]);

    library.record_file_locations(&storage(), "a");
    library.record_file_locations(&storage(), "b");

    let locations = library.file_locations_by_item();
    assert_eq!(locations.get("a"), Some(&vec!["Vault".to_owned()]));
    assert_eq!(locations.get("b").map(Vec::len), Some(1));
    assert_ne!(locations.get("b"), locations.get("a"));
}

#[test]
fn the_record_stays_when_the_item_is_written_again() {
    let item = downloaded_item("a", "Paizo");
    let library = open_library(std::slice::from_ref(&item));
    library.record_file_locations(&storage(), "a");

    library.index.lock().unwrap().replace_item(&item).unwrap();

    assert_eq!(library.file_locations_by_item().get("a"),
               Some(&vec!["Vault".to_owned()]));
}

#[test]
fn the_record_stays_when_the_rules_change_later() {
    let library = open_library(&[downloaded_item("a", "Paizo")]);
    library.record_file_locations(&storage(), "a");
    let mut changed = storage();
    changed.rules.clear();

    library.record_missing_file_locations(&changed);

    assert_eq!(library.file_locations_by_item().get("a"),
               Some(&vec!["Vault".to_owned()]));
}

#[test]
fn items_downloaded_before_locations_were_recorded_get_one() {
    let library = open_library(&[downloaded_item("a", "Paizo"), downloaded_item("b", "Paizo")]);
    library.record_file_locations(&storage(), "a");

    library.record_missing_file_locations(&storage());

    assert_eq!(library.file_locations_by_item().len(), 2);
}

#[test]
fn clearing_forgets_the_item() {
    let library = open_library(&[downloaded_item("a", "Paizo")]);
    library.record_file_locations(&storage(), "a");

    library.clear_file_locations("a");

    assert!(library.file_locations_by_item().is_empty());
}
