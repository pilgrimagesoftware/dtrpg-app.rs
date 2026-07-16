# real-file-download-transfer Specification

## Purpose
TBD - created by archiving change implement-real-file-downloads. Update Purpose after archive.
## Requirements
### Requirement: A dispatched download MUST fetch and write real file bytes
`LibraryService::download_item` MUST resolve a download URL for the requested item and write the fetched bytes to the destination path, so that `Downloaded` status only reflects a file that actually exists on disk. A transient failure MUST NOT immediately fail the download while retry attempts remain — see the retry requirement below.

#### Scenario: Successful transfer
- **WHEN** a download is dispatched for an item and the fetch completes without error
- **THEN** a file exists at the item's resolved on-disk path containing the fetched bytes

#### Scenario: Transfer succeeds after a retry
- **WHEN** a download's first attempt fails with a retryable error and a subsequent retry attempt completes without error
- **THEN** a file exists at the item's resolved on-disk path containing the fetched bytes, and the download is reported as successful

### Requirement: A failed download transfer SHALL retry with exponential backoff before finally failing
When a download attempt fails with a retryable error, the system SHALL wait for an exponentially increasing backoff delay (with jitter) and then retry the transfer from scratch, up to a fixed maximum number of attempts, before reporting a final failure.

#### Scenario: A transient failure triggers a retry
- **WHEN** a download attempt fails with a network/transfer error and the maximum attempt count has not been reached
- **THEN** the system waits a backoff delay and starts a new attempt, re-resolving the download URL from scratch

#### Scenario: Backoff delay increases across attempts
- **WHEN** consecutive attempts fail
- **THEN** each subsequent retry's backoff delay is longer than the previous one, up to a fixed maximum delay

#### Scenario: Exhausting all attempts reports a final failure
- **WHEN** every attempt up to the maximum fails
- **THEN** the download reports a failure to the caller after the last attempt, with no further retry

#### Scenario: A non-retryable error does not retry
- **WHEN** a download attempt fails with an error that is not a network/transfer error (e.g. a session or not-found error)
- **THEN** the system does not retry and reports the failure immediately

#### Scenario: Cancellation during a backoff wait stops retries immediately
- **WHEN** the download is cancelled while waiting between retry attempts
- **THEN** the system does not start another attempt and reports the transfer as cancelled

#### Scenario: Cancellation during a backoff wait leaves no partial data
- **WHEN** the download is cancelled while waiting between retry attempts
- **THEN** no partial or final file remains at the destination path, per the existing cancellation-cleanup requirement

### Requirement: An in-progress transfer MUST NOT leave a partial file at the final path
The transfer MUST write to a temporary `.part` path and only place the file at its final destination after the full transfer succeeds.

#### Scenario: Transfer fails partway through
- **WHEN** a download's network fetch fails after only some bytes have been received
- **THEN** no file exists at the final destination path; at most a `.part` file remains

#### Scenario: Transfer completes successfully
- **WHEN** a download's fetch completes in full
- **THEN** the final destination path contains the complete file and no `.part` file remains

### Requirement: Cancelling an in-progress transfer MUST remove any partial data
When a download is cancelled while its transfer is in progress, no partial or final file MUST remain at the destination path.

#### Scenario: Cancelling mid-transfer
- **WHEN** the user cancels a download while bytes are still being fetched
- **THEN** the transfer stops, any `.part` file is deleted, and no file exists at the final destination path

### Requirement: A successful download SHALL persist the catalog to the on-disk cache immediately
`LibraryController` SHALL write the updated catalog to the on-disk cache as
soon as a dispatched download completes successfully, rather than waiting for
the next live fetch to run `save_catalog_cache`.

#### Scenario: Downloaded status survives a restart without a live fetch
- **WHEN** a download completes successfully and the app is quit and
  relaunched before the on-disk cache would otherwise be rewritten (e.g. the
  cache is still within its freshness window and the auto-load policy skips
  the live fetch)
- **THEN** the item is loaded from the on-disk cache with `status:
  Downloaded` and its downloaded file's `downloaded` flag set to `true`

#### Scenario: A cancelled download does not trigger a cache write
- **WHEN** a dispatched download is cancelled before completion
- **THEN** `save_catalog_cache` is not called as a result of that download
