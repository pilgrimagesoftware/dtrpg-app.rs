//! Library UI state and interaction controller.

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

use gpui::{BorrowAppContext, Context, Entity};

use crate::controllers::activity::ActivityController;
use crate::data::catalog_cache::{load_cache_metadata, load_catalog_cache, save_catalog_cache};
use crate::data::collection::CollectionEntry;
use crate::data::collections_cache::{load_collections_cache, save_collections_cache};
use crate::data::cover_cache::{load_cached_cover, save_cached_cover};
use crate::data::enums::*;
use crate::data::events::*;
use crate::data::library::*;
use crate::data::paths::{cache_dir, covers_dir};
use crate::data::selection::Selection;
use crate::data::theme::LibriTheme;
use crate::data::theme::*;
use crate::services::collections::CollectionsService;
use crate::services::{LibraryService, LibraryServiceErrorKind};
use crate::ui::library::cover::CoverCache;
use crate::util::filter::*;
use crate::util::matching::*;
use crate::util::publisher::*;
use crate::util::sort::*;
use crate::view_models::library::{LibraryPaneState, LibraryViewModel};

// ── LibraryController
// ─────────────────────────────────────────────────────────

/// Snapshot of all data needed by the root view for a single render pass.
pub struct LibrarySnapshot {
    pub filter:                  SidebarFilter,
    pub counts:                  SectionCounts,
    pub publishers:              Vec<PublisherEntry>,
    pub collections:             Vec<CollectionEntry>,
    /// True once the initial collections fetch for the current session has
    /// completed; the sidebar shows "?" for the collection count until then.
    pub collections_loaded:      bool,
    /// All numeric product IDs in the full catalog; used by the sidebar to
    /// compute per-collection resolved item counts.
    pub catalog_ids:             HashSet<u64>,
    /// Total items in the full catalog (no filter, no search).
    pub total_count:             usize,
    pub total_mb:                f64,
    /// Items matching the active sidebar filter but ignoring the search query.
    pub filter_count:            usize,
    /// Items matching both the active sidebar filter and the search query.
    pub matched_count:           usize,
    pub search_query:            String,
    pub sort:                    SortMethod,
    pub sort_direction:          SortDirection,
    pub grouped:                 bool,
    pub presentation:            CatalogPresentation,
    pub selected_item:           Option<LibraryItem>,
    pub items:                   Vec<LibraryItem>,
    pub catalog_loading:         bool,
    pub current_page:            usize,
    pub page_size:               usize,
    pub total_pages:             usize,
    /// Whether the publishers section's inline search bar is expanded.
    /// Session-only; never persisted.
    pub publisher_search_open:   bool,
    /// Publishers section search filter text. Session-only; never persisted.
    pub publisher_search_query:  String,
    /// Whether the collections section's inline search bar is expanded.
    /// Session-only; never persisted.
    pub collection_search_open:  bool,
    /// Collections section search filter text. Session-only; never persisted.
    pub collection_search_query: String,
    /// Current width of the detail panel, in pixels.
    pub detail_panel_width:      f32,
}

/// Owns all mutable state for the library view.
pub struct LibraryController {
    /// View model that owns the service and pane state.
    vm:                      LibraryViewModel,
    /// Keeps the `ActivityController` entity alive so the weak reference in
    /// background task closures remains valid for the lifetime of this
    /// controller.
    #[allow(dead_code)]
    activity:                Entity<ActivityController>,
    /// Full catalog — never filtered.
    catalog:                 Vec<LibraryItem>,
    /// Active sidebar filter.
    pub filter:              SidebarFilter,
    /// Text search query.
    pub search_query:        String,
    /// Current sort method.
    pub sort:                SortMethod,
    /// Current sort direction.
    pub sort_direction:      SortDirection,
    /// Whether the catalog is grouped by publisher.
    pub grouped:             bool,
    /// Active catalog presentation mode.
    pub presentation:        CatalogPresentation,
    /// The currently selected item id (for the detail panel).
    pub selection:           Selection,
    /// Smart section counts derived from the full catalog.
    pub section_counts:      SectionCounts,
    /// Publisher list derived from the full catalog (count desc, name asc).
    pub publishers:          Vec<PublisherEntry>,
    /// Collection list loaded from the API product-list endpoint.
    pub collections:         Vec<CollectionEntry>,
    /// True once the first `apply_collections` call has completed for the
    /// current session. Used by the sidebar to show a "?" placeholder for
    /// the collection count instead of a misleading `0` while the initial
    /// fetch is still in flight.
    collections_loaded:      bool,
    /// Backing service for collections; stored so it can be replaced on
    /// sign-in.
    collections_service:     Arc<dyn CollectionsService>,
    /// Set of numeric product IDs belonging to the active collection filter.
    /// Populated by `set_filter` when `SidebarFilter::Collection(_)` is set;
    /// cleared when any other filter is active.
    pub collection_members:  HashSet<u64>,
    /// Queue of `(item_id, cover_url, force_network)` triples pending thumbnail
    /// fetches. `force_network` skips the disk cache and always re-fetches —
    /// set for manual "Load Thumbnail"/"Refresh Thumbnails" actions, which
    /// exist specifically to bypass a stale cached image; left `false` for
    /// automatic background loads.
    thumbnail_queue:         VecDeque<(Arc<str>, Arc<str>, bool)>,
    /// Whether a thumbnail fetch is currently in flight.
    thumbnail_loading:       bool,
    /// Activity id for the aggregated thumbnail loading entry.
    thumbnail_activity_id:   Option<u64>,
    /// Number of thumbnails processed (successes and failures both count) in
    /// the current batch. Reset to `0` whenever a new aggregated activity
    /// item starts. Together with the live queue length this gives a real,
    /// monotonically advancing progress fraction — `processed / (processed
    /// + queue.len())` — instead of an indeterminate placeholder.
    thumbnail_processed:     usize,
    /// True from startup until the first `set_catalog` call completes.
    catalog_loading:         bool,
    /// Incremented each time [`start_load_inner`](Self::start_load_inner)
    /// starts a new load attempt. Background tasks from a superseded load
    /// compare their captured generation against the current value before
    /// writing catalog state, so a load started before a `clear_and_reload`
    /// cannot clobber it after the fact.
    load_generation:         u64,
    /// 1-based current page index.
    pub current_page:        usize,
    /// Number of items per page. One of: 10, 25, 50, 100, 200.
    pub page_size:           usize,
    /// Cached filtered/sorted result of the current catalog, filter, search
    /// query, and sort settings. `None` means stale; recomputed lazily by
    /// [`cached_visible_items`](Self::cached_visible_items).
    items_cache:             Option<Vec<LibraryItem>>,
    /// Whether the publishers section's inline search bar is expanded.
    /// Session-only; never persisted.
    publisher_search_open:   bool,
    /// Publishers section search filter text. Session-only; never persisted.
    publisher_search_query:  String,
    /// Whether the collections section's inline search bar is expanded.
    /// Session-only; never persisted.
    collection_search_open:  bool,
    /// Collections section search filter text. Session-only; never persisted.
    collection_search_query: String,
    /// Current width of the detail panel, in pixels. Session-only; never
    /// persisted to disk.
    detail_panel_width:      f32,
}

impl LibraryController {
    /// Creates a controller and immediately schedules catalog loading on a
    /// background thread.
    ///
    /// The controller starts in the `Loading` pane state with an empty catalog.
    /// When the background fetch completes, [`apply_load_result`] is called
    /// and [`LibraryChanged`] emitted.
    ///
    /// # Panics
    ///
    /// Does not panic; service errors are reflected in [`pane_state`].
    pub fn new(service: Box<dyn LibraryService>,
               collections_service: Box<dyn CollectionsService>,
               activity: Entity<ActivityController>, cx: &mut Context<Self>)
               -> Self {
        let vm = LibraryViewModel::new(service);

        let mut ctrl = Self { vm,
                              activity,
                              catalog: Vec::new(),
                              filter: SidebarFilter::default(),
                              search_query: String::new(),
                              sort: SortMethod::default(),
                              sort_direction: SortDirection::default(),
                              grouped: false,
                              presentation: CatalogPresentation::default(),
                              selection: Selection::default(),
                              section_counts: SectionCounts::default(),
                              publishers: Vec::new(),
                              collections: Vec::new(),
                              collections_loaded: false,
                              collections_service: Arc::from(collections_service),
                              collection_members: HashSet::new(),
                              thumbnail_queue: VecDeque::new(),
                              thumbnail_loading: false,
                              thumbnail_activity_id: None,
                              thumbnail_processed: 0,
                              catalog_loading: true,
                              load_generation: 0,
                              current_page: 1,
                              page_size: crate::data::ui_prefs::UiPrefs::load().page_size()
                                                                               .unwrap_or(25),
                              items_cache: None,
                              publisher_search_open: false,
                              publisher_search_query: String::new(),
                              collection_search_open: false,
                              collection_search_query: String::new(),
                              detail_panel_width:
                                  crate::data::constants::DETAIL_PANEL_DEFAULT_WIDTH };
        ctrl.start_load(cx);
        ctrl
    }

