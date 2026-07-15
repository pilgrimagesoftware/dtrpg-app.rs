# thumbnail-loading-queue Specification

## Purpose
TBD - created by archiving change catalog-thumbnail-loading. Update Purpose after archive.
## Requirements
### Requirement: Auto-enqueue thumbnails on catalog page arrival
When a catalog page is appended, every item whose `cover_url` is `Some` AND whose cover is not already present in `CoverCache` AND is not already in-flight SHALL be added to the thumbnail loading queue.

#### Scenario: New items with cover URLs are enqueued
- **WHEN** `append_catalog_page` is called with items that have `cover_url` set
- **THEN** each such item whose cover is not cached and not in-flight is added to the queue

#### Scenario: Already-cached items are skipped
- **WHEN** `append_catalog_page` is called and `CoverCache` already contains the cover for an item
- **THEN** that item is NOT added to the queue

#### Scenario: In-flight items are skipped
- **WHEN** `append_catalog_page` is called and an item's cover is already marked in-flight in `CoverCache`
- **THEN** that item is NOT added to the queue

### Requirement: Sequential single-item processing
The thumbnail queue SHALL process exactly one item at a time. The next fetch SHALL NOT begin until the current fetch completes (success or failure).

#### Scenario: Queue drains one item at a time
- **WHEN** the queue contains multiple items
- **THEN** each item is fetched sequentially, with the next fetch starting only after the previous one finishes

#### Scenario: Fetch completion triggers next item
- **WHEN** a thumbnail fetch completes (success or failure)
- **THEN** the next item in the queue is dequeued and fetched

### Requirement: Successful fetch stored in CoverCache
When a thumbnail HTTP request returns a successful response with image data, the bytes SHALL be stored in `CoverCache` keyed by the item's `cover_url`, the in-flight marker SHALL be cleared, and the catalog view SHALL be re-rendered.

#### Scenario: HTTP 200 response stores cover
- **WHEN** a thumbnail fetch returns HTTP 200 with image bytes
- **THEN** the bytes are inserted into `CoverCache` and the catalog refreshes

#### Scenario: Failed fetch clears in-flight marker
- **WHEN** a thumbnail fetch fails (network error or non-200 response)
- **THEN** the in-flight marker is cleared and the queue continues to the next item

### Requirement: Activity panel tracking
While the thumbnail queue is non-empty, the activity panel SHALL show a single aggregated item. The label SHALL reflect the number of remaining covers. When the queue empties, the activity item SHALL be marked complete.

#### Scenario: Activity item appears when queue starts
- **WHEN** the first item is added to a previously empty queue
- **THEN** a new activity entry is created with a label indicating covers are loading

#### Scenario: Activity label updates as queue drains
- **WHEN** a thumbnail fetch completes and the queue still has items
- **THEN** the activity entry's label is updated to reflect the new remaining count

#### Scenario: Activity item completes when queue empties
- **WHEN** the last item in the queue has been fetched
- **THEN** the activity entry is marked complete

### Requirement: Thumbnail fetches run without requiring an ambient async runtime
The thumbnail HTTP fetch SHALL execute in a way that does not depend on an active Tokio (or other) async reactor being present on the executing thread, since `dtrpg-ui` does not own or depend on one and gpui's executors are not Tokio-backed.

#### Scenario: Fetch succeeds without a Tokio runtime in scope
- **WHEN** a thumbnail fetch runs on a `gpui` background-executor thread with no Tokio runtime entered anywhere in the process
- **THEN** the fetch completes successfully (using a mechanism, such as `reqwest`'s blocking client, that manages its own runtime internally rather than requiring one)

### Requirement: Manual trigger via context menu
Every catalog entry in all three layouts (list, thumbs, grid) SHALL expose a "Load Thumbnail" context menu item that enqueues the entry's thumbnail for loading.

#### Scenario: Context menu enqueues item
- **WHEN** the user right-clicks a catalog entry and selects "Load Thumbnail"
- **THEN** the entry's `cover_url` is added to the front of the queue and loading begins if not already active

