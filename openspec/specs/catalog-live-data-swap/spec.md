# catalog-live-data-swap Specification

## Purpose
TBD - created by archiving change catalog-live-merge. Update Purpose after archive.
## Requirements

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

### Requirement: Per-file downloaded state SHALL survive reconcile against a live fetch
When reconciling the existing catalog against a live SDK fetch, a catalog
item present in both SHALL keep the `downloaded` flag of any of its files
that also appears (matched by file id) in the live item's file list, and the
item's aggregate status SHALL be recomputed from the merged file list rather
than taken from the live fetch.

#### Scenario: A downloaded item's status survives a restart's live fetch
- **WHEN** the cached catalog contains an item with `status: Downloaded` and
  all its files marked `downloaded: true`, and a live fetch on startup
  returns the same item (with the API's default `downloaded: false` on every
  file)
- **THEN** after reconcile the item's files retain `downloaded: true` and its
  status remains `Downloaded`

#### Scenario: A live file with no cached counterpart is not downloaded
- **WHEN** the live fetch returns a file id for an item that was not present
  in the cached item's file list
- **THEN** that file's `downloaded` flag is `false` after reconcile

#### Scenario: A partially-downloaded item keeps its per-file state
- **WHEN** the cached item has one file marked `downloaded: true` and one
  marked `downloaded: false`, and the live fetch returns both file ids
- **THEN** after reconcile the first file is still `downloaded: true`, the
  second is still `downloaded: false`, and the item's status is `Cloud`
