## Context

`LibraryItem` carries two distinct numeric ids from the API's `OrderProduct` resource:
`order_product_id` (the per-purchase/order record id) and `product_id` (the catalog product
id). `collection_member_id()` (`util/matching.rs`) picks `order_product_id` when non-zero,
falling back to `product_id` — this value is used uniformly today as `item_id` everywhere
collection membership is touched: the drag-and-drop payload, the "Add to..." context menu, the
Manage Collections dialog's checked state, and `LibraryController::add_item_to_collection`/
`remove_item_from_collection`.

That uniform value is correct for membership *matching* (`item_matches_filter` already checks
both `order_product_id` and `product_id` against the collection's cached member ids, and
`list_collections` populates those cached ids via the same dual-field check via
`extract_member_id`). It is wrong for the network *add* call specifically: the API's
`POST /product_list_items` requires the catalog `product_id` in its `productId` field, and
rejects an `order_product_id` value with HTTP 409.

## Goals / Non-Goals

**Goals:**
- The add-to-collection network request always sends `item.product_id`, regardless of what
  `collection_member_id()` would have picked for matching purposes.
- Leave every other membership code path (matching, removal, the checked state in the Manage
  Collections dialog, the local optimistic `member_ids` cache) untouched — they are not broken.

**Non-Goals:**
- Migrating the whole membership id space to use `product_id` exclusively everywhere and retire
  `collection_member_id()`'s order_product_id-preferring behavior. That would be a larger,
  riskier change (the fallback exists for a reason not investigated here — some items may have
  a `order_product_id` of 0), and isn't needed: removal and matching already tolerate either id
  via `extract_member_id`'s dual check, so nothing downstream of the add call needs to change.
- Fixing `dtrpg-sdk/rust`'s `decode_response` (the error-message-swallowing issue that made this
  bug's symptom confusing). Tracked as a separate, independently useful change.

## Decisions

- **Add a dedicated `product_id: u64` parameter to `LibraryController::add_item_to_collection`
  and `create_collection_and_add_member`, alongside the existing `item_id: u64`.** `item_id`
  (still `collection_member_id()`'s value) continues to drive the optimistic local `member_ids`/
  `collection_members` update and the rollback-on-failure path, unchanged. Only the
  `collections_service.add_member(collection_id, ...)` call switches from `item_id` to the new
  `product_id` parameter.
  - Alternative considered: change `collection_member_id()` itself to prefer `product_id`.
    Rejected — that would change the id stored in `member_ids`/the collections cache globally
    (affecting matching, filtering, and every existing collection's cached membership), a much
    larger blast radius than fixing the one call site that's actually wrong.
  - Alternative considered: resolve `product_id` from `item_id` inside the gateway (e.g. look
    the item up by `item_id` before calling the API). Rejected — the gateway has no access to
    the in-memory catalog/library item list; threading the already-known `product_id` from the
    UI layer (where the full `LibraryItem` is in hand) down through the existing call chain is
    simpler and avoids a new cross-layer lookup.
- **Rename `SdkCollectionsGateway::add_product_list_item`'s second parameter from
  `order_product_id` to `product_id`** (and update its doc comment) so the trait signature
  documents what it actually requires, preventing this same mistake from recurring at that
  layer. `CollectionsService::add_member`'s doc comment (`services/collections.rs`) gets the
  same correction.
- **Call sites that build the add payload (`catalog_view.rs`'s drag payload and "Add to..."
  menu, `detail_panel_view.rs`, `manage_collections_dialog.rs`) pass `item.product_id`
  directly** rather than `collection_member_id(item)`, at the specific point where the value
  feeds into `add_item_to_collection`/`create_collection_and_add_member`. Where the same call
  site also needs the matching/removal id (e.g. the Manage Collections dialog's checked-state
  check and its remove branch), it keeps using `collection_member_id(item)` as before —  the two
  values are now both threaded through, not one replacing the other.

## Risks / Trade-offs

- [Two ids now flow through the same functions (`item_id` for matching/optimistic state,
  `product_id` for the network call), which is more parameters to keep straight than a single
  id] → Mitigated by naming (`item_id` vs `product_id`, not `item_id`/`item_id_2`) and doc
  comments on both controller methods explaining which one is used for what.
- [If some `LibraryItem`s genuinely have `product_id == 0` (unpopulated), sending that would
  produce the same class of 409 for those items] → Not observed in the reported bug, but worth
  a manual check during verification: confirm no catalog item has `product_id == 0` in a real
  library, and if any do, surface that as a separate follow-up rather than blocking this fix
  (this fix strictly improves on the current always-wrong-when-ids-differ behavior).
