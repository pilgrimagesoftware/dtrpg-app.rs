## 1. Constants

- [x] 1.1 In `crates/dtrpg-core/src/constants.rs`, add
      `MAX_DOWNLOAD_ATTEMPTS: u32 = 4`,
      `DOWNLOAD_RETRY_BASE_DELAY_SECS: u64 = 2`,
      `DOWNLOAD_RETRY_MAX_DELAY_SECS: u64 = 30`

## 2. Backoff Calculation

- [x] 2.1 Reuse the existing shared helper instead of adding a new one:
      `crates/dtrpg-ui/src/services/retry.rs` already provides
      `backoff_delay(attempt, jitter_source, base_secs, max_secs) ->
      Duration` and `retry_with_backoff(config, cancel, operation,
      is_retryable, on_retry) -> Result<T, E>` (landed in
      `catalog-maintenance-behavior`, explicitly intended for download
      transfers). No new backoff function is added in `download.rs`.

## 3. Retry Loop and Signature Change

- [x] 3.1 In `crates/dtrpg-ui/src/services/mod.rs`, add an optional
      `on_retry: Option<&mut dyn FnMut(u32, std::time::Duration)>`
      parameter to `LibraryService::download_item`'s trait signature
      (attempt number about to retry, delay before it)
- [x] 3.2 In `crates/dtrpg-core/src/services/sdk/library/download.rs`, wrap
      `download_item`'s existing single-attempt body in a call to
      `dtrpg_ui::services::retry::retry_with_backoff` with a `RetryConfig`
      built from `MAX_DOWNLOAD_ATTEMPTS`, `DOWNLOAD_RETRY_BASE_DELAY_SECS`,
      `DOWNLOAD_RETRY_MAX_DELAY_SECS`; `is_retryable` returns `true` only
      for `kind == LibraryServiceErrorKind::Network`; adapt the trait-level
      `on_retry: Option<&mut dyn FnMut(u32, Duration)>` to `retry.rs`'s
      `OnRetry<'_, E>` shape by discarding the error argument
- [x] 3.3 Update `mod.rs`'s `download_item` impl to forward the new
      parameter to `download::download_item`
- [x] 3.4 In `crates/dtrpg-ui/src/services/stub.rs`, update the stub's
      `download_item` signature to match (parameter unused/prefixed `_`)

## 4. UI Wiring

- [x] 4.1 In `dispatch_download` (`crates/dtrpg-ui/src/controllers/library.rs`),
      pass a callback that updates the download's activity label with
      retry progress (e.g. "Retrying (n/N) in Ns…"), reusing the existing
      `weak_activity`/`activity_id` label-update mechanism

## 5. Tests

- [x] 5.1 (Covered by existing `retry.rs` tests — no new `backoff_delay`
      tests needed here.)
- [x] 5.4 Unit/integration test: a `download_item` call whose first N-1
      attempts fail with a retryable error and whose final attempt
      succeeds returns `Ok(())` and writes the file
- [x] 5.5 Unit/integration test: a `download_item` call whose every attempt
      fails with a retryable error returns the last failure after exactly
      `MAX_DOWNLOAD_ATTEMPTS` attempts
- [x] 5.6 Unit/integration test: a non-retryable error (e.g. session) from
      `prepare_download` fails immediately with no retry
- [x] 5.7 Unit/integration test: cancelling during a backoff wait stops
      retries and leaves no partial or final file at the destination
- [x] 5.8 Confirm existing `download.rs` tests (`part_path_for` cases)
      still pass unmodified

## 6. Build and Quality

- [x] 6.1 `cargo check --workspace`
- [x] 6.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 6.3 `cargo test --workspace`
- [x] 6.4 `cargo +nightly fmt --all -- --check`

## 7. Manual Verification

- [ ] 7.1 Simulate a transient failure (e.g. disconnect networking briefly
      during a download) and confirm the app retries and eventually
      succeeds once connectivity returns, with the activity label showing
      retry progress
- [ ] 7.2 Cancel a download while it's waiting between retry attempts and
      confirm it stops immediately with no partial file left behind
