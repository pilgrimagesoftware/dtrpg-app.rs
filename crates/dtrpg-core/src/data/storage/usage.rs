//! Which storage root holds each title, and how many titles each root holds.

use std::collections::HashMap;
use std::sync::Arc;

use super::StorageConfig;
use crate::data::collection::CollectionEntry;
use crate::data::enums::ItemStatus;
use crate::data::library::LibraryItem;
use crate::util::file_presence::collection_ids_for_item;

/// How many downloaded titles each root holds, by root id, going by the
/// placement rules: a title counts for the root the rules send it to. A root
/// with no titles is not in the map.
///
/// The rules say where a download goes; this does not look at the disk, so a
/// file moved by hand, or downloaded before a rule changed, counts for where
/// the rules would put it now.
#[must_use]
pub fn titles_per_root(storage: &StorageConfig, items: &[LibraryItem],
                       collections: &[CollectionEntry])
                       -> HashMap<u32, usize> {
    let mut counts = HashMap::new();
    for item in items.iter()
                     .filter(|item| item.status == ItemStatus::Downloaded)
    {
        let collection_ids = collection_ids_for_item(item, collections);
        let root = storage.resolve_root_for(&item.publisher, &collection_ids, item.size_mb);
        *counts.entry(root.id).or_insert(0) += 1;
    }
    counts
}

/// The name of the root each downloaded title is in, by item id, going by the
/// placement rules as [`titles_per_root`] does. A title that is only in the
/// cloud is not in the map.
#[must_use]
pub fn locations_of_titles(storage: &StorageConfig, items: &[LibraryItem],
                           collections: &[CollectionEntry])
                           -> HashMap<Arc<str>, String> {
    items.iter()
         .filter(|item| item.status == ItemStatus::Downloaded)
         .map(|item| {
             let collection_ids = collection_ids_for_item(item, collections);
             let root = storage.resolve_root_for(&item.publisher, &collection_ids, item.size_mb);
             (Arc::clone(&item.id), root.name)
         })
         .collect()
}
