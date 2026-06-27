## Context

The sidebar is a stateless `render_sidebar(...)` function. All sidebar data comes from `LibraryController::snapshot()` passed through `LibraryRootView::render`. The controller already holds `publishers: Vec<PublisherEntry>` and exposes it in the snapshot.

The SDK `DtrpgClient` already exposes:
- `list_product_lists()` → `ProductListCollectionResponse` (collection name, id, item_count)
- `list_product_list_items(product_list_id)` → `ProductListItemsResponse` (data as `Vec<serde_json::Value>`)

`SdkLibraryService` in `dtrpg-core` calls SDK async methods via `runtime.block_on(...)`.

`ProductListItemsResponse.data` is untyped (`Vec<serde_json::Value>`) because the item schema is not yet formally specified in the API. The response objects are expected to have a `productId` field (integer) based on observed API behavior, so we parse that field with `value["productId"].as_u64()`.

## Goals / Non-Goals

**Goals:**
- Fetch collections (product lists) and their membership after the catalog loads.
- Filter the catalog to items whose `numeric_id` is in the selected collection.
- Render a collapsible Collections section in the sidebar.
- Make the existing Publishers section collapsible with the same toggle pattern.
- Persist collapsed/expanded state across re-renders (in-memory in the controller).

**Non-Goals:**
- Creating, editing, or deleting DTRPG Collections from within Libri.
- Persisting collapsed/expanded state to disk.
- Paginating within the `list_product_list_items` endpoint (assume one page for now; revisit if users have very large collections).
- Showing items that are in a collection but not in the user's library (intersection only).

## Decisions

### LibraryCollection data type in dtrpg-ui

Introduce `LibraryCollection` in `dtrpg-ui/src/data/library.rs`:

```
pub struct LibraryCollection {
    pub id: u64,          // product_list_id
    pub name: Arc<str>,   // display name
    pub item_count: usize,
}
```

Store `collections: Vec<LibraryCollection>` and `collection_membership: HashMap<Arc<str>, HashSet<u64>>` (collection name → set of product IDs) in `LibraryController`.

**Why**: The catalog is already keyed by `numeric_id` (product ID). A `HashMap<Arc<str>, HashSet<u64>>` lookup is O(1) per item during filtering, matching the pattern used for publisher filtering.

### Collections loaded after catalog completes

Collections are fetched in a second background task spawned after the catalog load task finishes (or in the same task, sequentially). Collection membership is eagerly fetched for all collections at startup.

**Why**: The catalog must exist before we can intersect; loading sequentially avoids a race. The number of collections is typically small (< 50), and each `list_product_list_items` call is cheap.

**Alternative**: Lazy membership fetch triggered by selecting a collection. Rejected — it introduces a loading state mid-filter interaction. Eager loading is simpler and the extra network cost at startup is low.

### LibraryService.list_collections() returns Vec<LibraryCollection>

Add to the `LibraryService` trait:
```
fn list_collections(&self) -> Result<Vec<LibraryCollection>, LibraryServiceError>;
```

Membership is a separate concern handled by `SdkLibraryService`: for each returned `LibraryCollection`, call `list_product_list_items(id)` and extract `productId` integers from the raw JSON.

**Why**: Keeping membership loading in the service layer hides the SDK's untyped JSON detail from the controller.

### Collapsible state in LibraryController

Add `publishers_collapsed: bool` (default `false`) and `collections_collapsed: bool` (default `false`) to `LibraryController`. Expose toggle methods: `toggle_publishers_collapsed` and `toggle_collections_collapsed`. Include these in `LibrarySnapshot`.

**Why**: The sidebar render function is stateless, so collapsed state must come from the controller. Using the existing snapshot pattern avoids threading GPUI entity handles through the render function.

**Alternative**: A separate `SidebarController`. Rejected — the sidebar collapse state is directly coupled to the library filter context; keeping it in `LibraryController` avoids adding another entity.

### SidebarFilter::Collection(Arc<str>)

Add a `Collection(Arc<str>)` variant to `SidebarFilter` (keyed by collection name, not ID).

**Why**: Consistent with `Publisher(Arc<str>)`. Names are stable within a session and are what the user sees.

### Collapsible section header rendering

A "section header" row renders the section label (e.g. `PUBLISHERS`) as a clickable button. When expanded: a downward chevron (`˅`). When collapsed: a rightward chevron (`›`). Clicking calls the appropriate `toggle_*_collapsed` method on `LibraryController`.

**Why**: Simple, no external icon dependency. The same unicode characters used in other parts of the UI.

## Risks / Trade-offs

- [Risk] `productId` field name in `list_product_list_items` JSON is based on observed behavior, not a formal schema. → Mitigation: filter with `as_u64()` and skip items that don't parse; log a warning at most once per session.
- [Risk] A user with many large collections incurs multiple network round-trips at startup. → Mitigation: load collections concurrently (one task per list using `join_all` or sequential `block_on` loop); document as a known startup cost. Paginated fetching can be added later.
- [Risk] A collection item's product may not be in the user's library (e.g. a wishlist item). → Mitigation: intersection-only display — silently exclude non-matching IDs from the filter result; the count shown in the sidebar reflects catalog items, not the API item_count.

## Open Questions

- Should the sidebar show the API `item_count` from the product list attributes (fast, from the list endpoint) or recount from the catalog intersection (accurate, but slightly more work)? → Use the catalog intersection count for consistency with how Publishers counts work.
