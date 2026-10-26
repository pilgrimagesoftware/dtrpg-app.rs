//! Reads a catalog item from the index with its ephemeral overlay applied,
//! and the two tracking maps that overlay comes from.

use std::sync::{Arc, PoisonError};
use std::time::SystemTime;

use crate::data::library::LibraryItem;
use crate::library::Library;

impl Library {
    /// Sets `item.thumbnail_last_attempted`/`availability_last_checked` from
    /// the session-only tracking maps, since the index never persists
    /// either (see this module's doc comment).
    fn overlay_ephemeral_state(&self, item: &mut LibraryItem) {
        let ephemeral = self.ephemeral
                            .lock()
                            .unwrap_or_else(PoisonError::into_inner);
        item.thumbnail_last_attempted = ephemeral.thumbnail_attempts.get(&item.id).copied();
        item.availability_last_checked = ephemeral.availability_checks.get(&item.id).copied();
    }

    /// Overlays the ephemeral state and the file locations on `items`.
    #[track_caller]
    pub(super) fn overlaid_items(&self, mut items: Vec<LibraryItem>) -> Vec<LibraryItem> {
        let mut locations = self.file_locations_by_item();
        let mut collections = self.collection_names_by_member();
        let pages = self.file_pages_by_item();
        for item in &mut items {
            self.overlay_ephemeral_state(item);
            item.pages = pages.get(item.id.as_ref()).copied().unwrap_or(item.pages);
            item.collections = collections.remove(&item.order_product_id)
                                          .unwrap_or_default();
            item.locations = locations.remove(&item.id)
                                      .unwrap_or_default()
                                      .into_iter()
                                      .map(Arc::from)
                                      .collect();
        }
        items
    }

    /// Returns every item in the catalog, with ephemeral state overlaid.
    ///
    /// The one place every other catalog-reading method should go through,
    /// so none of them forget the overlay.
    #[track_caller]
    pub fn all_items_scan(&self) -> Vec<LibraryItem> {
        let items = self.index
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .list_items()
                        .unwrap_or_default();
        self.overlaid_items(items)
    }

    /// Looks up a catalog item by id, with ephemeral state overlaid.
    ///
    /// # Errors
    ///
    /// Returns `None` on a missing item or an index query failure (logged).
    #[must_use]
    pub fn item_by_id(&self, id: &str) -> Option<LibraryItem> {
        let mut item = self.index
                           .lock()
                           .unwrap_or_else(PoisonError::into_inner)
                           .get_item(id)
                           .map_err(|e| tracing::warn!(error = %e, "library index query failed"))
                           .ok()
                           .flatten()?;
        self.overlay_ephemeral_state(&mut item);
        item.pages = self.index
                         .lock()
                         .unwrap_or_else(PoisonError::into_inner)
                         .file_pages_of(id)
                         .map_err(|e| tracing::warn!(error = %e, "library index query failed"))
                         .ok()
                         .flatten()
                         .unwrap_or(item.pages);
        item.locations = self.index
                             .lock()
                             .unwrap_or_else(PoisonError::into_inner)
                             .file_locations_of(id)
                             .map_err(|e| tracing::warn!(error = %e, "library index query failed"))
                             .unwrap_or_default()
                             .into_iter()
                             .map(Arc::from)
                             .collect();
        item.collections = self.collection_names_by_member()
                               .remove(&item.order_product_id)
                               .unwrap_or_default();
        Some(item)
    }

    /// Records that a thumbnail fetch for item `id` was attempted at `at`,
    /// in the session-only tracking map and in the cached filtered result
    /// set.
    ///
    /// Does not recompute the result set, as `invalidate_cache` would: the
    /// time is in neither the filter nor the sort, and a cover arrives for
    /// every item scrolled into view.
    pub fn note_thumbnail_attempt(&self, id: &Arc<str>, at: SystemTime) {
        self.ephemeral
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .thumbnail_attempts
            .insert(Arc::clone(id), at);

        let mut cache = self.query_cache
                            .lock()
                            .unwrap_or_else(PoisonError::into_inner);
        let Some(cached) = cache.items.as_mut()
        else {
            return;
        };
        if let Some(item) = Arc::make_mut(cached).iter_mut().find(|item| item.id == *id) {
            item.thumbnail_last_attempted = Some(at);
        }
    }

    /// Records that item `id`'s availability was checked at `at`, in the
    /// session-only tracking map. Unlike [`Self::note_thumbnail_attempt`],
    /// does not patch the cached result set — callers that change an item's
    /// status (which the filter/sort can depend on) always follow this with
    /// `invalidate_cache`.
    pub fn note_availability_check(&self, id: &Arc<str>, at: SystemTime) {
        self.ephemeral
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .availability_checks
            .insert(Arc::clone(id), at);
    }
}
