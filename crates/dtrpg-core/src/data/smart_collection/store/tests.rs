//! Tests for this module. A failed unwrap is a failed test, so they may use it.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::*;
use crate::data::smart_collection::{MatchMode, Operator, Rule};
use crate::util::search::Field;

fn adventures() -> SmartCollection {
    SmartCollection { name:                 "Unread Adventures".to_owned(),
                      mode:                 MatchMode::All,
                      rules:                vec![Rule { field:    Field::Type,
                                                        operator: Operator::Is,
                                                        value:    "Adventure".to_owned(), },
                                                 Rule { field:    Field::Tag,
                                                        operator: Operator::IsNot,
                                                        value:    "Running".to_owned(), }],
                      mirror_as_symlinks:   true,
                      download_new_matches: false,
                      id:                   0, }
}

#[test]
fn a_missing_file_is_an_empty_store() {
    let dir = tempfile::tempdir().unwrap();

    let store = SmartCollectionStore::open(&dir.path().join("none.toml")).unwrap();

    assert!(store.all().is_empty());
}

#[test]
fn collections_survive_closing_and_reopening() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nested").join("smart.toml");
    let mut store = SmartCollectionStore::open(&path).unwrap();

    let first = store.add(adventures()).unwrap();
    let second = store.add(SmartCollection { name: "Second".to_owned(),
                                             ..adventures() })
                      .unwrap();
    let reopened = SmartCollectionStore::open(&path).unwrap();

    assert_eq!((first, second), (1, 2));
    assert_eq!(reopened.all().len(), 2);
    assert_eq!(reopened.get(1).unwrap().rules, adventures().rules);
    assert!(reopened.get(1).unwrap().mirror_as_symlinks);
    assert_eq!(reopened.get(2).unwrap().name, "Second");
}

#[test]
fn ids_are_not_reused_after_a_removal_in_the_middle() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = SmartCollectionStore::open(&dir.path().join("smart.toml")).unwrap();
    store.add(adventures()).unwrap();
    store.add(adventures()).unwrap();

    store.remove(1).unwrap();
    let next = store.add(adventures()).unwrap();

    assert_eq!(next, 3);
    assert!(store.get(1).is_none());
}

#[test]
fn an_update_replaces_the_collection_and_an_unknown_id_changes_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("smart.toml");
    let mut store = SmartCollectionStore::open(&path).unwrap();
    let id = store.add(adventures()).unwrap();

    store.update(SmartCollection { id,
                                   name: "Renamed".to_owned(),
                                   ..adventures() })
         .unwrap();
    store.update(SmartCollection { id: 99,
                                   ..adventures() })
         .unwrap();
    let reopened = SmartCollectionStore::open(&path).unwrap();

    assert_eq!(reopened.all().len(), 1);
    assert_eq!(reopened.get(id).unwrap().name, "Renamed");
}

#[test]
fn a_file_that_cannot_be_read_is_set_aside_and_the_store_starts_empty() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("smart_collections.toml");
    std::fs::write(&path, "collections = [ not valid").unwrap();

    assert!(matches!(SmartCollectionStore::open(&path),
                     Err(SmartCollectionError::Parse { .. })));
    let store = SmartCollectionStore::load_or_set_aside(path.clone());

    assert!(store.all().is_empty());
    assert!(dir.path().join("smart_collections.toml.bak").exists());
}

#[test]
fn an_unknown_field_in_the_file_is_an_error_not_a_silent_drop() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("smart.toml");
    std::fs::write(&path,
                   "[[collections]]\nid = 1\nname = \"x\"\nmode = \"all\"\n\
                    download_new_matches = false\nmirror_as_symlinks = false\n\
                    [[collections.rules]]\nfield = \"title\"\noperator = \"is\"\nvalue = \"a\"\n")
        .unwrap();

    assert!(SmartCollectionStore::open(&path).is_err());
}
