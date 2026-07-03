## MODIFIED Requirements

### Requirement: Cached publisher grouping backs delegate section lookups

The cached `Vec<PublisherGroup>` on `LibraryController` SHALL back
`CatalogListDelegate`'s `sections_count` and `items_count` lookups, in addition to any
existing consumers, so grouped list rendering does not require a separate hand-rolled
grouping pass.

#### Scenario: Delegate reads from the existing cache

- **WHEN** `CatalogListDelegate::sections_count` is called while `grouped_cache` is
  populated
- **THEN** it returns the number of publisher groups without re-grouping the catalog

#### Scenario: Cache invalidation still triggers a re-group

- **WHEN** `LibraryChanged` fires
- **THEN** `grouped_cache` is invalidated and the next section lookup re-groups the
  catalog
