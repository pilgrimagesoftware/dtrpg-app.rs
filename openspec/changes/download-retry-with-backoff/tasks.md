## 1. Constants

- [ ] 1.1 In `crates/dtrpg-core/src/constants.rs`, add
      `MAX_DOWNLOAD_ATTEMPTS: u32 = 4`,
      `DOWNLOAD_RETRY_BASE_DELAY_SECS: u64 = 2`,
      `DOWNLOAD_RETRY_MAX_DELAY_SECS: u64 = 30`

## 2. Backoff Calculation

- [ ] 2.1 In `crates/dtrpg-core/src/services/sdk/library/download.rs`, add a
      pure `fn backoff_delay(attempt: u32, jitter_source: u64) -> Duration`
      computing `base * 2^(attempt - 1)` capped at the max delay, with
      jitter derived deterministically from `jitter_source`
- [ ] 2.2 At the production call site, pass
      `SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_nanos() as u64`
      as the jitter source

## 3. Retry Loop and Signature Change

- [ ] 3.1 In `crates/dtrpg-ui/src/services/mod.rs`, add an optional
      `on_retry: Option<&mut dyn FnMut(u32, std::time::Duration)>`
      parameter to `LibraryService::download_item`'s trait signature
      (attempt number about to retry, delay before it)
- [ ] 3.2 In `crates/dtrpg-core/src/services/sdk/library/download.rs`, wrap
      `download_item`'s existing single-attempt body in a loop up to
      `MAX_DOWNLOAD_ATTEMPTS`: on a retryable failure
      (`kind == LibraryServiceErrorKind::Network` and not a cancellation),
      call `on_retry` if present, then wait via short cancel-checked sleep
      ticks before the next attempt; on a non-retryable failure or
      cancellation, return immediately; on exhausting all attempts, return
      the last failure
- [ ] 3.3 Update `mod.rs`'s `download_item` impl to forward the new
      parameter to `download::download_item`
- [ ] 3.4 In `crates/dtrpg-ui/src/services/stub.rs`, update the stub's
      `download_item` signature to match (parameter unused/prefixed `_`)

## 4. UI Wiring

- [ ] 4.1 In `dispatch_download` (`crates/dtrpg-ui/src/controllers/library.rs`),
      pass a callback that updates the download's activity label with
      retry progress (e.g. "Retrying (n/N) in Ns…"), reusing the existing
      `weak_activity`/`activity_id` label-update mechanism

## 5. Tests

- [ ] 5.1 Unit test: `backoff_delay` returns increasing delays across
      consecutive attempt numbers
- [ ] 5.2 Unit test: `backoff_delay` is capped at
      `DOWNLOAD_RETRY_MAX_DELAY_SECS` for a large attempt number
- [ ] 5.3 Unit test: `backoff_delay` is deterministic for a fixed
      `jitter_source` (same inputs produce the same output)
- [ ] 5.4 Unit/integration test: a `download_item` call whose first N-1
      attempts fail with a retryable error and whose final attempt
      succeeds returns `Ok(())` and writes the file
- [ ] 5.5 Unit/integration test: a `download_item` call whose every attempt
      fails with a retryable error returns the last failure after exactly
      `MAX_DOWNLOAD_ATTEMPTS` attempts
- [ ] 5.6 Unit/integration test: a non-retryable error (e.g. session) from
      `prepare_download` fails immediately with no retry
- [ ] 5.7 Unit/integration test: cancelling during a backoff wait stops
      retries and leaves no partial or final file at the destination
- [ ] 5.8 Confirm existing `download.rs` tests (`part_path_for` cases)
      still pass unmodified

## 6. Build and Quality

- [ ] 6.1 `cargo check --workspace`
- [ ] 6.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 6.3 `cargo test --workspace`
- [ ] 6.4 `cargo +nightly fmt --all -- --check`

## 7. Manual Verification

- [ ] 7.1 Simulate a transient failure (e.g. disconnect networking briefly
      during a download) and confirm the app retries and eventually
      succeeds once connectivity returns, with the activity label showing
      retry progress
- [ ] 7.2 Cancel a download while it's waiting between retry attempts and
      confirm it stops immediately with no partial file left behind
