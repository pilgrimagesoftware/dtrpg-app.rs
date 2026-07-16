## ADDED Requirements

### Requirement: Catalog synchronization runs on a serial dispatch path
`LibraryController` SHALL ensure that catalog update and synchronization tasks execute one at a
time, so that a second catalog-sync task does not begin while one is already in flight.

#### Scenario: A second catalog sync request is deferred
- **WHEN** a catalog synchronization task is already in flight and another is requested
- **THEN** `LibraryController` does not start the second task until the first completes

#### Scenario: Catalog sync completes and unblocks the next request
- **WHEN** an in-flight catalog synchronization task completes
- **THEN** `LibraryController` allows a subsequently requested catalog synchronization task to
  begin

### Requirement: Content downloads and catalog sync execute independently
A content download in progress SHALL NOT prevent a catalog synchronization task from running
concurrently, and the download queue's concurrency settings SHALL remain independent of the
catalog-sync dispatch path.

#### Scenario: Catalog sync proceeds while a download is in progress
- **WHEN** a content download is in progress on `download_queue`
- **THEN** a catalog synchronization task can still execute on its own serial dispatch path

### Requirement: Background work runs on gpui's background executor, never the UI thread
All catalog-sync, download, and cache-queue dispatch SHALL run through gpui's
`background_executor()`/`cx.spawn`, never blocking the UI thread.

#### Scenario: Background task executes while the UI remains responsive
- **WHEN** a catalog-sync, download, or cache task is dispatched
- **THEN** it runs on gpui's background executor and the UI thread remains responsive
