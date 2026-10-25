//! The user's smart collections: keeping them, counting their members, and
//! showing one in the catalog.

use dtrpg_core::data::smart_collection::SmartCollection;
use dtrpg_core::library::SmartPreview;
use dtrpg_core::util::filter::SidebarFilter;

use super::super::*;

impl LibraryController {
    /// The smart collections, in the order they were added.
    #[must_use]
    pub fn smart_collections(&self) -> &[SmartCollection] {
        self.smart_collections.all()
    }

    /// How many items belong to the smart collection with `id`, as of the last
    /// change to the catalog or the collections.
    #[must_use]
    pub fn smart_collection_count(&self, id: u64) -> usize {
        self.smart_counts.get(&id).copied().unwrap_or_default()
    }

    /// What `collection` matches, for the editor's live count and title list.
    /// Reads the whole catalog, so call it when the collection changes, not
    /// while drawing.
    #[must_use]
    pub fn smart_collection_preview(&self, collection: &SmartCollection) -> SmartPreview {
        self.library.smart_collection_preview(collection)
    }

    /// Adds `collection` and returns its id. The collection is kept in
    /// memory even when it could not be saved, and the failure is logged.
    pub fn add_smart_collection(&mut self, collection: SmartCollection, cx: &mut Context<Self>)
                                -> u64 {
        let id = match self.smart_collections.add(collection) {
            Ok(id) => id,
            Err(error) => {
                tracing::error!(%error, "could not save the smart collection");
                self.smart_collections
                    .all()
                    .last()
                    .map_or(0, |added| added.id)
            }
        };
        self.smart_collections_changed(cx);
        id
    }

    /// Replaces the smart collection that has `collection`'s id.
    pub fn update_smart_collection(&mut self, collection: SmartCollection, cx: &mut Context<Self>) {
        self.match_tracker.forget(collection.id);
        if let Some(before) = self.smart_collections.get(collection.id).cloned()
           && (before.name != collection.name || !collection.mirror_as_symlinks)
        {
            self.remove_mirror(&before, cx);
        }
        if let Err(error) = self.smart_collections.update(collection) {
            tracing::error!(%error, "could not save the smart collection");
        }
        self.smart_collections_changed(cx);
    }

    /// Deletes the smart collection with `id`. If it was on show, the catalog
    /// goes back to all titles.
    pub fn delete_smart_collection(&mut self, id: u64, cx: &mut Context<Self>) {
        self.match_tracker.forget(id);
        if let Some(before) = self.smart_collections.get(id).cloned() {
            self.remove_mirror(&before, cx);
        }
        if let Err(error) = self.smart_collections.remove(id) {
            tracing::error!(%error, "could not save the smart collections");
        }
        if matches!(&self.filter, SidebarFilter::SmartCollection(shown, _) if *shown == id) {
            self.set_filter(SidebarFilter::AllTitles, cx);
        }
        self.smart_collections_changed(cx);
    }

    /// Recounts the members, and refreshes the catalog if a smart collection
    /// is on show, after the collections or their rules changed.
    fn smart_collections_changed(&mut self, cx: &mut Context<Self>) {
        self.refresh_smart_collections();
        self.invalidate_cache();
        self.run_smart_actions(cx);
        cx.emit(LibraryChanged);
    }

    /// Recounts the members of every smart collection and, if one is on show,
    /// recomputes which items it holds. Reads the whole catalog, so it runs
    /// when the catalog or the tags change, not on every keystroke.
    pub(crate) fn refresh_smart_collections(&mut self) {
        let collections = self.smart_collections.all();
        let counts = self.library.smart_collection_counts(collections);
        self.smart_counts = collections.iter()
                                       .map(|collection| collection.id)
                                       .zip(counts)
                                       .collect();
        self.refresh_smart_members();
    }

    /// Recomputes the items held by the smart collection on show, if one is.
    /// A collection that no longer exists sends the catalog back to all
    /// titles.
    pub(crate) fn refresh_smart_members(&mut self) {
        let SidebarFilter::SmartCollection(id, _) = &self.filter
        else {
            return;
        };
        match self.smart_collections.get(*id) {
            Some(collection) => {
                self.collection_members = self.library.smart_collection_member_ids(collection);
            }
            None => {
                self.collection_members.clear();
                self.filter = SidebarFilter::AllTitles;
            }
        }
    }
}
