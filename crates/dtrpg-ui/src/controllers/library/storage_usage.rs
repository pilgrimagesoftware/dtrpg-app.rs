//! Which storage location holds each title, and how many each holds, for the
//! Storage settings page and the inspector.

use std::collections::HashMap;

use dtrpg_core::data::storage::{StorageConfig, titles_per_root};

use super::*;

impl LibraryController {
    /// How many downloaded titles each storage location holds, by location id,
    /// as of the last change to the catalog, the collections or the storage
    /// settings.
    #[must_use]
    pub const fn root_title_counts(&self) -> &HashMap<u32, usize> {
        &self.root_title_counts
    }

    /// The name of the location the downloaded title `item_id` is in, as of the
    /// last change to the catalog, the collections or the storage settings.
    /// `None` for a title that is not downloaded.
    #[must_use]
    pub fn item_location(&self, item_id: &str) -> Option<&str> {
        self.item_locations.get(item_id).map(String::as_str)
    }

    /// The names of the collections, by id, for describing placement rules.
    #[must_use]
    pub fn collection_names(&self) -> HashMap<u64, Arc<str>> {
        self.library
            .collections()
            .iter()
            .map(|collection| (collection.id, Arc::clone(&collection.name)))
            .collect()
    }

    /// Reads where each downloaded title's files are kept, and counts the
    /// titles in each location. Downloads from before locations were
    /// recorded get one first, from the placement rules. Reads the index
    /// and the storage settings, so it runs when those change, not while
    /// drawing.
    pub fn refresh_root_title_counts(&mut self) {
        let storage = StorageConfig::load();
        self.library.record_missing_file_locations(&storage);
        let recorded = self.library.file_locations_by_item();
        self.root_title_counts = titles_per_root(&recorded, &storage.roots());
        self.item_locations = recorded.into_iter()
                                      .map(|(item_id, locations)| (item_id, locations.join(", ")))
                                      .collect();
    }
}
