## 1. Error Types

- [x] 1.1 In `crates/dtrpg-ui/src/services/mod.rs`, add
      `LibraryServiceErrorKind::RateLimited`; add
      `retry_after: Option<std::time::Duration>` to `LibraryServiceError`;
      update `LibraryServiceError::new` (or add a constructor) so existing
      call sites default to `None`; add a `RateLimited` arm to
      `panel_detail`'s hint match
- [x] 1.2 In `crates/dtrpg-ui/src/services/collections.rs`, make the same
      additions to `CollectionsServiceErrorKind`/`CollectionsServiceError`

## 2. Retry Primitive

- [x] 2.1 In `crates/dtrpg-ui/src/services/retry.rs`, add an
      `extract_retry_after: impl Fn(&E) -> Option<Duration>` parameter to
      `retry_with_backoff`; when it returns `Some(d)` for the failed
      attempt, use `d` as the wait instead of `backoff_delay(...)`,
      otherwise unchanged
- [x] 2.2 Update all three existing `retry_with_backoff` call sites
      (catalog-sync totals request, cover/avatar image fetch, file
      download's `download_item_with_config`) to pass `|_| None` for the
      new parameter, preserving current behavior — a fourth call site
      (`data/avatar.rs`'s Gravatar avatar fetch, not enumerated in this
      design since it's not a DriveThruRPG API request) also needed
      updating to keep compiling; given `|_| None` since Gravatar 429
      handling is out of scope
- [x] 2.3 Update each call site's `is_retryable` closure to also treat
      `RateLimited` as retryable (matching how 429 was already retried
      under the old `Network` classification), e.g. `e.kind ==
      LibraryServiceErrorKind::Network || e.kind ==
      LibraryServiceErrorKind::RateLimited`

## 3. Raw-HTTP Paths (unblocked — no `dtrpg-sdk` dependency)

- [x] 3.1 In `crates/dtrpg-core/src/services/sdk/library/download.rs`'s
      `stream_to_file`, on a non-success status read
      `response.headers().get(reqwest::header::RETRY_AFTER)`, parse as
      delay-seconds, and return a `LibraryServiceError` with
      `kind: RateLimited` and the parsed `retry_after` when status is 429
      (keep the existing `Network` classification for other non-success
      statuses)
- [x] 3.2 In `crates/dtrpg-ui/src/controllers/library.rs`'s cover/avatar
      thumbnail fetch closure, apply the same treatment — check the
      response status before calling `.bytes()`, classify a 429
      distinctly, and pass the extracted `retry_after` through to
      `retry_with_backoff`'s new `extract_retry_after` parameter (this
      closure's error type is currently a bare `String`; widen it to carry
      the retry-after value, e.g. a small local struct or tuple, rather
      than parsing the string back out)
- [x] 3.3 Wire `download_item_with_config` and the cover-fetch call site's
      `extract_retry_after` closures to read the value now attached to
      their respective error types

## 4. SDK-Mediated Paths (blocked on `dtrpg-sdk` releasing
      `expose-retry-after-header`)

- [x] 4.1 Bump the `dtrpg-sdk` dependency version in the workspace
      `Cargo.toml` once `expose-retry-after-header` is released —
      `dtrpg-sdk = "1.0"` already permitted 1.1.0 under semver; ran
      `cargo update -p dtrpg-sdk` to bump `Cargo.lock` to 1.1.0
- [x] 4.2 In `crates/dtrpg-core/src/services/sdk/library/errors.rs`'s
      `map_client_error`, destructure `ClientError::ApiError`'s new
      `retry_after` field; when `status == 429`, return
      `LibraryServiceErrorKind::RateLimited` with that value instead of
      falling through to the generic `Network` branch
- [x] 4.3 Apply the same change to
      `crates/dtrpg-core/src/services/sdk/collections/errors.rs`'s
      `map_client_error`
- [x] 4.4 Wire the catalog-sync totals-request `retry_with_backoff` call
      site's `extract_retry_after` closure to read
      `LibraryServiceError::retry_after` — already done in task 2.3's
      commit, since the field existed on the app side before the SDK
      exposed it

## 5. Tests

- [x] 5.1 Unit test: `retry_with_backoff` waits the `extract_retry_after`-
      supplied duration instead of the computed backoff when present
- [x] 5.2 Unit test: `retry_with_backoff` falls back to computed backoff
      when `extract_retry_after` returns `None`
- [x] 5.3 Unit test: a `RateLimited` error is retried under each updated
      `is_retryable` closure, matching prior `Network` retry behavior —
      covered via `download_item_retries_a_rate_limited_response_like_a_network_failure`
      in `download.rs`
- [x] 5.4 Integration-style test (mocked HTTP, matching this repo's
      existing `download_item_*` test style in `download.rs`): a 429 with
      `Retry-After` on the download path produces a `RateLimited` error
      with the parsed duration
- [x] 5.5 Test (once task group 4 is unblocked): `map_client_error` maps a
      429 `ClientError::ApiError` with `retry_after: Some(_)` to
      `LibraryServiceErrorKind::RateLimited` carrying that value

## 6. Build and Quality

- [x] 6.1 `cargo check --workspace`
- [x] 6.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 6.3 `cargo test --workspace`
- [x] 6.4 `cargo +nightly fmt --all -- --check`

## 7. Manual Verification

- [x] 7.1 Confirm a simulated 429 (e.g. via a local mock server swapped
      into a debug build, or observed against a real rate-limited request)
      causes the app to wait the server-specified duration rather than the
      default backoff schedule, and that the request still eventually
      succeeds once retried
