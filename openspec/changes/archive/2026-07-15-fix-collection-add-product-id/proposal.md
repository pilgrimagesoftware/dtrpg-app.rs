## Why

Adding a catalog item to a collection fails against the live API with HTTP 409:
`productId: Requires a valid Product ID. Invalid value 22654728.` The app sends the item's
`order_product_id` (the per-order purchase id) as the request's `productId`, but the API's
`product_list_items` endpoint requires the catalog `product_id` — a distinct field. Every
add-to-collection attempt where `order_product_id` differs from `product_id` fails.

## What Changes

- `collection_member_id()`'s order_product_id-preferring value is no longer sent as the
  network payload for adding an item to a collection. The add path now sends `item.product_id`
  specifically, while membership matching, the "already a member" check, and removal continue
  to use the existing `collection_member_id()` value unchanged (removal already matches against
  either id via `extract_member_id`, so it is unaffected by which id was used to add).
- No change to collection membership matching/filtering, removal, or the collections cache —
  only the id sent on the add (`POST /product_list_items`) request body changes.

## Capabilities

### New Capabilities
- `collection-add-item-product-id`: Adding a catalog item to a collection sends the item's
  catalog `product_id`, not its `order_product_id`, as the API request's product identifier.

### Modified Capabilities
<!-- none -->

## Impact

- `dtrpg-app/rust/crates/dtrpg-ui/src/ui/views/catalog_view.rs` (drag payload, "Add to..."
  context menu action) and `detail_panel_view.rs`: the value used to build the add-to-collection
  request must be `item.product_id`, not `collection_member_id(item)`.
- `dtrpg-app/rust/crates/dtrpg-ui/src/controllers/library.rs`
  (`LibraryController::add_item_to_collection`,
  `create_collection_and_add_member`): needs a distinct product id parameter for the network
  call, separate from the member id used for the optimistic local `member_ids` update.
- `dtrpg-app/rust/crates/dtrpg-core/src/services/collections_sdk.rs`: `SdkCollectionsGateway::
  add_product_list_item`'s parameter is currently named/documented as `order_product_id`;
  rename/re-document to reflect that it must be a catalog `product_id`.
- Not in scope (tracked separately): `dtrpg-sdk/rust`'s `decode_response` swallows the API's
  actual error message on non-2xx responses (it unconditionally tries to deserialize the body
  as the success type), which is why this bug initially surfaced as a confusing "missing field
  productId" decode error instead of the API's real message. That's a genuine, independently
  useful robustness fix affecting every SDK client method, not just this one — worth its own
  change rather than bundling an SDK-repo error-handling improvement into this app-repo bug fix.
