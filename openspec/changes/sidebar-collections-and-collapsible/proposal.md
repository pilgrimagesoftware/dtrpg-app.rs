## Why

The sidebar currently shows a flat, non-interactive Publishers list with no way to navigate by DTRPG Collections (user-curated product lists). Adding a Collections section lets users browse their library through the groupings they've already set up on DriveThruRPG, and making both sections collapsible keeps the sidebar tidy when those lists are long.

## What Changes

- A new **Collections** section appears below Publishers in the sidebar, listing the user's DTRPG product lists by name with item counts. Clicking a collection filters the catalog to show only items in that list.
- The **Publishers** section gains a collapse/expand toggle header; clicking it hides or shows the publisher rows.
- The **Collections** section also has a collapse/expand toggle header.
- `LibraryService` gains a `list_collections()` method; `SdkLibraryService` implements it by calling the SDK's `list_product_lists` and `list_product_list_items` endpoints.
- `SidebarFilter` gains a `Collection(Arc<str>)` variant so the catalog can be filtered by collection name.
- `LibraryController` tracks which items belong to which collections (mapping collection name → set of product IDs), and exposes collapsed/expanded state for both sidebar sections.

## Capabilities

### New Capabilities

- `sidebar-collections`: Sidebar section that fetches and displays the user's DTRPG product lists and filters the catalog by the selected collection.
- `sidebar-collapsible-sections`: Publishers and Collections sections can each be independently collapsed or expanded via a clickable header toggle.

### Modified Capabilities

<!-- none — SidebarFilter.Publisher behavior is unchanged; the new Collection variant is additive -->

## Impact

- `dtrpg-ui/src/services/mod.rs` — add `list_collections()` to `LibraryService` trait; add `LibraryCollection` data type
- `dtrpg-core/src/services/sdk.rs` — implement `list_collections()` using `runtime.block_on(client.list_product_lists(...))` and `list_product_list_items(...)`
- `dtrpg-ui/src/util/filter.rs` — add `SidebarFilter::Collection(Arc<str>)` variant
- `dtrpg-ui/src/controllers/library.rs` — add `collections: Vec<LibraryCollection>`, `collection_membership: HashMap<Arc<str>, HashSet<u64>>`, `publishers_collapsed: bool`, `collections_collapsed: bool`; load collections after catalog load
- `dtrpg-ui/src/ui/views/sidebar_view.rs` — add collapsible section headers for Publishers and Collections; render the Collections nav rows
- `dtrpg-ui/src/ui/views/root_view.rs` — pass new sidebar state fields through to `render_sidebar`
- SDK (`dtrpg-sdk/rust`): read-only — `list_product_lists` and `list_product_list_items` already exist
