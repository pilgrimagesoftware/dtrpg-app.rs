//! Tests for recording the page count of each downloaded file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;
use std::sync::Arc;

use lopdf::{Document, Object, Stream, dictionary};

use crate::data::enums::ItemStatus;
use crate::data::library::{LibraryItem, LibraryItemFile};
use crate::data::storage::StorageConfig;
use crate::library::Library;
use crate::util::file_presence::resolved_file_path;

fn downloaded_item(id: &str) -> LibraryItem {
    let mut item = LibraryItem::new(id,
                                    "Title",
                                    "Paizo",
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
    item.files = vec![LibraryItemFile { id:         "f".into(),
                                        index:      0,
                                        name:       "book.pdf".into(),
                                        format:     "PDF".into(),
                                        size_mb:    1.0,
                                        downloaded: true,
                                        offloaded:  false, }];
    item
}

fn write_pdf(path: &Path, pages: usize) {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let kids: Vec<Object> = (0..pages).map(|_| {
                                          let content =
                                              doc.add_object(Stream::new(dictionary! {}, vec![]));
                                          doc.add_object(dictionary! {
                                                             "Type" => "Page",
                                                             "Parent" => pages_id,
                                                             "Contents" => content,
                                                         })
                                             .into()
                                      })
                                      .collect();
    doc.objects.insert(pages_id,
                       Object::Dictionary(dictionary! {
                                              "Type" => "Pages",
                                              "Count" => i64::try_from(pages).unwrap(),
                                              "Kids" => kids,
                                          }));
    let catalog = doc.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    doc.trailer.set("Root", catalog);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    doc.save(path).unwrap();
}

struct Setup {
    library: Arc<Library>,
    storage: StorageConfig,
    item:    LibraryItem,
}

fn setup(pdf_pages: Option<usize>) -> Setup {
    let storage_dir = tempfile::tempdir().unwrap();
    let storage = StorageConfig::for_test(storage_dir.keep());
    let item = downloaded_item("a");
    if let Some(pages) = pdf_pages {
        write_pdf(&resolved_file_path(&storage, &item, &item.files[0]), pages);
    }
    let datadir = tempfile::tempdir().unwrap();
    let cachedir = tempfile::tempdir().unwrap();
    let (library, _work, _sync) =
        Library::open(datadir.keep(), cachedir.keep(), &storage).expect("open library");
    library.index
           .lock()
           .unwrap()
           .replace_all_items(std::slice::from_ref(&item))
           .unwrap();
    Setup { library: Arc::new(library),
            storage,
            item }
}

#[test]
fn the_pages_of_a_downloaded_pdf_are_counted_from_the_file() {
    let setup = setup(Some(3));

    setup.library.record_page_counts(&setup.storage, "a");

    assert_eq!(setup.library.item_by_id("a").unwrap().pages, 3);
}

#[test]
fn the_total_matches_the_pages_the_items_show() {
    let setup = setup(Some(3));
    setup.library.record_page_counts(&setup.storage, "a");

    setup.library.refresh_section_counts(0, 30);

    assert_eq!(setup.library.total_pages(),
               u64::from(setup.library.item_by_id("a").unwrap().pages));
}

#[test]
fn the_count_stays_when_the_item_is_written_again() {
    let setup = setup(Some(3));
    setup.library.record_page_counts(&setup.storage, "a");

    setup.library
         .index
         .lock()
         .unwrap()
         .replace_item(&setup.item)
         .unwrap();

    assert_eq!(setup.library.item_by_id("a").unwrap().pages, 3);
}

#[test]
fn a_file_that_cannot_be_read_counts_zero_and_is_not_read_again() {
    let setup = setup(None);

    assert!(setup.library.record_missing_page_counts(&setup.storage));
    assert!(!setup.library.record_missing_page_counts(&setup.storage));
    assert_eq!(setup.library.item_by_id("a").unwrap().pages, 0);
}

#[test]
fn items_downloaded_before_pages_were_recorded_get_counted() {
    let setup = setup(Some(2));

    let recorded = setup.library.record_missing_page_counts(&setup.storage);

    assert!(recorded);
    assert_eq!(setup.library.item_by_id("a").unwrap().pages, 2);
}
