## Why

`download_item` (`crates/dtrpg-core/src/services/sdk/library/download.rs`)
makes exactly one attempt: `gateway.prepare_download`, then a single
`reqwest::blocking::get` streamed to a `.part` file. Any transient failure —
a dropped connection, a momentary DNS blip, a 5xx from the object storage
backend, a timeout on a large file — fails the whole download immediately
and surfaces an error to the user, who has to notice and manually retry via
the UI. Flaky networks and large PDF/ZIP files make transient mid-transfer
failures common enough that this is a real, recurring friction point, not
an edge case.

## What Changes

- `download_item` retries a failed transfer up to a fixed number of times
  before giving up, with an exponential backoff delay (with jitter) between
  attempts, instead of failing on the first error.
- Only genuinely retryable failures trigger a retry: network/transfer
  errors (`LibraryServiceErrorKind::Network`) excluding an in-progress
  cancellation, which always stops immediately regardless of retry budget.
- The backoff wait itself remains responsive to cancellation — it's
  composed of short sleep ticks checking the cancel flag, not one long
  blocking sleep, matching the existing per-chunk cancellation
  responsiveness in the transfer loop.
- The download activity's UI label reflects an in-progress retry (e.g.
  "Retrying (2/3) in 4s...") via an optional retry-progress callback on
  `LibraryService::download_item`, following the same callback-parameter
  pattern `list_items_paged` already uses for page/total progress.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `real-file-download-transfer`: A dispatched download now retries a
  retryable failure with exponential backoff before finally failing, rather
  than failing on the first error; cancellation still stops the transfer
  immediately including during a backoff wait.

## Impact

- `crates/dtrpg-core/src/services/sdk/library/download.rs`: `download_item`
  gains a retry loop, a pure backoff-delay calculation, and an optional
  retry-progress callback parameter.
- `crates/dtrpg-ui/src/services/mod.rs`: `LibraryService::download_item`'s
  trait signature gains the optional retry-progress callback parameter.
- `crates/dtrpg-ui/src/services/stub.rs`: stub implementation updated to
  match the new signature (no-op for the callback; stub downloads don't
  fail transiently).
- `crates/dtrpg-ui/src/controllers/library.rs`: `dispatch_download` passes a
  callback that updates the download's activity label with retry progress.
