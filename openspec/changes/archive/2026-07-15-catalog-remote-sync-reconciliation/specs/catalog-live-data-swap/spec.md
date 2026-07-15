## MODIFIED Requirements

### Requirement: Catalog SHALL be updated atomically once all live pages are received

Once all pages of a live SDK fetch have been received, the application SHALL apply the
result to the catalog in a single update, not incrementally page-by-page. When the catalog
was empty before the fetch started, the update SHALL be a full replace with the live
dataset. When the catalog already contained local data before the fetch started, the update
SHALL be a reconciliation of the live dataset against the local catalog by item id (see
`catalog-availability-flag`) rather than a wholesale replace.

#### Scenario: Empty catalog is replaced once after all pages are received
- **WHEN** the catalog was empty before the fetch started and the SDK fetch channel closes
  after delivering all pages
- **THEN** the catalog is replaced in a single operation with the complete set of live items

#### Scenario: Non-empty catalog is reconciled once after all pages are received
- **WHEN** the catalog already contained local data before the fetch started and the SDK
  fetch channel closes after delivering all pages
- **THEN** the catalog is updated in a single operation that reconciles the live items
  against the existing local items by id, rather than replacing the catalog wholesale

#### Scenario: Cached catalog preserved on SDK fetch failure
- **WHEN** the SDK fetch fails at any point
- **THEN** the catalog retains the pre-populated cached data rather than a partial live
  result, and no reconciliation is applied

## ADDED Requirements

### Requirement: An empty catalog SHALL populate incrementally as live pages arrive

The application SHALL append each page's items to the visible catalog as that page arrives,
rather than waiting for the entire fetch to complete before showing anything to the user,
whenever a live SDK fetch starts with no local catalog data present (no cache to
pre-populate from, or an empty catalog).

#### Scenario: First page appears before the fetch completes
- **WHEN** a live SDK fetch starts with an empty catalog and the first page of items arrives
- **THEN** those items are visible in the catalog UI immediately, without waiting for
  subsequent pages or the fetch to finish

#### Scenario: Each subsequent page extends the visible catalog
- **WHEN** a live SDK fetch with an empty starting catalog delivers a second or later page
- **THEN** that page's items are appended to the already-visible catalog from prior pages
