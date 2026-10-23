//! `Library` methods that record which storage location each downloaded file
//! was put in.

use std::collections::HashMap;
use std::sync::{Arc, PoisonError};

use crate::data::storage::StorageConfig;
use crate::library::Library;
use crate::util::file_presence::collection_ids_for_item;

impl Library {
    /// Records the storage location of each downloaded file of `item_id`, as
    /// the placement rules in `storage` choose it now, replacing what was
    /// recorded.
    ///
    /// Called when a download finishes, so the record says where the file was
    /// put and stays true when the rules change afterwards. A failure to write
    /// is logged, not returned: the record is rebuilt by
    /// [`Self::record_missing_file_locations`] on a later run.
    pub fn record_file_locations(&self, storage: &StorageConfig, item_id: &str) {
        let Some(item) = self.item_by_id(item_id)
        else {
            return;
        };
        let collection_ids = collection_ids_for_item(&item, &self.collections());
        let locations: Vec<(String, String)> = item.files
                                                   .iter()
                                                   .filter(|file| file.downloaded)
                                                   .map(|file| {
                                                       let root =
                        storage.resolve_root_for(&item.publisher, &collection_ids, file.size_mb);
                                                       (file.name.to_string(), root.name)
                                                   })
                                                   .collect();
        let written = self.index
                          .lock()
                          .unwrap_or_else(PoisonError::into_inner)
                          .set_file_locations(item_id, &locations);
        if let Err(error) = written {
            tracing::error!(%error, "failed to record file locations in the library index");
        }
    }

    /// Forgets where `item_id`'s files were, for when its downloads are
    /// removed.
    pub fn clear_file_locations(&self, item_id: &str) {
        let cleared = self.index
                          .lock()
                          .unwrap_or_else(PoisonError::into_inner)
                          .clear_file_locations(item_id);
        if let Err(error) = cleared {
            tracing::error!(%error, "failed to clear file locations in the library index");
        }
    }

    /// Records locations for downloaded items that have none, which are those
    /// downloaded before locations were recorded. Their location is taken from
    /// the placement rules in `storage`, the best that is known.
    #[track_caller]
    pub fn record_missing_file_locations(&self, storage: &StorageConfig) {
        let missing = self.index
                          .lock()
                          .unwrap_or_else(PoisonError::into_inner)
                          .downloaded_items_without_location()
                          .map_err(|error| tracing::warn!(%error, "library index query failed"))
                          .unwrap_or_default();
        for item_id in missing {
            self.record_file_locations(storage, &item_id);
        }
    }

    /// The names of the storage locations each downloaded item has files in, by
    /// item id.
    #[must_use]
    #[track_caller]
    pub fn file_locations_by_item(&self) -> HashMap<Arc<str>, Vec<String>> {
        let by_item = self.index
                          .lock()
                          .unwrap_or_else(PoisonError::into_inner)
                          .file_locations_by_item()
                          .map_err(|error| tracing::warn!(%error, "library index query failed"))
                          .unwrap_or_default();
        by_item.into_iter()
               .map(|(item_id, locations)| (Arc::from(item_id), locations))
               .collect()
    }
}

#[cfg(test)]
mod tests;
