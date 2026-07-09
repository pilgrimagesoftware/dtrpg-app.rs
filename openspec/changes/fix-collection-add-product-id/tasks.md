## 1. Controller: thread a distinct product_id through the add path

- [ ] 1.1 In `crates/dtrpg-ui/src/controllers/library.rs`, add a `product_id: u64` parameter to
  `add_item_to_collection(&mut self, collection_id: u64, item_id: u64, product_id: u64, cx: &mut
  Context<Self>)`; keep using `item_id` for the optimistic `member_ids`/`collection_members`
  update and its rollback-on-failure branch
- [ ] 1.2 Change the `collections_service.add_member(collection_id, item_id)` call inside
  `add_item_to_collection`'s spawned task to `collections_service.add_member(collection_id,
  product_id)`
- [ ] 1.3 Add a `product_id: u64` parameter to `create_collection_and_add_member`, threading it
  through to its internal `add_item_to_collection` call
- [ ] 1.4 Update both methods' doc comments to explain the distinction: `item_id` for local
  membership tracking/matching, `product_id` for the network call

## 2. Service/gateway: name the parameter for what it is

- [ ] 2.1 In `crates/dtrpg-ui/src/services/collections.rs`, update `CollectionsService::
  add_member`'s doc comment to state its `item_id` parameter must be the catalog `product_id`
- [ ] 2.2 In `crates/dtrpg-core/src/services/collections_sdk.rs`, rename
  `SdkCollectionsGateway::add_product_list_item`'s second parameter from `order_product_id` to
  `product_id` (trait definition, `HttpSdkCollectionsGateway` impl, `UnavailableCollectionsGateway`
  impl, and the `FakeCollectionsGateway` test double) and update doc comments accordingly

## 3. Call sites: pass item.product_id to the add path

- [ ] 3.1 In `crates/dtrpg-ui/src/ui/views/catalog_view.rs`, at both `on_drag`/`DraggedLibraryItem`
  construction sites, keep `member_id: collection_member_id(item)` for the drag payload's
  existing role, and ensure the sidebar drop handler (`sidebar_view.rs`) passes `item.product_id`
  as the new argument when it calls `add_item_to_collection`
- [ ] 3.2 In `catalog_view.rs`'s "Add to..." context-menu action (`append_collection_menu_items`
  or equivalent) and `detail_panel_view.rs`, pass `item.product_id` as the new
  `add_item_to_collection`/`create_collection_and_add_member` argument
- [ ] 3.3 In `crates/dtrpg-ui/src/ui/views/manage_collections_dialog.rs`, thread a
  `product_id: u64` alongside the existing `member_id: u64` parameter through
  `render_collection_row`/its caller, passing `item.product_id` to `add_item_to_collection`
  while `member_id` continues to drive the checked-state check and
  `remove_item_from_collection`

## 4. Verify

- [ ] 4.1 Run `cargo check --workspace --all-targets`
- [ ] 4.2 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 4.3 Run `cargo fmt --all -- --check`
- [ ] 4.4 Run `cargo test --workspace` and confirm no new failures
- [ ] 4.5 Manually add an item whose `order_product_id` differs from its `product_id` to a
  collection (via drag-and-drop, the context menu, and the Manage Collections dialog) and
  confirm the request succeeds (no 409, item persists in the collection after a reload)
- [ ] 4.6 Manually confirm removal, the "already a member" checked state, and collection
  filtering still behave correctly for items added under this change
