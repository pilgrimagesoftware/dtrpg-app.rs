## Requirements

### Requirement: HTTP 429 responses MUST be classified distinctly from generic network failures
Any service-layer error resulting from an HTTP 429 response SHALL be classified as `RateLimited`, not `Network`, in both `LibraryServiceErrorKind` and `CollectionsServiceErrorKind`.

#### Scenario: SDK-mediated request receives a 429
- **WHEN** an SDK-mediated library or collections request receives HTTP 429
- **THEN** the resulting service error's kind is `RateLimited`

#### Scenario: Direct file-download request receives a 429
- **WHEN** the file-download stream (bypassing the SDK) receives HTTP 429
- **THEN** the resulting service error's kind is `RateLimited`

#### Scenario: Direct cover-thumbnail fetch receives a 429
- **WHEN** the cover-thumbnail fetch (bypassing the SDK) receives HTTP 429
- **THEN** the resulting error is classified the same way as the other 429 cases (`RateLimited`), even though this call site's return type does not carry the full `LibraryServiceErrorKind` enum

### Requirement: A 429 response's Retry-After duration MUST be honored when present
When a `RateLimited` error carries a `retry_after: Some(duration)` value, the next retry attempt SHALL wait that duration rather than the computed exponential-backoff delay. When no `Retry-After` value is available, the existing exponential-backoff schedule SHALL apply unchanged.

#### Scenario: Retry-After header present
- **WHEN** a 429 response includes a `Retry-After: 30` header
- **THEN** the next retry attempt is scheduled 30 seconds later, not at the exponential-backoff-computed delay

#### Scenario: Retry-After header absent
- **WHEN** a 429 response includes no `Retry-After` header
- **THEN** the next retry attempt uses the existing exponential-backoff schedule, identical to a generic `Network` failure

#### Scenario: 429 remains retryable
- **WHEN** a request fails with a `RateLimited` error and attempts remain within the configured `RetryConfig.max_attempts`
- **THEN** the request is retried, matching the retryability a `Network`-kind failure already had before this change
