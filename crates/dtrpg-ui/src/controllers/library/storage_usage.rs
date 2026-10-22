//! Which storage location holds each title, and how many each holds, for the
//! Storage settings page and the inspector.

use std::collections::HashMap;

use dtrpg_core::data::storage::{StorageConfig, locations_of_titles, titles_per_root};

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

    /// Counts the titles in each location. Reads the catalog and the storage
    /// settings, so it runs when those change, not while drawing.
    pub fn refresh_root_title_counts(&mut self) {
        let storage = StorageConfig::load();
        let items = self.library.all_items();
        let collections = self.library.collections();
        self.root_title_counts = titles_per_root(&storage, &items, &collections);
        self.item_locations = locations_of_titles(&storage, &items, &collections);
    }
}