    /// Spawns a background task to load the catalog from cache then optionally
    /// from the live API.
    ///
    /// When `force_reload` is true the auto-load policy is bypassed and a full
    /// live fetch always runs. When false, the fetch is skipped if the
    /// cache is non-empty and was written within the last 7 days.
    ///
    /// Pages are delivered via an mpsc channel so each page triggers a UI
    /// update before the next page arrives.
    fn start_load(&mut self, cx: &mut Context<Self>) {
        self.start_load_inner(cx, false);
    }

    fn start_load_inner(&mut self, cx: &mut Context<Self>, force_reload: bool) {
        self.load_generation += 1;
        let generation = self.load_generation;
        let service_arc = self.vm.service_arc();
        let weak_activity = self.activity.downgrade();
        let storage_root = cache_dir();
        let save_root = storage_root.clone();

        cx.spawn(async move |this, async_cx| {
            // ── Pre-populate from disk cache ──────────────────────────────────
            let col_cache_root = storage_root.clone();
            let cached_collections = async_cx
                .background_executor()
                .spawn(async move { load_collections_cache(&col_cache_root) })
                .await;
            if let Some(entries) = cached_collections {
                this.update(async_cx, |ctrl, cx| {
                    if ctrl.load_generation != generation {
                        return; // superseded by a newer load (e.g. cache-clear reload)
                    }
                    ctrl.apply_collections(entries, cx);
                })
                .ok();
            }

            let cache_root = storage_root.clone();
            let meta_root = cache_root.clone();

            let cached = async_cx
                .background_executor()
                .spawn(async move { load_catalog_cache(&cache_root) })
                .await;
            if let Some(items) = cached.as_ref() {
                this.update(async_cx, |ctrl, cx| {
                    if ctrl.load_generation != generation {
                        return; // superseded by a newer load (e.g. cache-clear reload)
                    }
                    if force_reload {
                        ctrl.catalog.clear();
                    }
                    ctrl.append_catalog_page(items.clone(), cx);
                })
                .ok();
            }

            // ── Auto-load policy ──────────────────────────────────────────────
            // Skip the live fetch when: the cache is non-empty, was written within
            // the last 7 days, and force_reload is false.
            if !force_reload && let Some(items) = cached.as_ref().filter(|items| !items.is_empty()) {
                        let meta = async_cx
                            .background_executor()
                            .spawn(async move { load_cache_metadata(&meta_root) })
                            .await;
                        let is_fresh = meta.as_ref().is_some_and(|m| !m.is_stale());

                        if is_fresh {
                            // Check remote count if the service supports it cheaply. Surface
                            // this as its own activity item — it is usually fast, but on a
                            // slow connection it is otherwise indistinguishable from the app
                            // being stuck doing nothing.
                            let count_activity_id = weak_activity
                                .update(async_cx, |a, cx| {
                                    a.start("Getting count of items\u{2026}", None, cx)
                                })
                                .unwrap_or(0);
                            let svc = service_arc.clone();
                            let remote_count = async_cx
                                .background_executor()
                                .spawn(async move { svc.count_items() })
                                .await;
                            weak_activity
                                .update(async_cx, |a, cx| a.complete(count_activity_id, cx))
                                .ok();
                            let count_matches = match remote_count {
                                Some(Ok(n)) => n == items.len(),
                                Some(Err(_)) => false, // count fetch failed; do live fetch
                                None => true, // count not supported; trust fresh cache
                            };
                            if count_matches {
                                tracing::debug!(
                                    cached = items.len(),
                                    "catalog auto-load: cache is fresh, skipping live fetch"
                                );
                                this.update(async_cx, |ctrl, cx| {
                                    if ctrl.load_generation != generation {
                                        return; // superseded by a newer load
                                    }
                                    ctrl.catalog_loading = false;
                                    cx.emit(LibraryChanged);
                                })
                                .ok();
                                return;
                            }
                        }
            }

            // ── Fetch live catalog from API ───────────────────────────────────
            let activity_id = weak_activity
                .update(async_cx, |a, cx| a.start("Loading library\u{2026}", None, cx))
                .unwrap_or(0);

            // Two channels: one for page batches, one for the API-reported total, if any.
            let (tx, rx) = std::sync::mpsc::channel::<Vec<LibraryItem>>();
            let (total_tx, total_rx) = std::sync::mpsc::channel::<usize>();

            // Run the paginated fetch on the background executor. Each page — and, when
            // the API reports one (via `links.last`), the estimated total — is sent
            // through its channel as it arrives; the result indicates overall success/failure.
            let fetch = async_cx
                .background_executor()
                .spawn(async move {
                    service_arc.list_items_paged(
                        &mut |page| { tx.send(page).ok(); },
                        Some(&mut |total| { total_tx.send(total).ok(); }),
                    )
                });

            // Seed the progress denominator from the local cache count — a close
            // approximation of the remote count in the common case — so the bar starts
            // determinate and moves immediately, rather than blocking page delivery on
            // a `total_rx.recv()` that may never resolve (the API's `links.last` is
            // optional and often absent). If the API does report its own total later,
            // it replaces this estimate the next time progress is recomputed below.
            let mut estimated_total: Option<usize> =
                cached.as_ref().map(|items| items.len()).filter(|&n| n > 0);
            if let Some(total) = estimated_total {
                weak_activity
                    .update(async_cx, |a, cx| a.update_progress(activity_id, 0.0, cx))
                    .ok();
                tracing::debug!(estimated_total = total, "catalog load: seeded total from cache");
            }

            // Accumulate all live SDK pages into a local buffer so a *populated* cache
            // remains visible in the UI throughout the fetch instead of flashing down to
            // a single page (see `catalog-live-merge`). When there is nothing cached to
            // protect — first launch, or right after `clear_and_reload` — there is no
            // flash risk, so each page is appended to the visible catalog (and its
            // thumbnails enqueued) as it arrives instead of waiting for the entire fetch
            // to finish. This is what makes thumbnail loading observable/testable without
            // waiting through a full multi-page fetch.
            let catalog_was_empty = this
                .update(async_cx, |ctrl, _cx| ctrl.catalog.is_empty())
                .unwrap_or(false);

            let mut rx = Some(rx);
            let mut total_rx = Some(total_rx);
            let mut live_items: Vec<LibraryItem> = Vec::new();
            loop {
                let Some(receiver) = rx.take() else { break; };

                // Non-blocking check for an API-reported total before processing the
                // next page, so the bar re-bases onto the real count as soon as it's
                // known without ever blocking page delivery.
                if let Some(total_receiver) = total_rx.take() {
                    match total_receiver.try_recv() {
                        Ok(total) => {
                            estimated_total = Some(total);
                            tracing::debug!(
                                estimated_total = total,
                                "catalog load: API-reported item count"
                            );
                        }
                        Err(std::sync::mpsc::TryRecvError::Empty) => {
                            total_rx = Some(total_receiver);
                        }
                        Err(std::sync::mpsc::TryRecvError::Disconnected) => {}
                    }
                }

                let (msg, returned_rx) = async_cx
                    .background_executor()
                    .spawn(async move {
                        let msg = receiver.recv();
                        (msg, receiver)
                    })
                    .await;

                match msg {
                    Ok(items) => {
                        if catalog_was_empty {
                            let page = items.clone();
                            this.update(async_cx, |ctrl, cx| {
                                if ctrl.load_generation == generation {
                                    ctrl.append_catalog_page(page, cx);
                                }
                            })
                            .ok();
                        }
                        live_items.extend(items);
                        // Update progress after each page if we have an estimate; the
                        // real DriveThruRPG API never reports `links.last`, so in
                        // practice this only fires when a local cache seeded a total
                        // above. Without a total there is no way to compute a real
                        // percentage — fall back to a growing item-count label instead
                        // of leaving the activity item looking frozen (its progress bar
                        // renders indeterminate, see `activity_panel_view`).
                        if let Some(total) = estimated_total.filter(|&t| t > 0) {
                            let progress = (live_items.len() as f32 / total as f32).min(1.0);
                            weak_activity
                                .update(async_cx, |a, cx| a.update_progress(activity_id, progress, cx))
                                .ok();
                        } else {
                            let label =
                                format!("Loading library\u{2026} ({} items)", live_items.len());
                            weak_activity
                                .update(async_cx, |a, cx| a.update_label(activity_id, label, cx))
                                .ok();
                        }
                        rx = Some(returned_rx);
                    }
                    Err(_) => break, // sender dropped — all pages have been sent
                }
            }

            // Wait for the fetch task to complete and surface any error.
            // On success: swap the catalog atomically with the full live dataset — a no-op
            //   diff when `catalog_was_empty` already appended every page incrementally
            //   above, but still required to guarantee the final state is exactly the live
            //   dataset (e.g. if a page arrived out of order or a duplicate id was
            //   deduped differently) — then save to disk and dismiss the loading indicator.
            // On error: leave the cached catalog unchanged in memory.
            match fetch.await {
                Ok(()) => {
                    let is_current = this
                        .update(async_cx, |ctrl, _cx| ctrl.load_generation == generation)
                        .unwrap_or(false);
                    if !is_current {
                        // A newer load (e.g. triggered by a cache clear) has already
                        // taken over; discard this stale result instead of clobbering it.
                        weak_activity.update(async_cx, |a, cx| a.complete(activity_id, cx)).ok();
                        return;
                    }
                    this.update(async_cx, |ctrl, cx| {
                        ctrl.set_catalog(live_items, cx);
                    }).ok();
                    let items_to_save = this
                        .update(async_cx, |ctrl, _cx| ctrl.catalog.clone())
                        .unwrap_or_default();
                    async_cx
                        .background_executor()
                        .spawn(async move {
                            if let Err(e) = save_catalog_cache(&save_root, &items_to_save) {
                                tracing::warn!(error = %e, "failed to save catalog cache");
                            }
                        })
                        .await;
                    weak_activity.update(async_cx, |a, cx| a.complete(activity_id, cx)).ok();
                }
                Err(e) => {
                    // Session errors are expected when starting before auth completes;
                    // treat them as a quiet completion rather than a user-facing alert.
                    // Network and other errors are genuine failures worth surfacing.
                    if e.kind == LibraryServiceErrorKind::Session {
                        tracing::debug!(error = %e, "catalog load skipped: no authenticated session");
                        weak_activity.update(async_cx, |a, cx| a.complete(activity_id, cx)).ok();
                    } else {
                        tracing::error!(error = %e, backtrace = %app_backtrace(), "catalog load failed");
                        let detail = e.panel_detail();
                        weak_activity.update(async_cx, |a, cx| a.error(activity_id, detail, cx)).ok();
                    }
                }
            }
        })
        .detach();
    }

