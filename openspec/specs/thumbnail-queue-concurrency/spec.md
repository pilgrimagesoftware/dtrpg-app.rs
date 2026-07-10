# thumbnail-queue-concurrency Specification

## Purpose
TBD - created by archiving change download-queue-concurrency-control. Update Purpose after archive.
## Requirements
### Requirement: Thumbnail loading MUST run concurrently up to the configured limit
The app MUST load thumbnails concurrently, dispatching up to `max_concurrent_downloads` parallel HTTP fetches at a time, rather than serially one at a time.

#### Scenario: Multiple thumbnails visible in grid
- **WHEN** the grid view shows more thumbnails than a single fetch can resolve before the next render
- **THEN** up to `max_concurrent_downloads` thumbnail fetches run in parallel, reducing total time to fill the visible grid

#### Scenario: Slot freed after thumbnail fetch
- **WHEN** a thumbnail fetch completes (success or error)
- **THEN** the next pending thumbnail in the queue is dispatched immediately without waiting for other slots

### Requirement: Thumbnail and download concurrency MUST share the same limit
The number of concurrent thumbnail fetches plus concurrent file downloads MUST NOT exceed `max_concurrent_downloads` in aggregate, so that thumbnail loading and active downloads do not collectively saturate the connection.

#### Scenario: Downloading while thumbnails are loading
- **WHEN** a file download is in progress and grid thumbnails are being fetched
- **THEN** the combined number of active fetches stays at or below `max_concurrent_downloads`

### Requirement: Priority thumbnails MUST preempt lower-priority ones
When the user scrolls to a new grid region or selects an item, the app MUST promote that item's thumbnail to the front of the thumbnail queue.

#### Scenario: Promoting a thumbnail on item selection
- **WHEN** the user selects an item whose thumbnail is pending in the queue
- **THEN** that thumbnail moves to the front of the queue and is dispatched as soon as a slot is available

