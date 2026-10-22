//! Which items belong to a smart collection.
//!
//! These read the whole catalog, so they are for the moments the catalog or
//! the collection changes, never for the render path.

use std::collections::HashSet;
use std::sync::Arc;

use crate::constants::SMART_PREVIEW_TITLE_LIMIT;
use crate::data::smart_collection::SmartCollection;
use crate::library::Library;

/// What a smart collection matches: the counts, and a few titles to show.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SmartPreview {
    /// How many items match.
    pub matching: usize,
    /// How many items the catalog holds.
    pub total:    usize,
    /// The first matching titles, alphabetically, at most
    /// [`SMART_PREVIEW_TITLE_LIMIT`] of them.
    pub titles:   Vec<Arc<str>>,
}

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

    /// What `collection` matches right now: how many items, of how many in the
    /// catalog, and the first few titles in alphabetical order. This is the
    /// "Matches N of M titles" and the preview list of the editor.
    #[must_use]
    pub fn smart_collection_preview(&self, collection: &SmartCollection) -> SmartPreview {
        let tags = self.tags();
        let items = self.all_items();
        let mut titles: Vec<Arc<str>> =
            items.iter()
                 .filter(|item| collection.contains(item, tags.tag_names_of(&item.id)))
                 .map(|item| Arc::clone(&item.title))
                 .collect();
        let matching = titles.len();
        titles.sort_by_cached_key(|title| title.to_lowercase());
        titles.truncate(SMART_PREVIEW_TITLE_LIMIT);
        SmartPreview { matching,
                       total: items.len(),
                       titles }
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
