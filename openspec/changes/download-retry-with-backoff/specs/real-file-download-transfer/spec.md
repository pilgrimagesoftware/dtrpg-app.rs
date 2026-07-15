## MODIFIED Requirements

### Requirement: A dispatched download MUST fetch and write real file bytes
`LibraryService::download_item` MUST resolve a download URL for the requested item and write the fetched bytes to the destination path, so that `Downloaded` status only reflects a file that actually exists on disk. A transient failure MUST NOT immediately fail the download while retry attempts remain — see the retry requirement below.

#### Scenario: Successful transfer
- **WHEN** a download is dispatched for an item and the fetch completes without error
- **THEN** a file exists at the item's resolved on-disk path containing the fetched bytes

#### Scenario: Transfer succeeds after a retry
- **WHEN** a download's first attempt fails with a retryable error and a subsequent retry attempt completes without error
- **THEN** a file exists at the item's resolved on-disk path containing the fetched bytes, and the download is reported as successful

## ADDED Requirements

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
