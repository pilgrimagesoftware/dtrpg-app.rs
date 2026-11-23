// File naming and dedupe tests for `map_files`.

use super::super::SdkLibraryService;
use super::mapping_test_support::*;
use super::*;
use crate::services::LibraryService;

#[test]
fn map_order_product_names_files_by_filename_not_title() {
    let mut item = order_product_item(515_276, "Moria");
    item.attributes.files = vec![OrderProductFile { index:                     0,
                                                    order_product_download_id: 1234,
                                                    title:                     "PDF".to_string(),
                                                    filename:
                                                        "moria.pdf".to_string(),
                                                    size:                      1_048_576,
                                                    size_mb:                   "1.0".to_string(),
                                                    checksums:                 vec![], },
                                 OrderProductFile { index:                     1,
                                                    order_product_download_id: 1235,
                                                    title:                     "Map".to_string(),
                                                    filename:                  String::new(),
                                                    size:                      524_288,
                                                    size_mb:                   "0.5".to_string(),
                                                    checksums:                 vec![], },];

    let service = SdkLibraryService::new(Box::new(FakeSdkGateway::seeded_with(item)));
    let items = service.list_items().expect("list items");

    assert_eq!(items[0].files[0].name.as_ref(), "moria.pdf");
    // An empty filename falls back to the title rather than an empty path
    // component.
    assert_eq!(items[0].files[1].name.as_ref(), "Map");
}

fn download(id: u64, title: &str, filename: &str) -> OrderProductFile {
    OrderProductFile { index:                     0,
                       order_product_download_id: id,
                       title:                     title.to_string(),
                       filename:                  filename.to_string(),
                       size:                      1,
                       size_mb:                   "0.0".to_string(),
                       checksums:                 vec![], }
}

#[test]
fn map_order_product_keeps_files_that_share_a_filename_but_not_a_name_key() {
    let mut item = order_product_item(515_277, "Dupes");
    item.attributes.files = vec![download(7, "A", "same.pdf"),
                                 download(7, "B", "same.pdf"),
                                 download(7, "C", "other.pdf"),];

    let service = SdkLibraryService::new(Box::new(FakeSdkGateway::seeded_with(item)));
    let items = service.list_items().expect("list items");

    // Same id + same filename is one file; a different filename is another.
    let names: Vec<&str> = items[0].files.iter().map(|f| f.name.as_ref()).collect();
    assert_eq!(names, ["same.pdf", "other.pdf"]);
}

#[test]
fn map_order_product_treats_a_whitespace_filename_as_missing() {
    let mut item = order_product_item(515_278, "Blank");
    item.attributes.files = vec![download(9, "Handbook", "   ")];

    let service = SdkLibraryService::new(Box::new(FakeSdkGateway::seeded_with(item)));
    let items = service.list_items().expect("list items");

    assert_eq!(items[0].files[0].name.as_ref(), "Handbook");
    assert_eq!(items[0].files[0].format.as_ref(), "FILE");
}
