## MODIFIED Requirements

### Requirement: Cached publisher grouping backs the grouped table's row list

The cached `Vec<PublisherGroup>` on `CatalogView` (`grouped_cache`) SHALL back the
flattened `GroupedRow` list pushed into `catalog_grouped_list_table`, in addition to any
existing consumers, so grouped list rendering does not require a separate hand-rolled
grouping pass.

#### Scenario: Grouped rows read from the existing cache

- **WHEN** `CatalogView::grouped_items` is called while `grouped_cache` is populated
- **THEN** it returns the cached grouping without re-grouping the catalog

#### Scenario: Cache invalidation still triggers a re-group

- **WHEN** `LibraryChanged` fires
- **THEN** `grouped_cache` is invalidated and the next call to `grouped_items` re-groups the
  catalog and rebuilds the flattened `GroupedRow` list
