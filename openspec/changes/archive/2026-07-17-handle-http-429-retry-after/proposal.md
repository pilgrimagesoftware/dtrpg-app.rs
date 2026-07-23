## Why

HTTP 429 (Too Many Requests) responses are currently indistinguishable from any other transient failure: both `LibraryServiceErrorKind` and `CollectionsServiceErrorKind` classify a 429 as generic `Network`, and `retry_with_backoff` retries it on the same fixed exponential-backoff schedule used for connection failures, DNS errors, and every other retryable condition. This risks retrying sooner than the server wants and prolonging the rate-limit window instead of resolving it. This is the app-side half of a coordinated umbrella change (`dtrpg/openspec/changes/handle-http-429-retry-after`); the SDK-mediated half depends on `dtrpg-sdk/rust/openspec/changes/expose-retry-after-header` shipping first, but the raw-HTTP download/thumbnail paths in this repo (which never go through the SDK) can honor `Retry-After` independently of that dependency.

## What Changes

- `LibraryServiceErrorKind` and `CollectionsServiceErrorKind` gain a `RateLimited` variant, distinct from `Network`.
- `LibraryServiceError` and `CollectionsServiceError` gain a `retry_after: Option<std::time::Duration>` field.
- `crates/dtrpg-core/src/services/sdk/library/errors.rs` and
  `.../collections/errors.rs` map HTTP 429 to `RateLimited`, reading
  `ClientError::ApiError`'s new `retry_after` field (once
  `expose-retry-after-header` ships in `dtrpg-sdk`) rather than falling
  through to the generic `Network` branch.
- The two raw-HTTP call sites that bypass the SDK entirely — the cover
  thumbnail fetch (`controllers/library.rs`) and the file-download stream
  (`crates/dtrpg-core/src/services/sdk/library/download.rs`'s
  `stream_to_file`) — read the `Retry-After` header directly from the
  `reqwest::blocking::Response` on a 429 and surface it the same way.
- `retry::retry_with_backoff` gains a way for a caller to supply a
  server-directed wait duration per error, used instead of the computed
  exponential-backoff delay when present, falling back to the existing
  schedule otherwise.

## Capabilities

### New Capabilities

- `rate-limit-retry-after`: The desktop app classifies HTTP 429 responses distinctly from other network failures and, when the server specifies a `Retry-After` duration, waits that duration instead of the default exponential backoff before retrying.

### Modified Capabilities

_(none — `thumbnail-queue-concurrency` and `download-queue` reference retry/concurrency behavior in terms of `max_concurrent_downloads` and queue dispatch, neither of which this change alters; this is purely a new, additive retry-timing behavior layered on the existing retry mechanism.)_

## Impact

- `crates/dtrpg-ui/src/services/mod.rs`: `LibraryServiceErrorKind::RateLimited`
  added; `LibraryServiceError` gains `retry_after`.
- `crates/dtrpg-ui/src/services/collections.rs`: same additions for
  `CollectionsServiceErrorKind`/`CollectionsServiceError`.
- `crates/dtrpg-ui/src/services/retry.rs`: `retry_with_backoff` (or a
  sibling function) gains a way to honor a per-error server-directed delay.
- `crates/dtrpg-core/src/services/sdk/library/errors.rs`,
  `.../collections/errors.rs`: map 429 to `RateLimited` + the SDK's
  `retry_after` value (depends on `dtrpg-sdk` upgrade from
  `expose-retry-after-header`).
- `crates/dtrpg-core/src/services/sdk/library/download.rs`: `stream_to_file`
  reads `Retry-After` on a 429 from the raw `reqwest::blocking::Response`.
- `crates/dtrpg-ui/src/controllers/library.rs`: the cover-thumbnail fetch
  closure reads `Retry-After` the same way; both this and the download path
  changes are independent of the SDK dependency.
- `Cargo.toml` (workspace): `dtrpg-sdk` version bump once
  `expose-retry-after-header` is released, required for the
  SDK-error-mapping half of this change.