    /// Atomically replaces the catalog with a complete dataset and recomputes
    /// derived state.
    ///
    /// Used after all live SDK pages have been collected so the UI transitions
    /// from cached data to the full live set in one render pass.
    fn set_catalog(&mut self, items: Vec<LibraryItem>, cx: &mut Context<Self>) {
        self.enqueue_thumbnails(&items, cx);
        self.catalog = items;
        self.catalog_loading = false;
        self.section_counts = section_counts(&self.catalog);
        self.publishers = publisher_entries(&self.catalog);
        self.invalidate_cache();
        cx.emit(LibraryChanged);
    }

    /// Appends a page of items received incrementally from the background load
    /// task.
    ///
    /// Used only for the initial disk-cache pre-population, not for live SDK
    /// pages.
    fn append_catalog_page(&mut self, items: Vec<LibraryItem>, cx: &mut Context<Self>) {
        self.enqueue_thumbnails(&items, cx);
        self.catalog.extend(items);
        self.section_counts = section_counts(&self.catalog);
        self.publishers = publisher_entries(&self.catalog);
        self.invalidate_cache();
        cx.emit(LibraryChanged);
    }

    /// Spawns a background task to fetch product lists from the API and apply
    /// them.
    pub fn load_collections(&mut self, cx: &mut Context<Self>) {
        let collections_service = Arc::clone(&self.collections_service);
        let cache_root = cache_dir();
        let weak_activity = self.activity.downgrade();
        let activity_id = self.activity
                              .update(cx, |a, cx| a.start("Loading collections\u{2026}", None, cx));
        tracing::debug!("load_collections: starting collections fetch");
        cx.spawn(async move |this, async_cx| {
            let result = async_cx
                .background_executor()
                .spawn(async move { collections_service.list_collections() })
                .await;
            match result {
                Ok(entries) => {
                    tracing::debug!(
                        count = entries.len(),
                        "load_collections: fetched {} entries",
                        entries.len()
                    );
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
                    weak_activity
                        .update(async_cx, |a, cx| a.complete(activity_id, cx))
                        .ok();
                }
                Err(e) => {
                    // Session errors are expected when starting before auth completes
                    // (see `start_load_inner`'s matching treatment for the catalog
                    // fetch); quietly complete rather than surfacing a user-facing error.
                    if e.kind == crate::services::collections::CollectionsServiceErrorKind::Session
                    {
                        tracing::debug!(error = %e, "collections load skipped: no authenticated session");
                        weak_activity
                            .update(async_cx, |a, cx| a.complete(activity_id, cx))
                            .ok();
                    } else {
                        tracing::warn!(error = %e, "collections load failed");
                        weak_activity
                            .update(async_cx, |a, cx| a.error(activity_id, e.to_string(), cx))
                            .ok();
                    }
                }
            }
        })
        .detach();
    }

    /// Stores the fetched collections and emits a change event.
    fn apply_collections(&mut self, collections: Vec<CollectionEntry>, cx: &mut Context<Self>) {
        self.collections = collections;
        self.collections_loaded = true;
        // Refresh collection_members if the current filter is a Collection filter,
        // in case the filter was set before the collections loaded.
        if let SidebarFilter::Collection(id, _) = &self.filter {
            let id = *id;
            self.collection_members = self.collections
                                          .iter()
                                          .find(|c| c.id == id)
                                          .map(|c| c.member_ids.iter().copied().collect())
                                          .unwrap_or_default();
        }
        cx.emit(LibraryChanged);
    }

    /// Returns the current pane state from the service layer.
    pub fn pane_state(&self) -> &LibraryPaneState {
        self.vm.pane_state()
    }

    /// Replaces the backing services and triggers a fresh catalog and
    /// collections load.
    ///
    /// Clears the activity panel so stale error messages from the previous
    /// (unauthenticated) service do not persist after sign-in.
    pub fn replace_service(&mut self, service: Box<dyn LibraryService>,
                           collections_service: Box<dyn CollectionsService>,
                           cx: &mut Context<Self>) {
        tracing::debug!("replace_service: installing authenticated services");
        self.vm.replace_service(service);
        self.collections_service = Arc::from(collections_service);
        self.catalog.clear();
        self.section_counts = section_counts(&self.catalog);
        self.publishers = publisher_entries(&self.catalog);
        self.collections.clear();
        self.collections_loaded = false;
        self.collection_members.clear();
        self.selection = Selection::default();
        self.invalidate_cache();
        self.activity.update(cx, |a, cx| a.clear(cx));
        cx.emit(LibraryChanged);
        // Load collections concurrently with the catalog — don't wait for the
        // full catalog fetch to complete before showing collection names.
        self.load_collections(cx);
        self.start_load(cx);
    }

