# rust-resource-error-handling-and-retry Specification

## Purpose
TBD - created by archiving change implement-catalog-maintenance-behavior. Update Purpose after archive.
## Requirements
### Requirement: Catalog and cache errors produce a user-facing message and a verbose internal log
When an error occurs during a catalog-sync, cover-cache, or avatar-cache operation, the app
SHALL display a clear, concise, user-facing error message and separately log a more verbose
internal-only message describing the error.

#### Scenario: An error occurs during a catalog or cache operation
- **WHEN** a catalog-sync, cover-cache, or avatar-cache operation fails
- **THEN** the app displays a clear, concise user-facing error message
- **AND** logs a verbose internal-only message describing the error via `tracing`

### Requirement: Catalog and cache operations retry transient failures with backoff
Catalog-sync, cover-cache, and avatar-cache operations SHALL reuse the retry-with-backoff
helper generalized from `download-retry-with-backoff`'s `backoff_delay`, retrying a transient
failure up to a fixed attempt limit with a backoff delay between attempts.

#### Scenario: A transient failure is retried
- **WHEN** a catalog-sync or image-cache operation fails with a transient error and the retry
  limit has not been reached
- **THEN** the app retries the operation after a backoff delay computed by the shared
  `backoff_delay` helper

#### Scenario: The retry limit is reached
- **WHEN** a catalog-sync or image-cache operation has failed and been retried up to its limit
- **THEN** the app stops retrying and treats the operation as failed

### Requirement: Retry attempts are logged with attempt number and reason
Each retry attempt for a catalog-sync or image-cache operation SHALL be logged via `tracing`
with the attempt number and the reason for the retry.

#### Scenario: A retry attempt is logged
- **WHEN** the app retries a catalog-sync or image-cache operation
- **THEN** it logs the retry attempt number and reason at an internal log level

### Requirement: Retry number may be shown to the user; retry reason is internal-only
The app MAY include the current retry attempt number in a user-facing progress display for a
retrying catalog-sync or image-cache operation, but SHALL NOT include the retry reason in that
display.

#### Scenario: Retry number is shown during a retrying operation
- **WHEN** the app displays progress for a retrying catalog-sync or image-cache operation
- **THEN** it may include the current retry number
- **AND** it does not include the internal retry reason
