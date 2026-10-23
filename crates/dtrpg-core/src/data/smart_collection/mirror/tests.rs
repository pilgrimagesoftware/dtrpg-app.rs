//! Tests for the symlink mirror of a smart collection.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;

#[cfg(unix)]
use super::sync_mirror;
use super::{clear_mirror, smart_collection_dir};

#[cfg(unix)]
fn file_in(root: &Path, name: &str) -> std::path::PathBuf {
    let path = root.join("items").join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, b"x").unwrap();
    path
}

#[test]
fn the_folder_is_named_for_the_collection_under_its_own_directory() {
    let dir = smart_collection_dir(Path::new("/lib"), "Solo play");

    assert_eq!(dir, Path::new("/lib/smart-collections/Solo_play"));
}

#[cfg(unix)]
#[test]
fn a_link_is_made_for_each_file() {
    let root = tempfile::tempdir().unwrap();
    let (a, b) = (file_in(root.path(), "a.pdf"), file_in(root.path(), "b.pdf"));
    let dir = root.path().join("mirror");

    let links = sync_mirror(&dir, &[a.clone(), b]).unwrap();

    assert_eq!(links, 2);
    assert_eq!(std::fs::read_link(dir.join("a.pdf")).unwrap(), a);
}

#[cfg(unix)]
#[test]
fn a_link_to_a_file_that_left_the_collection_is_removed() {
    let root = tempfile::tempdir().unwrap();
    let (a, b) = (file_in(root.path(), "a.pdf"), file_in(root.path(), "b.pdf"));
    let dir = root.path().join("mirror");
    sync_mirror(&dir, &[a.clone(), b]).unwrap();

    sync_mirror(&dir, &[a]).unwrap();

    assert!(dir.join("b.pdf").symlink_metadata().is_err());
}

#[cfg(unix)]
#[test]
fn syncing_twice_changes_nothing() {
    let root = tempfile::tempdir().unwrap();
    let a = file_in(root.path(), "a.pdf");
    let dir = root.path().join("mirror");
    sync_mirror(&dir, std::slice::from_ref(&a)).unwrap();

    assert_eq!(sync_mirror(&dir, &[a]).unwrap(), 1);
}

#[cfg(unix)]
#[test]
fn a_plain_file_in_the_folder_is_left_alone() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path().join("mirror");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("notes.txt"), b"mine").unwrap();

    sync_mirror(&dir, &[]).unwrap();

    assert!(dir.join("notes.txt").is_file());
}

#[cfg(unix)]
#[test]
fn clearing_removes_the_links_and_the_empty_folder() {
    let root = tempfile::tempdir().unwrap();
    let a = file_in(root.path(), "a.pdf");
    let dir = root.path().join("mirror");
    sync_mirror(&dir, std::slice::from_ref(&a)).unwrap();

    clear_mirror(&dir);

    assert!(!dir.exists());
    assert!(a.is_file());
}

#[test]
fn clearing_a_missing_folder_is_fine() {
    clear_mirror(Path::new("/definitely/not/here"));
}
