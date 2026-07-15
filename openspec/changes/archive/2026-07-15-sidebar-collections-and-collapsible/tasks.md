## 1. Data model

- [x] 1.1 Add `LibraryCollection { id: u64, name: Arc<str>, item_count: usize }` to `dtrpg-ui/src/data/library.rs`
- [x] 1.2 Add `Collection(Arc<str>)` variant to `SidebarFilter` in `dtrpg-ui/src/util/filter.rs`
- [x] 1.3 Add `CollectionEntry { name: Arc<str>, count: usize }` (parallel to `PublisherEntry`) to `dtrpg-ui/src/util/publisher.rs` (or a new `collection.rs` util)

## 2. LibraryService trait and SDK implementation

- [x] 2.1 Add `fn list_collections(&self) -> Result<Vec<LibraryCollection>, LibraryServiceError>` to `LibraryService` trait in `dtrpg-ui/src/services/mod.rs`
- [x] 2.2 Implement `list_collections` in `SdkLibraryService` (`dtrpg-core/src/services/sdk.rs`): call `runtime.block_on(client.list_product_lists(...))`, then for each list call `block_on(client.list_product_list_items(id))` and extract `productId` u64 values from the raw JSON, returning `Vec<LibraryCollection>` with membership sets stored separately
- [x] 2.3 Add a `fn list_collection_items(&self, collection_id: u64) -> Result<HashSet<u64>, LibraryServiceError>` helper in `sdk.rs` that parses `productId` from the raw JSON items (skip and warn on unparseable entries)
- [x] 2.4 Add a default no-op implementation of `list_collections` to `LibraryService` trait (returns empty vec) so existing stub/test implementations compile without changes

## 3. LibraryController state

- [x] 3.1 Add `collections: Vec<CollectionEntry>`, `collection_membership: HashMap<Arc<str>, HashSet<u64>>`, `publishers_collapsed: bool`, `collections_collapsed: bool` fields to `LibraryController`
- [x] 3.2 Add `toggle_publishers_collapsed(&mut self, cx)` and `toggle_collections_collapsed(&mut self, cx)` methods that flip the bool and emit `LibraryChanged`
- [x] 3.3 Add `load_collections` background task method: spawn after catalog load completes; on success call `apply_collections(collections, membership, cx)`; on failure log the error and leave `self.collections` empty
- [x] 3.4 Add `apply_collections(collections, membership, cx)` method that stores the data and emits `LibraryChanged`
- [x] 3.5 Extend `LibrarySnapshot` to include `collections: Vec<CollectionEntry>`, `collection_membership: HashMap<Arc<str>, HashSet<u64>>` (or just the active set), `publishers_collapsed: bool`, `collections_collapsed: bool`
- [x] 3.6 Update `visible_items()` to handle `SidebarFilter::Collection(name)`: filter catalog items where `item.numeric_id` is in `collection_membership[name]`

## 4. Sidebar view

- [x] 4.1 Update `render_sidebar` signature to accept `publishers_collapsed: bool`, `collections_collapsed: bool`, `collections: &[CollectionEntry]`, and the `lib_entity` reference for toggle callbacks
- [x] 4.2 Replace the static `PUBLISHERS` label `div` with a clickable `render_section_header("PUBLISHERS", publishers_collapsed, toggle_callback)` helper
- [x] 4.3 Gate the publisher rows behind `if !publishers_collapsed { ... }`
- [x] 4.4 Add a `render_section_header("COLLECTIONS", collections_collapsed, toggle_callback)` and collection nav rows below the publishers section, gated behind `if !collections_collapsed { ... }` and only rendered when `!collections.is_empty()`

## 5. Root view wiring

- [x] 5.1 Extract `publishers_collapsed`, `collections_collapsed`, `collections` from the snapshot in `LibraryRootView::render` and pass them to `render_sidebar`
- [x] 5.2 Call `load_collections` from `LibraryController::new` after the catalog fetch completes (chain into the existing background spawn)

## 6. Verify

- [x] 6.1 Run `cargo test --all-features --workspace` and confirm all tests pass
- [x] 6.2 Manually launch the app and confirm the Publishers section collapses and expands on header click
- [x] 6.3 Confirm the Collections section appears, lists product lists by name with catalog intersection counts, and filters the catalog on click
- [x] 6.4 Confirm collapsing one section does not affect the other