    /// Starts a background task to create a new collection with the given name.
    ///
    /// Tracks progress in the activity panel. On success the new entry is
    /// appended and [`LibraryChanged`] is emitted. On failure
    /// [`CollectionCreateFailed`] is emitted so the window can push an
    /// error notification.
    pub fn create_collection(&mut self, name: String, cx: &mut Context<Self>) {
        let label = format!("Creating collection '{name}'...");
        let activity_id = self.activity.update(cx, |a, cx| a.start(&label, None, cx));

        let collections_service = Arc::clone(&self.collections_service);
        cx.spawn(async move |this, async_cx| {
              let result =
                  async_cx.background_executor()
                          .spawn(async move { collections_service.create_collection(&name) })
                          .await;

              match result {
                  Ok(entry) => {
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.collections.push(entry);
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
                              cx.emit(CollectionCreateFailed { message: e.message.clone(), });
                          })
                          .ok();
                  }
              }
          })
          .detach();
    }

    /// Forces a full live catalog fetch, bypassing the auto-load policy.
    ///
    /// Used by the "Catalog > Reload" menu action.
    pub fn reload_catalog(&mut self, cx: &mut Context<Self>) {
        self.catalog_loading = true;
        cx.emit(LibraryChanged);
        self.start_load_inner(cx, true);
    }

    /// Drops the in-memory catalog and collections, then forces a full live
    /// fetch.
    ///
    /// Used after the on-disk app cache has been cleared, so stale content
    /// disappears from the UI immediately instead of lingering until an
    /// unrelated reload repopulates it from what is now a missing cache file.
    ///
    /// Any catalog load already in flight is superseded: `start_load_inner`
    /// bumps `load_generation`, so the older task's completion is discarded
    /// rather than clobbering the fresh reload triggered here.
    /// Queued-but-not-yet-started thumbnail fetches are dropped too; a
    /// fetch already in flight still completes and populates `CoverCache`,
    /// which is harmless since it only caches an image for a URL that may
    /// no longer be visible.
    pub fn clear_and_reload(&mut self, cx: &mut Context<Self>) {
        self.catalog.clear();
        self.collections.clear();
        self.collection_members.clear();
        self.section_counts = SectionCounts::default();
        self.publishers.clear();
        self.invalidate_cache();
        for (id, _url, _force) in self.thumbnail_queue.drain(..) {
            cx.global_mut::<CoverCache>().in_flight.remove(&id);
        }
        // If nothing is currently in flight, no pending fetch completion will ever run
        // `drain_thumbnail_queue`'s empty-queue completion branch — without this, the
        // aggregated activity item would be left showing "in progress" indefinitely.
        if !self.thumbnail_loading
           && let Some(id) = self.thumbnail_activity_id.take()
        {
            self.thumbnail_processed = 0;
            self.activity.update(cx, |a, cx| a.complete(id, cx));
        }
        self.reload_catalog(cx);
    }

    /// Emits `LibraryChanged` to trigger a UI re-render without modifying
    /// state.
    ///
    /// Used by sidebar section headers to force a re-render after persisting UI
    /// prefs.
    pub fn notify_ui_change(&mut self, cx: &mut Context<Self>) {
        cx.emit(LibraryChanged);
    }

    /// Deletes the collection with the given id via the service, then removes
    /// it locally.
    ///
    /// Logs failures to the activity panel and leaves the collection in place
    /// on error.
    pub fn delete_collection(&mut self, id: u64, cx: &mut Context<Self>) {
        let label = "Deleting collection\u{2026}".to_string();
        let activity_id = self.activity.update(cx, |a, cx| a.start(&label, None, cx));
        let collections_service = Arc::clone(&self.collections_service);
        cx.spawn(async move |this, async_cx| {
              let result = async_cx.background_executor()
                                   .spawn(async move { collections_service.delete_collection(id) })
                                   .await;
              match result {
                  Ok(()) => {
                      this.update(async_cx, |ctrl, cx| {
                              ctrl.collections.retain(|c| c.id != id);
                              if let SidebarFilter::Collection(cid, _) = &ctrl.filter
                                 && *cid == id
                              {
                                  ctrl.filter = SidebarFilter::default();
                                  ctrl.collection_members.clear();
                              }
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

    /// Reloads catalog from the service and resets selection.
    pub fn reload(&mut self, cx: &mut Context<Self>) {
        self.vm.refresh();
        self.catalog = self.vm.items().to_vec();
        self.section_counts = section_counts(&self.catalog);
        self.publishers = publisher_entries(&self.catalog);
        self.selection = Selection::default();
        self.invalidate_cache();
        cx.emit(LibraryChanged);
    }

    // ── Thumbnail loading ──────────────────────────────────────────────────────

    /// Enqueues thumbnail fetches for items that have a `cover_url` not yet
    /// cached or in flight.  Must be called before items are added to `catalog`
    /// so the in-flight marker is set before any render pass can check it.
    fn enqueue_thumbnails(&mut self, items: &[LibraryItem], cx: &mut Context<Self>) {
        let to_enqueue: Vec<(Arc<str>, Arc<str>)> = {
            let cache = cx.global::<CoverCache>();
            items.iter()
                 .filter_map(|item| {
                     let url = item.cover_url.as_ref()?;
                     let id = Arc::clone(&item.id);
                     if cache.get(&id).is_none() && !cache.is_in_flight(&id) {
                         Some((id, Arc::clone(url)))
                     }
                     else {
                         None
                     }
                 })
                 .collect()
        };
        for (id, url) in &to_enqueue {
            cx.global_mut::<CoverCache>().mark_in_flight(Arc::clone(id));
            self.thumbnail_queue
                .push_back((Arc::clone(id), Arc::clone(url), false));
        }
        self.drain_thumbnail_queue(cx);
    }

    /// Starts a thumbnail fetch for the next queued URL if none is in flight.
    fn drain_thumbnail_queue(&mut self, cx: &mut Context<Self>) {
        if self.thumbnail_loading || self.thumbnail_queue.is_empty() {
            return;
        }
        let Some((item_id, url, force_network)) = self.thumbnail_queue.pop_front()
        else {
            return;
        };

        self.thumbnail_loading = true;

        let activity_id = if let Some(id) = self.thumbnail_activity_id {
            id
        }
        else {
            let id = self.activity
                         .update(cx, |a, cx| a.start("Loading thumbnails\u{2026}", None, cx));
            self.thumbnail_activity_id = Some(id);
            self.thumbnail_processed = 0;
            id
        };

        let weak_activity = self.activity.downgrade();
        let url_str = url.to_string();

        cx.spawn(async move |this, async_cx| {
              // gpui's executors are not a Tokio runtime, and `dtrpg-ui` does not depend on
              // `tokio` directly, so the async `reqwest::get(...).await` used here previously
              // had no reactor to run on and always failed. `reqwest::blocking` manages its
              // own internal runtime per call and works from a plain OS thread, matching the
              // pattern the SDK gateway already uses (`tokio::runtime::Runtime::block_on`) to
              // run network calls from these same background-executor threads.
              //
              // Disk-cache-first: the in-memory `CoverCache` is always empty at startup, so
              // without a persistent disk cache every launch would re-download every cover
              // from the network. Check disk before hitting the network, and persist a fresh
              // network fetch to disk so the next launch is a cache hit.
              let fetch_url = url_str.clone();
              let disk_key = Arc::clone(&item_id);
              let result: Result<Vec<u8>, String> =
                  async_cx.background_executor()
                          .spawn(async move {
                              let covers_root = covers_dir();
                              if !force_network
                                 && let Some(bytes) = load_cached_cover(&covers_root, &disk_key)
                              {
                                  return Ok(bytes);
                              }
                              let resp =
                                  reqwest::blocking::get(&fetch_url).map_err(|e| e.to_string())?;
                              let bytes = resp.bytes().map_err(|e| e.to_string())?.to_vec();
                              save_cached_cover(&covers_root, &disk_key, &bytes);
                              Ok(bytes)
                          })
                          .await;

              match result {
                  Ok(bytes) => {
                      this.update(async_cx, |ctrl, cx| {
                              cx.global_mut::<CoverCache>()
                                .insert(Arc::clone(&item_id), bytes);
                              if let Some(item) = ctrl.catalog.iter_mut().find(|i| i.id == item_id)
                              {
                                  item.thumbnail_last_attempted =
                                      Some(std::time::SystemTime::now());
                              }
                              ctrl.invalidate_cache();
                              ctrl.thumbnail_loading = false;
                              cx.emit(LibraryChanged);
                              ctrl.drain_thumbnail_queue(cx);
                          })
                          .ok();
                  }
                  Err(e) => {
                      tracing::warn!(url = %url_str, error = %e, "thumbnail fetch failed");
                      this.update(async_cx, |ctrl, cx| {
                              cx.global_mut::<CoverCache>().in_flight.remove(&item_id);
                              if let Some(item) = ctrl.catalog.iter_mut().find(|i| i.id == item_id)
                              {
                                  item.thumbnail_last_attempted =
                                      Some(std::time::SystemTime::now());
                              }
                              ctrl.invalidate_cache();
                              ctrl.thumbnail_loading = false;
                              ctrl.drain_thumbnail_queue(cx);
                          })
                          .ok();
                  }
              }

              // Count this attempt (success or failure) toward the batch total so the
              // progress bar reflects real throughput instead of sitting indeterminate.
              let (processed, remaining) = this.update(async_cx, |ctrl, _cx| {
                                                   ctrl.thumbnail_processed += 1;
                                                   (ctrl.thumbnail_processed,
                                                    ctrl.thumbnail_queue.len())
                                               })
                                               .unwrap_or((0, 0));

              if remaining == 0 {
                  weak_activity.update(async_cx, |a, cx| a.complete(activity_id, cx))
                               .ok();
                  this.update(async_cx, |ctrl, _cx| {
                          ctrl.thumbnail_activity_id = None;
                          ctrl.thumbnail_processed = 0;
                      })
                      .ok();
              }
              else {
                  let label = format!("Loading thumbnails\u{2026} ({remaining} remaining)");
                  let total = (processed + remaining) as f32;
                  let progress = if total > 0.0 {
                      processed as f32 / total
                  }
                  else {
                      0.0
                  };
                  weak_activity.update(async_cx, |a, cx| {
                                   a.update_label(activity_id, label, cx);
                                   a.update_progress(activity_id, progress, cx);
                               })
                               .ok();
              }
          })
          .detach();
    }

    /// Enqueues a single cover URL at the front of the thumbnail queue and
    /// starts the drain loop.  Used by the per-item "Load Thumbnail"
    /// context menu action.
    ///
    /// Forces a network re-fetch bypassing both the in-memory and on-disk
    /// caches — this action exists specifically to retry or refresh a
    /// cover, not to redundantly reload whatever is already cached.
    pub fn load_thumbnail(&mut self, cover_url: Arc<str>, cx: &mut Context<Self>) {
        let item_id = self.catalog
                          .iter()
                          .find(|i| i.cover_url.as_deref() == Some(&*cover_url))
                          .map(|i| Arc::clone(&i.id));

        if let Some(id) = item_id {
            self.thumbnail_queue.retain(|(i, _, _)| i != &id);
            cx.global_mut::<CoverCache>()
              .mark_in_flight(Arc::clone(&id));
            self.thumbnail_queue.push_front((id, cover_url, true));
            self.drain_thumbnail_queue(cx);
        }
    }

    /// Re-fetches thumbnails for every catalog item with a `cover_url`,
    /// overwriting any cached image (in-memory, and on disk). Used by the
    /// "Refresh Thumbnails" catalog menu action.
    ///
    /// Unlike [`enqueue_thumbnails`](Self::enqueue_thumbnails), this does not
    /// skip items already present in the cache — every eligible item is
    /// re-queued with a forced network re-fetch.
    pub fn refresh_all_thumbnails(&mut self, cx: &mut Context<Self>) {
        let to_enqueue: Vec<(Arc<str>, Arc<str>, bool)> =
            self.catalog
                .iter()
                .filter_map(|item| {
                    let url = item.cover_url.as_ref()?;
                    Some((Arc::clone(&item.id), Arc::clone(url), true))
                })
                .collect();

        if to_enqueue.is_empty() {
            return;
        }

        self.thumbnail_queue.clear();
        // This forces a fresh full re-fetch batch — reset the processed counter so the
        // progress bar denominator isn't skewed by whatever fraction of a prior batch
        // had already completed before this action was invoked.
        self.thumbnail_processed = 0;
        let cache = cx.global_mut::<CoverCache>();
        for (id, _, _) in &to_enqueue {
            cache.mark_in_flight(Arc::clone(id));
        }
        self.thumbnail_queue.extend(to_enqueue);
        self.drain_thumbnail_queue(cx);
    }

    // ── Snapshot ──────────────────────────────────────────────────────────────

    /// Returns all data needed by the root view for one render pass.
    pub fn snapshot(&self) -> LibrarySnapshot {
        let filter_count =
            self.catalog
                .iter()
                .filter(|i| item_matches_filter(i, &self.filter, &self.collection_members))
                .count();
        let all_items = self.visible_items();
        let matched_count = all_items.len();
        let total_pages = matched_count.div_ceil(self.page_size).max(1);
        let page_start = (self.current_page - 1) * self.page_size;
        let items: Vec<LibraryItem> = all_items.into_iter()
                                               .skip(page_start)
                                               .take(self.page_size)
                                               .collect();
        let selected_item = self.selected_item().cloned();
        // Build a set of all IDs that can match collection member_ids.
        // Include both order_product_id and product_id since the product list items
        // API returns productId (not orderProductId) in its response.
        let catalog_ids: HashSet<u64> = self.catalog
                                            .iter()
                                            .flat_map(|i| {
                                                let mut ids = Vec::with_capacity(2);
                                                if i.order_product_id > 0 {
                                                    ids.push(i.order_product_id);
                                                }
                                                if i.product_id > 0 {
                                                    ids.push(i.product_id);
                                                }
                                                ids
                                            })
                                            .collect();
        LibrarySnapshot { filter: self.filter.clone(),
                          counts: self.section_counts,
                          publishers: self.publishers.clone(),
                          collections: self.collections.clone(),
                          collections_loaded: self.collections_loaded,
                          catalog_ids,
                          total_count: self.section_counts.all,
                          total_mb: self.total_size_mb(),
                          filter_count,
                          matched_count,
                          search_query: self.search_query.clone(),
                          sort: self.sort,
                          sort_direction: self.sort_direction,
                          grouped: self.grouped,
                          presentation: self.presentation,
                          selected_item,
                          items,
                          catalog_loading: self.catalog_loading,
                          current_page: self.current_page,
                          page_size: self.page_size,
                          total_pages,
                          publisher_search_open: self.publisher_search_open,
                          publisher_search_query: self.publisher_search_query.clone(),
                          collection_search_open: self.collection_search_open,
                          collection_search_query: self.collection_search_query.clone(),
                          detail_panel_width: self.detail_panel_width }
    }

    /// Returns true while the initial catalog fetch is still in flight and no
    /// items have arrived.
    pub fn is_loading(&self) -> bool {
        self.catalog_loading && self.catalog.is_empty()
    }

    // ── Pagination ────────────────────────────────────────────────────────────

    /// Total number of pages for the current filtered result set.
    ///
    /// Always returns at least 1.
    pub fn total_pages(&self) -> usize {
        let count = self.visible_items().len();
        count.div_ceil(self.page_size).max(1)
    }

    /// Navigates to the given 1-based page, clamped to the valid range.
    ///
    /// Emits [`LibraryChanged`].
    pub fn set_page(&mut self, page: usize, cx: &mut Context<Self>) {
        let total = self.total_pages();
        self.current_page = page.clamp(1, total);
        cx.emit(LibraryChanged);
    }

    /// Sets the number of items shown per page.
    ///
    /// Accepted values: `[10, 25, 50, 100, 200]`. Ignored for other values.
    /// Resets `current_page` to 1 and emits [`LibraryChanged`].
    pub fn set_page_size(&mut self, size: usize, cx: &mut Context<Self>) {
        const VALID: [usize; 5] = [10, 25, 50, 100, 200];
        if !VALID.contains(&size) {
            return;
        }
        self.page_size = size;
        self.current_page = 1;
        crate::data::ui_prefs::UiPrefs::load().save_page_size(size);
        cx.emit(LibraryChanged);
    }

    // ── Filtered result set ───────────────────────────────────────────────────

    /// Recomputes the filtered, sorted item cache from the current catalog,
    /// filter, search query, and sort settings.
    ///
    /// Called eagerly at every mutation site that changes any of those inputs
    /// so render-path accessors never re-scan the catalog.
    fn invalidate_cache(&mut self) {
        let mut items: Vec<LibraryItem> =
            self.catalog
                .iter()
                .filter(|i| {
                    item_matches_filter(i, &self.filter, &self.collection_members)
                    && item_matches_query(i, &self.search_query)
                })
                .cloned()
                .collect();
        sort_items(&mut items, self.sort, self.sort_direction);
        self.items_cache = Some(items);
    }

    /// Returns the filtered, sorted result set for the current state.
    #[must_use]
    pub fn visible_items(&self) -> Vec<LibraryItem> {
        self.items_cache.clone().unwrap_or_default()
    }

    /// Returns the number of items in the filtered, sorted result set.
    ///
    /// Cheaper than cloning the full `Vec` when only the count is needed.
    #[must_use]
    /// Returns the number of items on the current page.
    pub fn visible_items_count(&self) -> usize {
        let all = self.visible_items();
        let page_start = (self.current_page - 1) * self.page_size;
        all.len().saturating_sub(page_start).min(self.page_size)
    }

    /// Returns a slice within the current page window.
    ///
    /// `range` is 0-based relative to the start of the current page.
    /// Used by `uniform_list` render closures.
    #[must_use]
    pub fn visible_items_slice(&self, range: std::ops::Range<usize>) -> Vec<LibraryItem> {
        let page_start = (self.current_page - 1) * self.page_size;
        let items = self.visible_items();
        let abs_start = page_start + range.start;
        let abs_end = page_start + range.end;
        items.get(abs_start..abs_end)
             .map(|s| s.to_vec())
             .unwrap_or_default()
    }

    /// Returns all items on the current page.
    #[must_use]
    pub fn visible_page_items(&self) -> Vec<LibraryItem> {
        let page_start = (self.current_page - 1) * self.page_size;
        let items = self.visible_items();
        items.into_iter()
             .skip(page_start)
             .take(self.page_size)
             .collect()
    }

    // ── Sidebar filter mutations ──────────────────────────────────────────────

    /// Sets the active sidebar filter.
    ///
    /// When the new filter is `Collection(id, _)`, `collection_members` is
    /// populated from the matching entry's `member_ids`. For all other
    /// filters, `collection_members` is cleared.
    pub fn set_filter(&mut self, filter: SidebarFilter, cx: &mut Context<Self>) {
        if let SidebarFilter::Collection(id, _) = &filter {
            self.collection_members = self.collections
                                          .iter()
                                          .find(|c| c.id == *id)
                                          .map(|c| c.member_ids.iter().copied().collect())
                                          .unwrap_or_default();
        }
        else {
            self.collection_members.clear();
        }
        self.filter = filter;
        self.current_page = 1;
        self.selection = Selection::None;
        self.invalidate_cache();
        cx.emit(LibraryChanged);
    }

    // ── Search mutations ──────────────────────────────────────────────────────

    /// Updates the text search query.
    pub fn set_search_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.search_query = query;
        self.current_page = 1;
        self.invalidate_cache();
        cx.emit(LibraryChanged);
    }

    /// Clears the text search query.
    pub fn clear_search_query(&mut self, cx: &mut Context<Self>) {
        self.search_query.clear();
        self.invalidate_cache();
        cx.emit(LibraryChanged);
    }

    // ── Sidebar section search mutations ──────────────────────────────────────
    //
    // These filter the publishers/collections sidebar lists only. They never
    // touch the catalog cache and are intentionally session-only: closing a
    // section's search bar clears its query so reopening it always starts fresh.

    /// Toggles the publishers section's inline search bar, clearing its query
    /// on close.
    pub fn toggle_publisher_search(&mut self, cx: &mut Context<Self>) {
        self.publisher_search_open = !self.publisher_search_open;
        if !self.publisher_search_open {
            self.publisher_search_query.clear();
        }
        cx.emit(LibraryChanged);
    }

    /// Updates the publishers section search filter text.
    pub fn set_publisher_search_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.publisher_search_query = query;
        cx.emit(LibraryChanged);
    }

    /// Toggles the collections section's inline search bar, clearing its query
    /// on close.
    pub fn toggle_collection_search(&mut self, cx: &mut Context<Self>) {
        self.collection_search_open = !self.collection_search_open;
        if !self.collection_search_open {
            self.collection_search_query.clear();
        }
        cx.emit(LibraryChanged);
    }

    /// Updates the collections section search filter text.
    pub fn set_collection_search_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.collection_search_query = query;
        cx.emit(LibraryChanged);
    }

    // ── Sort mutations ────────────────────────────────────────────────────────

    /// Sets the sort method.
    pub fn set_sort(&mut self, sort: SortMethod, cx: &mut Context<Self>) {
        self.sort = sort;
        self.invalidate_cache();
        cx.emit(LibraryChanged);
    }

    /// Sets the sort direction.
    pub fn set_sort_direction(&mut self, direction: SortDirection, cx: &mut Context<Self>) {
        self.sort_direction = direction;
        self.invalidate_cache();
        cx.emit(LibraryChanged);
    }

    // ── Grouping / presentation mutations ────────────────────────────────────

    /// Toggles publisher grouping on or off.
    pub fn set_grouped(&mut self, grouped: bool, cx: &mut Context<Self>) {
        self.grouped = grouped;
        cx.emit(LibraryChanged);
    }

    /// Switches the catalog presentation mode.
    pub fn set_presentation(&mut self, mode: CatalogPresentation, cx: &mut Context<Self>) {
        self.presentation = mode;
        cx.emit(LibraryChanged);
    }

    // ── Selection mutations ───────────────────────────────────────────────────

    /// Opens the single-click item popover for `id` (see `main-window-tabs`).
    ///
    /// Distinct from opening an expanded detail tab, which is a separate,
    /// double-click action handled by `TabsController::open_detail_tab`.
    pub fn select_item(&mut self, id: Arc<str>, cx: &mut Context<Self>) {
        self.selection = Selection::Item(id);
        cx.emit(LibraryChanged);
    }

    /// Closes the item popover, if one is open.
    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selection = Selection::None;
        cx.emit(LibraryChanged);
    }

    /// Looks up a catalog item by id, independent of the current popover
    /// selection.
    ///
    /// Used to resolve the full item for an open expanded detail tab, which
    /// tracks only the item id, not a full `LibraryItem` snapshot.
    #[must_use]
    pub fn item_by_id(&self, id: &str) -> Option<&LibraryItem> {
        self.catalog.iter().find(|i| i.id.as_ref() == id)
    }

    // ── Detail panel width ────────────────────────────────────────────────────

    /// Returns the current detail panel width, in pixels.
    #[must_use]
    pub fn detail_panel_width(&self) -> f32 {
        self.detail_panel_width
    }

    /// Sets the detail panel width, clamped to
    /// `[DETAIL_PANEL_MIN_WIDTH, DETAIL_PANEL_MAX_WIDTH]`.
    ///
    /// Emits [`LibraryChanged`].
    pub fn set_detail_panel_width(&mut self, width: f32, cx: &mut Context<Self>) {
        self.detail_panel_width = width.clamp(crate::data::constants::DETAIL_PANEL_MIN_WIDTH,
                                              crate::data::constants::DETAIL_PANEL_MAX_WIDTH);
        cx.emit(LibraryChanged);
    }

    // ── Download toggle ───────────────────────────────────────────────────────

    /// Toggles the download status of the item with the given id.
    pub fn toggle_download(&mut self, id: &str, cx: &mut Context<Self>) {
        use crate::data::enums::ItemStatus;
        if let Some(item) = self.catalog.iter_mut().find(|i| i.id.as_ref() == id) {
            item.status = match item.status {
                ItemStatus::Downloaded => ItemStatus::Cloud,
                ItemStatus::Cloud => ItemStatus::Downloaded,
            };
            self.section_counts = section_counts(&self.catalog);
            self.invalidate_cache();
        }
        cx.emit(LibraryChanged);
    }

    // ── Theme / density mutations (dispatched via callbacks) ──────────────────

    /// Applies a new theme key (updates the GPUI global).
    ///
    /// Also re-syncs `gpui_component::Theme`'s table colors (see
    /// [`crate::data::theme::apply_table_colors`]) so the catalog `DataTable`
    /// tracks the newly selected Libri palette instead of staying on whichever
    /// palette was active at startup.
    pub fn set_theme(&self, key: ThemeKey, cx: &mut Context<Self>) {
        let current = cx.global::<LibriTheme>();
        let new_theme = LibriTheme::new(key, current.density);
        let colors = new_theme.colors.clone();
        cx.set_global(new_theme);
        cx.update_global::<gpui_component::Theme, _>(|theme, _cx| {
              apply_table_colors(theme, &colors);
          });
        cx.notify();
    }

    /// Applies a new density (updates the GPUI global).
    pub fn set_density(&self, density: Density, cx: &mut Context<Self>) {
        let current = cx.global::<LibriTheme>();
        let new_theme = LibriTheme::new(current.key, density);
        cx.set_global(new_theme);
        cx.notify();
    }

    // ── Helper accessors ──────────────────────────────────────────────────────

    /// Returns the selected `LibraryItem`, if any.
    #[must_use]
    pub fn selected_item(&self) -> Option<&LibraryItem> {
        match &self.selection {
            Selection::Item(id) => self.catalog.iter().find(|i| &i.id == id),
            Selection::None => None,
        }
    }

    /// Total file size of all items in the catalog, in MB.
    #[must_use]
    pub fn total_size_mb(&self) -> f64 {
        self.catalog.iter().map(|i| i.size_mb).sum()
    }
}

/// Captures a backtrace and returns only the frames from app crates
/// (`dtrpg_*`).
///
/// Each retained frame is one symbol line followed by its `at file:line:col`
/// line. Returns a hint string when `RUST_BACKTRACE` is not set or no app
/// frames are found.
fn app_backtrace() -> String {
    let bt = std::backtrace::Backtrace::capture();
    if bt.status() != std::backtrace::BacktraceStatus::Captured {
        return "<set RUST_BACKTRACE=1 to capture backtraces>".to_string();
    }
    let full = format!("{bt}");
    let mut out: Vec<&str> = Vec::new();
    let mut take_location = false;
    for line in full.lines() {
        if line.trim_start().starts_with("at ") {
            if take_location {
                out.push(line);
                take_location = false;
            }
        }
        else if line.contains("dtrpg_") {
            out.push(line);
            take_location = true;
        }
        else {
            take_location = false;
        }
    }
    if out.is_empty() {
        "<no app frames found in backtrace>".to_string()
    }
    else {
        out.join("\n")
    }
}

// //! Library UI state and interaction controller.

// use crate::app::shell::{AppCommand, AppShell, AppShellState,
// SessionPresentationState}; use crate::services::LibraryItem;
// use crate::services::sdk::RustSdkLibraryService;
// use crate::view_models::library::{LibraryPaneState, LibraryViewModel};

// use crate::ui::library::model::library_data::{
//     FilterScope, LibraryViewMode, MatchPresentation, SortMethod, TreeNode,
// filter_presets,     grouped_items, item_matches, mode_is_grid, mode_label,
// next_sort, root_matches, sort_label,     sorted_flat_items,
// };

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum Selection {
//     Publisher(String),
//     ProductType(String),
//     Item(u64),
// }

// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// pub enum SortPopup {
//     Flat,
//     Outer,
//     Inner,
// }

// /// UI state for the compact DriveThruRPG account menu.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct AccountMenuState {
//     /// User-facing account label shown in the account button/menu.
//     pub display_name: String,
//     /// Human-readable connection or token status.
//     pub connection_status: String,
//     /// Whether an access token is currently configured.
//     pub token_configured: bool,
//     /// Whether the compact account menu is visible.
//     pub menu_open: bool,
// }

// impl AccountMenuState {
//     fn signed_out() -> Self {
//         Self {
//             display_name: "DriveThruRPG account".to_string(),
//             connection_status: "Access token required".to_string(),
//             token_configured: std::env::var("DTRPG_ACCESS_TOKEN").is_ok(),
//             menu_open: false,
//         }
//     }
// }

// /// UI state for low-profile library sync/update reporting.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct SyncStatus {
//     /// Whether a sync or refresh operation is currently active.
//     pub active: bool,
//     /// Human-readable progress summary.
//     pub progress_label: String,
//     /// Human-readable network latency summary.
//     pub latency_label: String,
//     /// Human-readable last-update summary.
//     pub last_update_label: String,
// }

// impl SyncStatus {
//     fn idle() -> Self {
//         Self {
//             active: false,
//             progress_label: "Idle".to_string(),
//             latency_label: "Latency unavailable".to_string(),
//             last_update_label: "Not synced this session".to_string(),
//         }
//     }
// }

// pub struct LibraryController {
//     pub shell: AppShell,
//     pub view_mode: LibraryViewMode,
//     pub filter_scope: FilterScope,
//     pub match_presentation: MatchPresentation,
//     pub flat_sort: SortMethod,
//     pub outer_sort: SortMethod,
//     pub inner_sort: SortMethod,
//     pub filter_query: String,
//     pub search_editing: bool,
//     pub controls_disclosed: bool,
//     pub open_sort_popup: Option<SortPopup>,
//     pub selection: Option<Selection>,
//     pub account: AccountMenuState,
//     pub sync_status: SyncStatus,
// }

// impl LibraryController {
//     pub fn new() -> Self {
//         let service = RustSdkLibraryService::from_environment();
//         let library = LibraryViewModel::new(Box::new(service));

//         let mut shell = AppShell::new(
//             AppShellState {
//                 session: SessionPresentationState::Restoring,
//                 library: LibraryPaneState::Loading,
//                 selected_item_id: None,
//                 status_message: "Loading your library…".to_string(),
//             },
//             library,
//         );

//         shell.dispatch(AppCommand::LoadLibrary);

//         let selection = shell.first_item_id().map(Selection::Item);
//         if let Some(Selection::Item(first)) = selection {
//             shell.dispatch(AppCommand::SelectLibraryItem(first));
//         }

//         Self {
//             shell,
//             view_mode: LibraryViewMode::TreeByPublisher,
//             filter_scope: FilterScope::ChildOnly,
//             match_presentation: MatchPresentation::HideNonMatching,
//             flat_sort: SortMethod::AtoZ,
//             outer_sort: SortMethod::AtoZ,
//             inner_sort: SortMethod::AtoZ,
//             filter_query: String::new(),
//             search_editing: false,
//             open_sort_popup: None,
//             selection,
//             controls_disclosed: true,
//             account: AccountMenuState::signed_out(),
//             sync_status: SyncStatus::idle(),
//         }
//     }

//     pub fn cycle_view_mode(&mut self) {
//         self.view_mode = match self.view_mode {
//             LibraryViewMode::FlatList => LibraryViewMode::TreeByPublisher,
//             LibraryViewMode::TreeByPublisher =>
// LibraryViewMode::TreeByProductType,             
// LibraryViewMode::TreeByProductType => LibraryViewMode::GridByPublisher,
//             LibraryViewMode::GridByPublisher =>
// LibraryViewMode::GridByProductType,             
// LibraryViewMode::GridByProductType => LibraryViewMode::FlatList,         };
//         self.selection = None;
//         self.shell.dispatch(AppCommand::ClearSelection);
//     }

//     pub fn set_view_mode(&mut self, mode: LibraryViewMode) {
//         if self.view_mode != mode {
//             self.view_mode = mode;
//             self.selection = None;
//             self.shell.dispatch(AppCommand::ClearSelection);
//         }
//     }

//     pub fn cycle_filter_scope(&mut self) {
//         self.filter_scope = match self.filter_scope {
//             FilterScope::ChildOnly => FilterScope::RootAndChild,
//             FilterScope::RootAndChild => FilterScope::RootOnly,
//             FilterScope::RootOnly => FilterScope::ChildOnly,
//         };
//     }

//     pub fn set_filter_scope(&mut self, scope: FilterScope) {
//         self.filter_scope = scope;
//     }

//     pub fn set_match_presentation(&mut self, mode: MatchPresentation) {
//         self.match_presentation = mode;
//     }

//     pub fn toggle_match_presentation(&mut self) {
//         self.match_presentation = match self.match_presentation {
//             MatchPresentation::HideNonMatching =>
// MatchPresentation::HighlightMatching,             
// MatchPresentation::HighlightMatching => MatchPresentation::HideNonMatching,
//         };
//     }

//     pub fn cycle_flat_sort(&mut self) {
//         self.flat_sort = next_sort(self.flat_sort);
//     }

//     pub fn set_flat_sort(&mut self, sort: SortMethod) {
//         self.flat_sort = sort;
//         self.open_sort_popup = None;
//     }

//     pub fn cycle_outer_sort(&mut self) {
//         self.outer_sort = next_sort(self.outer_sort);
//     }

//     pub fn set_outer_sort(&mut self, sort: SortMethod) {
//         self.outer_sort = sort;
//         self.open_sort_popup = None;
//     }

//     pub fn cycle_inner_sort(&mut self) {
//         self.inner_sort = next_sort(self.inner_sort);
//     }

//     pub fn set_inner_sort(&mut self, sort: SortMethod) {
//         self.inner_sort = sort;
//         self.open_sort_popup = None;
//     }

//     pub fn toggle_sort_popup(&mut self, popup: SortPopup) {
//         self.open_sort_popup = match self.open_sort_popup {
//             Some(current) if current == popup => None,
//             _ => Some(popup),
//         };
//     }

//     pub fn close_sort_popup(&mut self) {
//         self.open_sort_popup = None;
//     }

//     pub fn toggle_controls_disclosure(&mut self) {
//         self.controls_disclosed = !self.controls_disclosed;
//     }

//     pub fn toggle_account_menu(&mut self) {
//         self.account.menu_open = !self.account.menu_open;
//     }

//     pub fn mark_token_set_action(&mut self) {
//         self.account.token_configured = true;
//         self.account.connection_status = "Access token action
// selected".to_string();         self.account.menu_open = false;
//     }

//     pub fn mark_token_reset_action(&mut self) {
//         self.account.token_configured = false;
//         self.account.connection_status = "Access token reset
// requested".to_string();         self.account.menu_open = false;
//     }

//     pub fn open_settings_action(&mut self) {
//         self.account.connection_status = "Settings action
// selected".to_string();         self.account.menu_open = false;
//     }

//     pub fn cycle_filter_query(&mut self) {
//         let presets = filter_presets();
//         let current = presets
//             .iter()
//             .position(|preset| *preset == self.filter_query)
//             .unwrap_or(0);
//         let next = (current + 1) % presets.len();
//         self.filter_query = presets[next].to_string();
//     }

//     pub fn set_filter_query(&mut self, query: impl Into<String>) {
//         self.filter_query = query.into();
//     }

//     pub fn begin_search_editing(&mut self) {
//         self.search_editing = true;
//     }

//     pub fn end_search_editing(&mut self) {
//         self.search_editing = false;
//     }

//     pub fn append_query_char(&mut self, ch: char) {
//         if !ch.is_control() {
//             self.filter_query.push(ch);
//         }
//     }

//     pub fn backspace_query(&mut self) {
//         self.filter_query.pop();
//     }

//     pub fn clear_filter_query(&mut self) {
//         self.filter_query.clear();
//     }

//     pub fn handle_global_key(&mut self, key: &str, modifiers:
// &gpui::Modifiers) {         if modifiers.secondary() &&
// key.eq_ignore_ascii_case("f") {             self.begin_search_editing();
//             return;
//         }

//         if modifiers.secondary() && key.eq_ignore_ascii_case("l") {
//             self.clear_filter_query();
//             self.begin_search_editing();
//             return;
//         }

//         if key == "/" {
//             self.begin_search_editing();
//             return;
//         }

//         if self.search_editing {
//             if key == "escape" {
//                 self.end_search_editing();
//             } else if key == "backspace" {
//                 self.backspace_query();
//             } else if key.chars().count() == 1
//                 && !modifiers.control
//                 && !modifiers.alt
//                 && !modifiers.platform
//                 && !modifiers.function
//             {
//                 if let Some(ch) = key.chars().next() {
//                     self.append_query_char(ch);
//                 }
//             }
//         }
//     }

//     pub fn refresh(&mut self) {
//         self.sync_status = SyncStatus {
//             active: true,
//             progress_label: "Refreshing library metadata".to_string(),
//             latency_label: "Last request pending".to_string(),
//             last_update_label: "Refresh in progress".to_string(),
//         };

//         self.shell.dispatch(AppCommand::RefreshLibrary);

//         if let Some(Selection::Item(item_id)) = self.selection {
//             self.shell.dispatch(AppCommand::SelectLibraryItem(item_id));
//         }

//         self.sync_status = SyncStatus {
//             active: false,
//             progress_label: "Library metadata current".to_string(),
//             latency_label: "Last request completed".to_string(),
//             last_update_label: "Updated this session".to_string(),
//         };
//     }

//     pub fn set_item_selection(&mut self, item_id: u64) {
//         self.selection = Some(Selection::Item(item_id));
//         self.shell.dispatch(AppCommand::SelectLibraryItem(item_id));
//     }

//     pub fn set_publisher_selection(&mut self, publisher: String) {
//         self.selection = Some(Selection::Publisher(publisher));
//         self.shell.dispatch(AppCommand::ClearSelection);
//     }

//     pub fn set_product_type_selection(&mut self, product_type: String) {
//         self.selection = Some(Selection::ProductType(product_type));
//         self.shell.dispatch(AppCommand::ClearSelection);
//     }

//     pub fn mode_label(&self) -> &'static str {
//         mode_label(self.view_mode)
//     }

//     pub fn flat_sort_label(&self) -> &'static str {
//         sort_label(self.flat_sort)
//     }

//     pub fn outer_sort_label(&self) -> &'static str {
//         sort_label(self.outer_sort)
//     }

//     pub fn inner_sort_label(&self) -> &'static str {
//         sort_label(self.inner_sort)
//     }

//     pub fn controls_summary(&self) -> String {
//         format!(
//             "{} | query: {} | {} | sections: {}",
//             self.mode_label(),
//             self.active_query_label(),
//             self.active_sort_summary(),
//             self.section_count()
//         )
//     }

//     pub fn active_sort_summary(&self) -> String {
//         match self.view_mode {
//             LibraryViewMode::FlatList => format!("sort: {}",
// self.flat_sort_label()),             _ => format!(
//                 "outer: {}, inner: {}",
//                 self.outer_sort_label(),
//                 self.inner_sort_label()
//             ),
//         }
//     }

//     pub fn account_summary(&self) -> String {
//         let token_status = if self.account.token_configured {
//             "token set"
//         } else {
//             "token missing"
//         };

//         format!("{} ({token_status})", self.account.display_name)
//     }

//     pub fn sync_status_summary(&self) -> String {
//         if self.sync_status.active {
//             format!("Syncing: {}", self.sync_status.progress_label)
//         } else {
//             format!("Sync: {}", self.sync_status.progress_label)
//         }
//     }

//     pub fn sync_status_detail(&self) -> String {
//         format!(
//             "{} | {} | {}",
//             self.sync_status.progress_label,
//             self.sync_status.latency_label,
//             self.sync_status.last_update_label
//         )
//     }

//     pub fn view_summary(&self) -> String {
//         format!(
//             "{} total | {} matched | {} sections",
//             self.shell.items().len(),
//             self.filtered_item_count(),
//             self.section_count()
//         )
//     }

//     pub fn match_presentation_label(&self) -> &'static str {
//         match self.match_presentation {
//             MatchPresentation::HideNonMatching => "Search mode: hide
// non-matching",             MatchPresentation::HighlightMatching => "Search
// mode: highlight matches",         }
//     }

//     pub fn active_query_label(&self) -> String {
//         if self.filter_query.is_empty() {
//             "(none)".to_string()
//         } else {
//             self.filter_query.clone()
//         }
//     }

//     pub fn flat_items(&self) -> Vec<LibraryItem> {
//         let mut items = sorted_flat_items(self.shell.items(),
// self.flat_sort);

//         if matches!(self.match_presentation,
// MatchPresentation::HideNonMatching)             &&
// !self.filter_query.is_empty()         {
//             items.retain(|item| item_matches(item, &self.filter_query));
//         }

//         items
//     }

//     pub fn tree_items(&self) -> Vec<TreeNode> {
//         let mut nodes = grouped_items(
//             self.shell.items(),
//             self.view_mode,
//             self.outer_sort,
//             self.inner_sort,
//         );

//         if self.filter_query.is_empty() {
//             return nodes;
//         }

//         if matches!(
//             self.match_presentation,
//             MatchPresentation::HighlightMatching
//         ) {
//             return nodes;
//         }

//         let query = self.filter_query.clone();

//         nodes.retain_mut(|node| {
//             let root_hit = root_matches(&node.root_label, &query);

//             match self.filter_scope {
//                 FilterScope::ChildOnly => {
//                     node.children.retain(|item| item_matches(item, &query));
//                 }
//                 FilterScope::RootAndChild => {
//                     node.children
//                         .retain(|item| root_hit || item_matches(item,
// &query));                 }
//                 FilterScope::RootOnly => {
//                     if !root_hit {
//                         node.children.clear();
//                     }
//                 }
//             }

//             !node.children.is_empty()
//         });

//         nodes
//     }

//     pub fn grid_sections(&self) -> Vec<TreeNode> {
//         self.tree_items()
//     }

//     pub fn is_item_match(&self, item: &LibraryItem) -> bool {
//         item_matches(item, &self.filter_query)
//     }

//     pub fn is_root_match(&self, root_label: &str) -> bool {
//         root_matches(root_label, &self.filter_query)
//     }

//     pub fn filtered_item_count(&self) -> usize {
//         match self.view_mode {
//             LibraryViewMode::FlatList => self.flat_items().len(),
//             _ => self
//                 .tree_items()
//                 .into_iter()
//                 .map(|node| node.children.len())
//                 .sum(),
//         }
//     }

//     pub fn section_count(&self) -> usize {
//         match self.view_mode {
//             LibraryViewMode::FlatList => 0,
//             _ => self.tree_items().len(),
//         }
//     }

//     pub fn renders_grid(&self) -> bool {
//         mode_is_grid(self.view_mode)
//     }

//     pub fn detail_lines(&self) -> Vec<String> {
//         match &self.selection {
//             Some(Selection::Item(item_id)) => {
//                 if let Some(item) = self.shell.items().iter().find(|item|
// item.id == *item_id) {                     return vec![
//                         "Catalog item detail".to_string(),
//                         format!("Title: {}", item.title),
//                         format!("Publisher: {}", item.publisher),
//                         format!("Type: {}", item.product_type),
//                         format!("Added order: {}", item.added_order),
//                         format!("Updated order: {}", item.updated_order),
//                         format!("Summary: {}", item.summary),
//                     ];
//                 }

//                 vec!["Catalog item detail unavailable.".to_string()]
//             }
//             Some(Selection::Publisher(publisher)) => {
//                 let count = self
//                     .shell
//                     .items()
//                     .iter()
//                     .filter(|item| &item.publisher == publisher)
//                     .count();

//                 vec![
//                     "Publisher detail".to_string(),
//                     format!("Publisher: {}", publisher),
//                     format!("Items in library: {}", count),
//                     "Publisher metadata is derived from SDK library
// responses.".to_string(),                 ]
//             }
//             Some(Selection::ProductType(product_type)) => {
//                 let count = self
//                     .shell
//                     .items()
//                     .iter()
//                     .filter(|item| &item.product_type == product_type)
//                     .count();

//                 vec![
//                     "Product type detail".to_string(),
//                     format!("Type: {}", product_type),
//                     format!("Items in library: {}", count),
//                     "Suggested arrangement enabled: tree grouped by product
// type.".to_string(),                 ]
//             }
//             None => vec!["Select a publisher or catalog item to view
// details.".to_string()],         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::services::stub::{StubLibraryService, StubMode};

//     fn make_controller() -> LibraryController {
//         let library =
// LibraryViewModel::new(Box::new(StubLibraryService::new(StubMode::Seeded)));
//         let mut shell = AppShell::new(
//             AppShellState {
//                 session: SessionPresentationState::SignedIn,
//                 library: LibraryPaneState::Loading,
//                 selected_item_id: None,
//                 status_message: "Loading your library…".to_string(),
//             },
//             library,
//         );
//         shell.dispatch(AppCommand::LoadLibrary);

//         LibraryController {
//             shell,
//             view_mode: LibraryViewMode::TreeByPublisher,
//             filter_scope: FilterScope::ChildOnly,
//             match_presentation: MatchPresentation::HideNonMatching,
//             flat_sort: SortMethod::AtoZ,
//             outer_sort: SortMethod::AtoZ,
//             inner_sort: SortMethod::AtoZ,
//             filter_query: String::new(),
//             search_editing: false,
//             controls_disclosed: true,
//             open_sort_popup: None,
//             selection: None,
//             account: AccountMenuState::signed_out(),
//             sync_status: SyncStatus::idle(),
//         }
//     }

//     #[test]
//     fn controls_disclosure_preserves_browsing_summary() {
//         let mut controller = make_controller();
//         controller.set_filter_query("atlas");
//         controller.set_view_mode(LibraryViewMode::GridByPublisher);

//         let expanded_summary = controller.controls_summary();
//         controller.toggle_controls_disclosure();

//         assert!(!controller.controls_disclosed);
//         assert_eq!(controller.filter_query, "atlas");
//         assert_eq!(controller.controls_summary(), expanded_summary);
//         assert!(controller.controls_summary().contains("Grid by publisher"));
//     }

//     #[test]
//     fn grid_and_tree_presentations_share_filtered_result_state() {
//         let mut controller = make_controller();
//         controller.set_filter_query("atlas");

//         controller.set_view_mode(LibraryViewMode::TreeByPublisher);
//         let tree_count = controller.filtered_item_count();
//         let tree_sections = controller.section_count();

//         controller.set_view_mode(LibraryViewMode::GridByPublisher);

//         assert!(controller.renders_grid());
//         assert_eq!(controller.filtered_item_count(), tree_count);
//         assert_eq!(controller.section_count(), tree_sections);
//     }

//     #[test]
//     fn account_actions_do_not_store_raw_token_values() {
//         let mut controller = make_controller();

//         controller.toggle_account_menu();
//         assert!(controller.account.menu_open);

//         controller.mark_token_set_action();
//         assert!(controller.account.token_configured);
//         assert!(!controller.account.menu_open);
//         assert!(!controller.account_summary().contains("DTRPG_ACCESS_TOKEN"
// ));

//         controller.mark_token_reset_action();
//         assert!(!controller.account.token_configured);
//         assert!(!controller.account_summary().contains("DTRPG_ACCESS_TOKEN"
// ));     }

//     #[test]
//     fn refresh_updates_sync_status_without_changing_browsing_state() {
//         let mut controller = make_controller();
//         controller.set_filter_query("atlas");
//         controller.set_view_mode(LibraryViewMode::GridByProductType);
//         let summary = controller.controls_summary();

//         controller.refresh();

//         assert!(!controller.sync_status.active);
//         assert_eq!(controller.filter_query, "atlas");
//         assert_eq!(controller.view_mode, LibraryViewMode::GridByProductType);
//         assert_eq!(controller.controls_summary(), summary);
//         assert!(
//             controller
//                 .sync_status_summary()
//                 .contains("Library metadata")
//         );
//     }
// }
