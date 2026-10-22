//! The manual collections as the `collection:` search term reads them: the
//! names of the collections an item is in, and the members of each name.

use std::collections::HashMap;
use std::sync::Arc;

use crate::library::Library;

impl Library {
    /// The names of the collections each member is in, by member id (an item's
    /// `order_product_id`), for filling in `LibraryItem::collections`.
    pub(super) fn collection_names_by_member(&self) -> HashMap<u64, Vec<Arc<str>>> {
        let mut names: HashMap<u64, Vec<Arc<str>>> = HashMap::new();
        for collection in self.collections().iter() {
            for member in collection.member_ids.iter() {
                names.entry(*member)
                     .or_default()
                     .push(Arc::clone(&collection.name));
            }
        }
        names
    }

    /// The member ids of each collection, by lower-cased name, for the SQL side
    /// of the `collection:` term. Collections that share a name share an entry.
    pub(super) fn collection_ids_by_name(&self) -> HashMap<String, Vec<u64>> {
        let mut ids: HashMap<String, Vec<u64>> = HashMap::new();
        for collection in self.collections().iter() {
            ids.entry(collection.name.to_lowercase())
               .or_default()
               .extend(collection.member_ids.iter().copied());
        }
        ids
    }
}
