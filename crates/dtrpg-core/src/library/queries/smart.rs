//! Which items belong to a smart collection.
//!
//! These read the whole catalog, so they are for the moments the catalog or
//! the collection changes, never for the render path.

use std::collections::HashSet;

use crate::data::smart_collection::SmartCollection;
use crate::library::Library;

impl Library {
    /// The product ids of the items in `collection`, in the form the catalog
    /// filter takes: each member's order product id and product id, where it
    /// has one.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn demo(library: &dtrpg_core::library::Library,
    /// #         collection: &dtrpg_core::data::smart_collection::SmartCollection) {
    /// let members = library.smart_collection_member_ids(collection);
    /// println!("{} ids", members.len());
    /// # }
    /// ```
    #[must_use]
    pub fn smart_collection_member_ids(&self, collection: &SmartCollection) -> HashSet<u64> {
        let tags = self.tags();
        let mut ids = HashSet::new();
        for item in self.members_of(collection, &tags) {
            ids.extend([item.order_product_id, item.product_id].into_iter()
                                                               .filter(|id| *id > 0));
        }
        ids
    }

    /// How many items, of how many in the catalog, belong to `collection`.
    /// This is the "Matches N of M titles" of the editor.
    #[must_use]
    pub fn smart_collection_match_count(&self, collection: &SmartCollection) -> (usize, usize) {
        let tags = self.tags();
        let items = self.all_items();
        let matching = items.iter()
                            .filter(|item| collection.contains(item, tags.tag_names_of(&item.id)))
                            .count();
        (matching, items.len())
    }

    /// How many catalog items belong to each of `collections`, in the same
    /// order, reading the catalog once for all of them.
    #[must_use]
    pub fn smart_collection_counts(&self, collections: &[SmartCollection]) -> Vec<usize> {
        let tags = self.tags();
        let mut counts = vec![0; collections.len()];
        for item in self.all_items() {
            let item_tags = tags.tag_names_of(&item.id);
            for (count, collection) in counts.iter_mut().zip(collections) {
                *count += usize::from(collection.contains(&item, item_tags));
            }
        }
        counts
    }

    /// The catalog items that belong to `collection`.
    fn members_of(&self, collection: &SmartCollection,
                  tags: &crate::data::tag_library::TagLibrary)
                  -> Vec<crate::data::library::LibraryItem> {
        self.all_items()
            .into_iter()
            .filter(|item| collection.contains(item, tags.tag_names_of(&item.id)))
            .collect()
    }
}
