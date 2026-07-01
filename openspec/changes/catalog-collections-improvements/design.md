## Context

The app currently:
- Always fetches the live catalog from the API on startup via `start_load`, regardless of cache freshness. The cache is used only as a pre-population while the live fetch runs.
- Loads collections from cache in `start_load` and from the API in `load_collections`, but `load_collections` is only called from `replace_service` (post-auth), not on startup when a session is already present. This is the root cause of collections not loading for users who are already signed in.
- Renders individual item counts in sidebar nav entries but the "Collections" and "Publishers" section headers show no aggregate counts.
- Has no "Catalog" menu entry and no "Show Activity" / "Show Alert History" actions in the "Window" menu.
- Does not show total or filtered count in the catalog title bar.

## Goals / Non-Goals

**Goals:**
- Fix collections not loading on startup (already-authenticated sessions)
- Add smart auto-load logic for the catalog: only hit the API when cache is empty, stale (>1 week), or the remote count differs from the cached count
- Show aggregate item counts next to "Collections" and "Publishers" sidebar section headers
- Add a "Catalog" menu with "Add Collection" and "Reload" actions
- Extend the "Window" menu with "Show Activity" and "Show Alert History" actions
- Display item count in the catalog title: total when unfiltered, filtered/total when a filter is active
- Add a right-click context menu on collection sidebar entries: Reload and Delete

**Non-Goals:**
- Changing the API contract or SDK layer
- Offline mode or background sync scheduling
- Alert history UI implementation (only the menu entry and panel toggle)
- Sorting or filtering of collections

## Decisions

### Collections load on startup

The fix is to call `load_collections` from `start_load` (after the collections cache pre-population) in addition to `replace_service`. Currently `start_load` only loads the collections cache; it never makes the live API call. `load_collections` should be invoked unconditionally inside the spawn closure so that both the cache path and the live API path run on startup.

Alternatives considered:
- Detect auth state before calling `load_collections`: more complex and `load_collections` already handles the error case gracefully (logs and drops the error).

### Catalog auto-load policy

Currently `start_load` always runs a full live fetch. The new policy:

1. Load the disk cache as before (pre-population).
2. Check cache freshness: if the cache is non-empty and was written within the last 7 days, skip the live fetch.
3. If the cache is present and within the staleness window, fetch only the remote item count (a cheap API call). If the count matches the cached item count, skip the full fetch.
4. Otherwise, run the full paginated fetch as today.

The cache metadata (write timestamp, item count) is stored alongside the existing cache file. A new `CacheMetadata` struct holds `saved_at: SystemTime` and `item_count: usize`, serialized to a sidecar JSON file next to the catalog cache.

Alternatives considered:
- Store metadata inside the cache file: would require reading the full file just to check staleness, defeating the purpose.
- Use file mtime: fragile across OS/filesystem combinations.

### Sidebar section header counts

The "Collections" and "Publishers" `SidebarMenuItem` headers are created in `sidebar_view.rs`. Each already has child items with individual counts. The header itself can receive a count badge using the same `.suffix(...)` pattern already used on nav items. The count is the number of child items (i.e., `collections.len()` and `publishers.len()`).

### Catalog menu

A new "Catalog" `Menu` is inserted between "Edit" and "View" in `app/mod.rs`. It contains:
- "Add Collection" - dispatches an existing or new `AddCollection` action (the sidebar already has this flow)
- "Reload" - dispatches a new `ReloadCatalog` action handled by `LibraryController`

### Window menu extensions

The existing "Window" menu gains two new entries after "Zoom":
- "Show Activity" - dispatches `ShowActivity` action (currently triggered via toolbar button)
- "Show Alert History" - dispatches a new `ShowAlertHistory` action; the panel is a stub that can be wired later

### Catalog title count

`catalog_view.rs` has access to `visible_items_count()` (filtered) and `total_count` from the snapshot. The title area renders the count as:
- `"N titles"` when `visible == total`
- `"M of N"` when `visible < total` (a filter is active)

The count is appended to or embedded in the existing toolbar/title area, consistent with the sidebar footer which already renders `"N titles"`.

### Collection context menu

Context menus in gpui are rendered via `right_click_menu`. Each collection nav item in the sidebar is an existing custom element. A `right_click_menu` wrapping the item offers:
- "Reload" - calls `load_collections` on `LibraryController`
- "Delete" - calls a new `delete_collection(id)` on `LibraryController` which delegates to `CollectionsService`

## Risks / Trade-offs

- [Auto-load count check adds an extra API call on startup] → Mitigation: only run it when the cache is present and within the staleness window, making it the fast path. The full fetch is the slow path.
- [Collections context menu delete is destructive] → Mitigation: no undo; the action delegates to the SDK which calls the API. Acceptable for MVP; a confirmation dialog can be added later.
- [Cache metadata sidecar is a new file on disk] → Mitigation: absence of the sidecar is treated as "stale"; the full fetch runs as before. Old installs degrade gracefully.
