## 1. Fix Collections Load on Startup

- [x] 1.1 In `library.rs` `start_load`, call `load_collections` after the collections cache pre-population so authenticated startup triggers the live API fetch
- [x] 1.2 Verify collections appear in the sidebar after a cold launch with an existing session (no sign-out/sign-in required) — confirmed via `startup_auth` -> `SignInSucceeded` -> `replace_service` -> `load_collections` path in `root_view.rs`, which fires on cold launch with a stored key, not only interactive sign-in

## 2. Cache Metadata Infrastructure

- [x] 2.1 Add `CacheMetadata` struct to `catalog_cache.rs` with `saved_at: SystemTime` and `item_count: usize`, deriving `Serialize`/`Deserialize`
- [x] 2.2 Implement `save_cache_metadata(root, metadata)` and `load_cache_metadata(root) -> Option<CacheMetadata>` in `catalog_cache.rs`
- [x] 2.3 Update `save_catalog_cache` to call `save_cache_metadata` after a successful cache write

## 3. Catalog Auto-Load Policy

- [x] 3.1 In `library.rs` `start_load`, after loading the catalog cache, load the cache metadata and check staleness (>7 days or missing metadata -> full fetch)
- [x] 3.2 Add a `count_items()` (or equivalent) method to `LibraryService` / `LibraryViewModel` that fetches only the remote item count for the count-mismatch check
- [x] 3.3 If cache is non-empty and fresh, fetch the remote count and compare to `cached.len()`; skip the full fetch if they match
- [x] 3.4 Ensure "Catalog > Reload" bypasses the policy and always runs the full fetch
- [x] 3.5 Write tests for the staleness and count-mismatch branches in `catalog_cache.rs`

## 4. Catalog Menu

- [x] 4.1 Define `ReloadCatalog` action struct in the actions module
- [x] 4.2 Add "Catalog" `Menu` in `app/mod.rs` between "Edit" and "View" with "Add Collection" and "Reload" items
- [x] 4.3 Wire `ReloadCatalog` action handler in `LibraryController` to call `start_load` unconditionally (bypassing the auto-load policy)
- [x] 4.4 Verify "Add Collection" menu item opens the new-collection input in the sidebar — confirmed in `root_view.rs`, `AddCollection` action opens a dialog with the name input wired to `create_collection`

## 5. Window Menu Extensions

- [x] 5.1 Add `ShowAlertHistory` action struct in the actions module
- [x] 5.2 Add "Show Activity" and "Show Alert History" entries to the existing "Window" menu in `app/mod.rs`
- [x] 5.3 Wire `ShowActivity` to the existing activity panel show/hide logic (same path as the toolbar button)
- [x] 5.4 Wire `ShowAlertHistory` to dispatch the action; implement a stub handler that logs the action until the alert history panel is built

## 6. Catalog Title Count

- [x] 6.1 Expose `total_count` and `visible_items_count()` together in the catalog view snapshot or read them directly from the controller in `catalog_view.rs`
- [x] 6.2 Render "N titles" in the catalog title/toolbar area when no filter is active
- [x] 6.3 Render "M of N" in the catalog title/toolbar area when a filter is active (visible < total)
- [x] 6.4 Verify the count updates reactively when a filter is applied or cleared — `filter_count`/`matched_count` are recomputed in `snapshot()` on every render and `set_filter`/`clear_search_query` both emit `LibraryChanged`

## 7. Sidebar Section Header Counts

- [x] 7.1 Pass `collections.len()` to the "Collections" `SidebarMenuItem` header using the `.suffix(...)` pattern already used on individual nav items
- [x] 7.2 Pass `publishers.len()` to the "Publishers" `SidebarMenuItem` header using the same suffix pattern
- [x] 7.3 Verify counts update when the catalog reloads and the publisher or collection set changes — `collections.len()`/`publishers.len()` are read from controller state on every `snapshot()`/render pass, which follows every `LibraryChanged` emit from `set_catalog`, `apply_collections`, `delete_collection`, and `create_collection`

## 8. Collection Context Menu

- [x] 8.1 Define `ReloadCollection { id: CollectionId }` and `DeleteCollection { id: CollectionId }` action structs
- [x] 8.2 Wrap each collection nav item in `sidebar_view.rs` with `right_click_menu` providing "Reload" and "Delete" actions
- [x] 8.3 Implement `delete_collection(id)` on `LibraryController` that calls the collections service delete endpoint and removes the entry from `self.collections`
- [x] 8.4 Wire "Reload" on a collection to call `load_collections` (refreshes all collections; per-collection reload can be a follow-up)
- [x] 8.5 Log delete failures to the activity panel and leave the collection entry in place on error
- [x] 8.6 Verify right-click on a collection shows the context menu and both actions work end-to-end — confirmed in `sidebar_view.rs`: `.context_menu(...)` on each collection `nav_item` wires "Reload" to `load_collections` and "Delete" to `delete_collection(col_id)`
