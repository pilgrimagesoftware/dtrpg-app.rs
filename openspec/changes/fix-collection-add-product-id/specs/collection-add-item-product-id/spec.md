## ADDED Requirements

### Requirement: Adding an item to a collection sends the catalog product id
When a catalog item is added to a collection (via drag-and-drop, the catalog item context
menu, or the Manage Collections dialog), the request sent to the API SHALL use the item's
catalog `product_id`, not its `order_product_id`, as the product identifier.

#### Scenario: Item's order_product_id and product_id differ
- **WHEN** a catalog item whose `order_product_id` differs from its `product_id` is added to a
  collection
- **THEN** the add-to-collection API request is sent with the item's `product_id`, and the
  request succeeds rather than being rejected with an invalid-product-id error

#### Scenario: Local membership state is unaffected
- **WHEN** an item is added to or removed from a collection
- **THEN** the collection's cached member ids, the "already a member" check, and sidebar/detail
  filtering continue to use the same id (`order_product_id`, falling back to `product_id`) they
  used before this change
