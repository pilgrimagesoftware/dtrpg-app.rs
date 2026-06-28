## ADDED Requirements

### Requirement: Cached catalog data SHALL remain visible during a live SDK fetch
While a live catalog fetch from the SDK is in progress, the application SHALL display the pre-populated cached catalog data in the UI without interruption. The catalog SHALL NOT be cleared or partially replaced before the complete live dataset is available.

#### Scenario: Cached data visible while SDK fetch is in progress
- **WHEN** the SDK fetch is in progress and cached data was pre-populated on startup
- **THEN** the UI continues to display the full cached catalog without any intermediate partially-loaded state

#### Scenario: First SDK page does not displace cached data
- **WHEN** the first live SDK page arrives during a fetch
- **THEN** the in-memory catalog still contains the cached data, not just the first page

### Requirement: Catalog SHALL be replaced atomically with the complete live dataset
Once all pages of a live SDK fetch have been received, the application SHALL replace the catalog in a single update, not incrementally page-by-page.

#### Scenario: Catalog is swapped once after all pages are received
- **WHEN** the SDK fetch channel closes after delivering all pages
- **THEN** the catalog is replaced in a single operation with the complete set of live items

#### Scenario: Cached catalog preserved on SDK fetch failure
- **WHEN** the SDK fetch fails at any point
- **THEN** the catalog retains the pre-populated cached data rather than a partial live result
