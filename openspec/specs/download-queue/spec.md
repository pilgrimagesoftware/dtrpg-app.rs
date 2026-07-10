# download-queue Specification

## Purpose
TBD - created by archiving change download-queue-concurrency-control. Update Purpose after archive.
## Requirements
### Requirement: Downloads MUST be dispatched through a bounded queue
The app MUST maintain a download queue that dispatches at most `max_concurrent_downloads` concurrent file downloads at any time. Requests that exceed the limit MUST wait in the queue until a slot opens.

#### Scenario: Enqueueing a download within capacity
- **WHEN** the user requests a download and fewer than `max_concurrent_downloads` downloads are active
- **THEN** the download starts immediately and an activity panel entry appears with the item title

#### Scenario: Enqueueing a download at capacity
- **WHEN** the user requests a download and `max_concurrent_downloads` slots are all occupied
- **THEN** the request is added to the queue and begins once a slot frees

#### Scenario: Download completes successfully
- **WHEN** a file download finishes without error
- **THEN** the item's status updates to Downloaded and its activity panel entry moves to the recent/complete state

#### Scenario: Download fails
- **WHEN** a file download encounters a network or storage error
- **THEN** the item's status reverts to Cloud, the activity panel entry shows an error message, and the slot is freed for the next queued item

### Requirement: Each download MUST have a named activity panel entry
The app MUST create one activity panel entry per enqueued download, identified by the item's title, and MUST update that entry when the download completes or fails.

#### Scenario: Activity entry lifecycle
- **WHEN** a download is enqueued
- **THEN** an activity entry is started with the item title as its label; it transitions to complete or error when the download finishes

### Requirement: Users MUST be able to cancel a queued or in-progress download
The app MUST allow a user to cancel a download that has not yet completed, removing it from the queue or aborting the in-flight fetch and freeing its concurrency slot.

#### Scenario: Cancelling a queued download
- **WHEN** the user cancels a download that is waiting in the queue
- **THEN** it is removed from the queue and no activity entry is created

#### Scenario: Cancelling an in-progress download
- **WHEN** the user cancels a download that is actively fetching
- **THEN** the fetch is aborted, any partial file is deleted, the slot is freed, and the activity entry is removed

