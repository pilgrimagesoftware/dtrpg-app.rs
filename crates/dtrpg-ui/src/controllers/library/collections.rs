//! Collection CRUD, membership changes, and the collections service swap.

use super::collection_links::{link_item_into_collection, unlink_item_from_collection};
use super::*;

impl LibraryController {
    /// Spawns a background task to fetch product lists from the API and apply
    /// them.
    pub fn load_collections(&mut self, cx: &mut Context<Self>) {
        let collections_service = Arc::clone(&self.collections_service);
        let cache_root = cache_dir();
        let weak_activity = self.activity.downgrade();
        let activity_id = self.activity.update(cx, |a, cx| {
                                           a.start(&t!("activity.loading_collections"), None, cx)
                                       });
        tracing::debug!("load_collections: starting collections fetch");
        cx.spawn(async move |this, async_cx| {
              let result = async_cx.background_executor()
                                   .spawn(async move { collections_service.list_collections() })
                                   .await;
              match result {
                  Ok(entries) => {
                      tracing::debug!(count = entries.len(),
                                      "load_collections: fetched {} entries",
                                      entries.len());
                      let to_save = entries.clone();
                      let save_root = cache_root.clone();
                      async_cx
                        .background_executor()
                        .spawn(async move {
                            if let Err(e) = save_collections_cache(&save_root, &to_save) {
                                tracing::warn!(error = %e, "failed to save collections cache");
                            }
                        })
                        .await;
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.apply_collections(entries, cx);
                          })
                          .ok();
                      weak_activity.update(async_cx, |a, cx| a.complete(activity_id, cx))
                                   .ok();
                  }
                  Err(e) => {
                      // Session errors are expected when starting before auth
                      // completes
                      // (see `start_load_inner`'s matching treatment for the
                      // catalog fetch); quietly complete
                      // rather than surfacing a user-facing error.
                      if e.kind == CollectionsServiceErrorKind::Session {
                          tracing::warn!(error = %e,
                                      "collections load skipped: no authenticated session");
                          weak_activity.update(async_cx, |a, cx| a.complete(activity_id, cx))
                                       .ok();
                      }
                      else {
                          tracing::warn!(error = %e, "collections load failed");
                          weak_activity.update(async_cx, |a, cx| {
                                           a.error(activity_id, e.to_string(), cx)
                                       })
                                       .ok();
                      }
                  }
              }
          })
          .detach();
    }

    /// Brings what depends on which items are in which collections up to date:
    /// the smart collections' counts and members, and the catalog, since a
    /// `collection:` search reads the membership.
    pub(super) fn membership_changed(&mut self) {
        self.refresh_smart_collections();
        self.invalidate_cache();
    }

    /// Stores the fetched collections and emits a change event.
    pub(super) fn apply_collections(&mut self, mut collections: Vec<CollectionEntry>,
                                    cx: &mut Context<Self>) {
        let catalog_ids = self.catalog_ids_scan();
        sort_collections(&mut collections,
                         self.collection_sort,
                         self.collection_sort_direction,
                         &catalog_ids);
        self.library.set_collections(collections);
        self.refresh_collection_cloud();
        // Refresh collection_members if the current filter is a Collection
        // filter, in case the filter was set before the collections
        // loaded.
        if let SidebarFilter::Collection(id, _) = &self.filter {
            self.collection_members = self.library.collection_member_ids(*id);
        }
        self.membership_changed();
        cx.emit(LibraryChanged);
    }

    /// The collections as they are now, for the views and for work started
    /// from them. A cheap `Arc` clone; edits go through `Library`.
    #[must_use]
    pub fn collections(&self) -> Arc<Vec<CollectionEntry>> {
        self.library.collections()
    }

    /// Returns the current pane state from the service layer.
    pub fn pane_state(&self) -> &LibraryPaneState {
        self.vm.pane_state()
    }

    /// Forgets the stored library, its collections and the selection, so that
    /// whoever signs in next does not see this account's titles. Called when
    /// the user signs out.
    pub fn forget_library(&mut self, cx: &mut Context<Self>) {
        self.library.clear_catalog();
        self.library.clear_collections();
        self.collection_members.clear();
        self.selection = Selection::default();
        self.invalidate_cache();
        cx.emit(LibraryChanged);
    }

    /// Replaces the backing services and triggers a fresh catalog and
    /// collections load.
    ///
    /// Clears the activity panel so stale error messages from the previous
    /// (unauthenticated) service do not persist after sign-in.
    ///
    /// Sets `catalog_loading` so the catalog view shows that it is loading
    /// while `start_load` checks the library against the service.
    ///
    /// The catalog is kept. It is the on-disk copy of the library, which the
    /// load reads first (to skip or shorten the fetch) and which has to
    /// survive a quit or a crash part way through a load. Clearing it here
    /// threw away everything fetched so far on each launch, because the quiet
    /// sign-in at startup comes through this method. Signing out is what
    /// forgets the library (see [`forget_library`](Self::forget_library)).
    pub fn replace_service(&mut self, service: Box<dyn LibraryService>,
                           collections_service: Box<dyn CollectionsService>,
                           cx: &mut Context<Self>) {
        self.install_services(service, collections_service, cx);
        // Collections are loaded as the first stage of `start_load`, sharing
        // its single activity item rather than starting a separate one.
        self.start_load(cx);
    }

    /// Installs the new services and resets what depended on the old ones,
    /// without starting a load. [`replace_service`](Self::replace_service)
    /// is this followed by the load; tests call this alone, so that they do
    /// not start a background load that outlives them.
    pub(super) fn install_services(&mut self, service: Box<dyn LibraryService>,
                                   collections_service: Box<dyn CollectionsService>,
                                   cx: &mut Context<Self>) {
        tracing::debug!("replace_service: installing authenticated services");
        self.vm.replace_service(service);
        self.collections_service = Arc::from(collections_service);
        self.catalog_loading = true;
        self.library.clear_collections();
        self.collection_members.clear();
        self.selection = Selection::default();
        self.invalidate_cache();
        self.activity.update(cx, |a, cx| a.clear(cx));
        cx.emit(LibraryChanged);
    }

    /// Starts a background task to create a new collection with the given name.
    ///
    /// Tracks progress in the activity panel. On success the new entry is
    /// appended and [`LibraryChanged`] is emitted. On failure
    /// [`CollectionCreateFailed`] is emitted so the window can push an
    /// error notification.
    pub fn create_collection(&mut self, name: String, cx: &mut Context<Self>) {
        self.create_collection_then(name, |_| {}, cx);
    }

    /// As [`Self::create_collection`], and calls `done` with the new entry, or
    /// the reason it could not be made, once the create call has finished.
    pub fn create_collection_then(&mut self, name: String,
                                  done: impl FnOnce(Result<CollectionEntry, String>) + 'static,
                                  cx: &mut Context<Self>) {
        let label = t!("activity.creating_collection", name = name);
        let activity_id = self.activity.update(cx, |a, cx| a.start(&label, None, cx));

        let collections_service = Arc::clone(&self.collections_service);
        cx.spawn(async move |this, async_cx| {
              let result =
                  async_cx.background_executor()
                          .spawn(async move { collections_service.create_collection(&name) })
                          .await;

              match result {
                  Ok(entry) => {
                      let created = entry.clone();
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.library.push_collection(entry);
                              ctrl.activity
                                  .update(cx, |a, cx| a.complete(activity_id, cx));
                              cx.emit(LibraryChanged);
                          })
                          .ok();
                      done(Ok(created));
                  }
                  Err(e) => {
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.activity.update(cx, |a, cx| {
                                               a.error(activity_id, e.to_string(), cx);
                                           });
                              cx.emit(CollectionCreateFailed { message: e.message.clone(), });
                          })
                          .ok();
                      done(Err(e.message));
                  }
              }
          })
          .detach();
    }

    /// Creates a new collection with the given name and, once the create call
    /// succeeds, immediately adds `item_id` as a member of it.
    ///
    /// `item_id` drives local membership tracking/matching (the optimistic
    /// `member_ids` update); `product_id` is the catalog product id sent on
    /// the network add call. See [`Self::add_item_to_collection`].
    ///
    /// Used by the "New collection…" affordance in the Manage Collections
    /// dialog, so a single user action both creates the collection and adds
    /// the current item to it. On create failure, only
    /// [`CollectionCreateFailed`] is emitted — no add is attempted.
    pub fn create_collection_and_add_member(&mut self, name: String, item_id: u64,
                                            product_id: u64, cx: &mut Context<Self>) {
        let label = t!("activity.creating_collection", name = name);
        let activity_id = self.activity.update(cx, |a, cx| a.start(&label, None, cx));

        let collections_service = Arc::clone(&self.collections_service);
        cx.spawn(async move |this, async_cx| {
              let result =
                  async_cx.background_executor()
                          .spawn(async move { collections_service.create_collection(&name) })
                          .await;

              match result {
                  Ok(entry) => {
                      let collection_id = entry.id;
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.library.push_collection(entry);
                              ctrl.activity
                                  .update(cx, |a, cx| a.complete(activity_id, cx));
                              cx.emit(LibraryChanged);
                              ctrl.add_item_to_collection(collection_id, item_id, product_id, cx);
                          })
                          .ok();
                  }
                  Err(e) => {
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.activity.update(cx, |a, cx| {
                                               a.error(activity_id, e.to_string(), cx);
                                           });
                              cx.emit(CollectionCreateFailed { message: e.message.clone(), });
                          })
                          .ok();
                  }
              }
          })
          .detach();
    }

    /// Deletes the collection with the given id via the service, then removes
    /// it locally.
    ///
    /// Logs failures to the activity panel and leaves the collection in place
    /// on error.
    pub fn delete_collection(&mut self, id: u64, cx: &mut Context<Self>) {
        let label = t!("activity.deleting_collection").to_string();
        let activity_id = self.activity.update(cx, |a, cx| a.start(&label, None, cx));
        let collections_service = Arc::clone(&self.collections_service);
        cx.spawn(async move |this, async_cx| {
              let result = async_cx.background_executor()
                                   .spawn(async move { collections_service.delete_collection(id) })
                                   .await;
              match result {
                  Ok(()) => {
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.library.remove_collection(id);
                              if let SidebarFilter::Collection(cid, _) = &ctrl.filter
                                 && *cid == id
                              {
                                  ctrl.filter = SidebarFilter::default();
                                  ctrl.collection_members.clear();
                              }
                              ctrl.membership_changed();
                              ctrl.activity
                                  .update(cx, |a, cx| a.complete(activity_id, cx));
                              cx.emit(LibraryChanged);
                          })
                          .ok();
                  }
                  Err(e) => {
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.activity.update(cx, |a, cx| {
                                               a.error(activity_id, e.to_string(), cx);
                                           });
                          })
                          .ok();
                  }
              }
          })
          .detach();
    }
}

#[path = "collection_membership.rs"]
mod collection_membership;
