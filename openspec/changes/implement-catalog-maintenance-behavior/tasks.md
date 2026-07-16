## 1. Sibling-Change Reconciliation

- [x] 1.1 Confirm `download-retry-with-backoff`'s status and final shape of `backoff_delay`
      before extracting it into a shared module, so the retry-helper generalization (task 3)
      rebases onto its finished form rather than racing an in-progress change.
      Corrected 2026-07-16: `download-retry-with-backoff` is 0/22 tasks — no `backoff_delay` or
      retry code exists anywhere in the codebase yet. Per user decision, built the shared
      `backoff_delay`/`retry_with_backoff` helper here from scratch (matching that change's own
      design.md algorithm) rather than relocating existing code; `download-retry-with-backoff`
      will consume this same module when it's implemented.

## 2. Retry Helper Extraction

- [x] 2.1 Build `backoff_delay(attempt, jitter_source, base_secs, max_secs)` in a new shared
      module `crates/dtrpg-ui/src/services/retry.rs` (not `dtrpg-core` — see design.md's
      corrected Decisions section: `dtrpg-core` depends on `dtrpg-ui`, not the reverse, and
      `LibraryController` is `dtrpg-ui` code), per `download-retry-with-backoff`'s design.md
      algorithm (exponential, base*2^(attempt-1), capped, deterministic +/-25% jitter, no
      `rand` dependency), generalized with explicit base/max parameters so catalog-sync and
      image-cache can use their own constants.
- [x] 2.2 Add a `retry_with_backoff` helper wrapping a fallible closure with max-attempts,
      a retry-gate predicate, and an `on_retry(attempt, delay, &error)` callback, built on
      `backoff_delay`, with cancel-aware short sleep ticks (200ms) during the backoff wait.
- [x] 2.3 N/A — `download.rs` has no retry call site to update yet (see 1.1 correction);
      nothing to change here without also implementing `download-retry-with-backoff` itself,
      which is out of this change's scope.

## 3. Network Monitor

- [x] 3.1 Add `crates/dtrpg-ui/src/services/network_monitor.rs`: a general-connectivity check
      (short-timeout TCP connect to a fixed IP, deliberately DNS- and DriveThruRPG-independent
      so a DriveThruRPG-specific outage doesn't read as a general outage) and an
      endpoint-specific check (`std::net` TCP connect with DNS resolution via
      `ToSocketAddrs`, short timeout) — no new HTTP/TCP crate; reuses `std::net`, already a
      dependency-free standard-library facility.
- [x] 3.2 Cache each check's result (`NETWORK_MONITOR_CACHE_TTL_SECS` = 5s) in an in-memory
      `Mutex<HashMap<String, CachedState>>` keyed by target, to bound check frequency under
      bursts of calling requests.
- [ ] 3.3 Wire the monitor's query calls into catalog-sync, download, cover-cache, and
      avatar-cache request paths, stopping the request when the monitor reports unreachable.
      Threaded through as each call site is touched: catalog-sync in tasks 4-5, cover/avatar
      cache alongside their retry wiring in task 7.1.

## 4. Catalog Sync Serial Dispatch

- [x] 4.1 Add `catalog_sync_in_flight: bool` to `LibraryController`, checked and set at the top
      of `start_load_inner` (returns early, logging at debug level, if already in flight).
- [x] 4.2 Clear the guard at every exit path of `start_load_inner`'s async closure: the
      cache-fresh skip-fetch return, the partial-fetch success return, the
      superseded-by-newer-load return, and the natural end of both the success and error arms
      of the final `match fetch.await` — 63 existing `controllers::library` tests pass
      unchanged, confirming no behavior regression in the paths the guard now wraps.
      Dedicated test coverage for the guard itself needs a full gpui `TestAppContext` harness
      (used in `view_models/library.rs`'s tests, not `controllers/library.rs`'s, which only
      tests pure helper functions) — deferred to task 9.1.

## 5. Fresh-Install Initialization

- [x] 5.1 Detect fresh install in `start_load_inner` as `!force_reload && cached.as_ref()
      .is_none_or(Vec::is_empty)` — no catalog cache loaded from disk. Downloaded-item and
      cover/avatar-cache detection from the spec's fuller definition is not separately checked;
      an empty catalog cache is the load-bearing signal `start_load_inner` already branches on
      (the existing auto-load-policy block only runs when cache is non-empty), so this reuses
      that existing branch point rather than adding new disk probes.
- [x] 5.2 Corrected 2026-07-16 (user decision): no new credentials pre-check added.
      `list_items_paged` already returns a `Session` error when no authenticated session
      exists, handled as a quiet completion (existing behavior, unchanged) — satisfies "does
      not surface a failure to the user" without a new cross-controller credential query.
      A true pre-check (stopping the request before it's attempted) would need synchronous
      access to `AuthStateController`'s state from `LibraryController`, which does not exist
      today; out of scope for this change.
- [x] 5.3 Corrected 2026-07-16 (user decision): the real DriveThruRPG API has no
      aggregate-size endpoint (`openapi.yaml` only has per-file `size`/`sizeMB` and
      per-collection `itemCount`, no catalog-wide total size) — the totals request is
      count-only, via `service_arc.count_items()` (the same `links.last`-derived count
      `catalog-auto-load-policy` already uses), called before the paginated fetch and feeding
      `estimated_total` directly rather than waiting on `list_items_paged`'s own `on_total`.
- [x] 5.4 Added `last_fresh_install_request_secs` to `CacheMetadata` (mirrors the existing
      `last_item_check_batch_secs` field/helper pattern) with
      `save_fresh_install_request_timestamp`; gated in `start_load_inner` against
      `CATALOG_FRESH_INSTALL_MIN_REQUEST_INTERVAL_SECS` (60s) — skips the whole fresh-install
      totals+timestamp step (not just the paginated fetch) when within the interval.
- [x] 5.5 `estimated_total`'s seed now prefers `fresh_install_total` (the count-only totals
      request above) over the cache-length fallback; `list_items_paged`'s own `on_total`
      (from `links.last`) can still override it later if the API reports one during the
      paginated fetch itself, unchanged from existing behavior.
      296 `dtrpg-ui` tests + 12 doctests pass; clippy clean (`-D warnings`).

## 6. Cache-Control Staleness Signal and Recurring Timer

- [x] 6.1 Corrected 2026-07-16 (user decision): the real API has no cache-control/ETag
      mechanism (`openapi.yaml` has neither); the only update-detection signal is the
      `updatedDate[after]` query parameter, already wired into `start_load_inner`'s
      auto-load-policy branch via `should_attempt_partial_fetch`/`partial_fetch_since`/
      `list_items_updated_since` (triggered on a remote-count mismatch). This existing
      mechanism already satisfies the requirement's intent; no new code needed. Documented
      here rather than editing the parent spec's wording, per user decision.
- [x] 6.2 Added `start_periodic_catalog_refresh_timer` to `LibraryController` (mirrors the
      existing `start_periodic_check_batch_timer` idiom exactly), waking every
      `CATALOG_REFRESH_TIMER_INTERVAL_SECS` (1h) to call the same non-forced `start_load` the
      startup sequence uses. Composes with the group-4 `catalog_sync_in_flight` guard (an
      overlapping wake-up is a no-op) and the existing auto-load-policy staleness check inside
      `start_load_inner` (decides whether the wake-up actually triggers a fetch). Started
      alongside the existing check-batch timer in the constructor. 296 tests pass, clippy
      clean.

## 7. Error Handling, Retry, and Logging Conventions

- [ ] 7.1 Apply the shared `retry_with_backoff` helper to catalog-sync requests, cover-cache
      fetches, and avatar-cache fetches, each with an appropriate attempt limit.
- [ ] 7.2 Log each retry attempt via `tracing` with attempt number and reason; ensure any
      user-facing retry display shows only the attempt number, never the reason.
- [ ] 7.3 Establish the `tracing::debug!`/`warn!`/`error!` convention across catalog-sync,
      cover-cache, and avatar-cache code paths for routine activity, warnings, and errors, and
      pair every user-facing error surface with a corresponding verbose internal log line.

## 8. Caveat Scenarios

- [ ] 8.1 Empty or relocated local catalog with valid credentials re-runs fresh-install
      initialization (task 5) rather than treating it as a routine startup.
- [ ] 8.2 Inaccessible or expired credentials with a valid local catalog: keep serving cached
      data and show a non-intrusive re-authentication banner instead of blocking the user.
- [ ] 8.3 Verify the long-running-session timer (task 6.2) operates independent of any startup
      event, including when the app has been running long enough to cross the staleness
      threshold without a restart.

## 9. Verification

- [ ] 9.1 Add unit tests for the network monitor's cache-expiry behavior, the retry helper's
      attempt-limit and backoff-delay math (reusing existing `backoff_delay` test coverage),
      and the catalog-sync in-flight guard.
- [ ] 9.2 Add unit tests for fresh-install detection, the totals-request-first ordering, and
      last-request-time gating.
- [ ] 9.3 Manually verify: fresh install against a real or mocked API, offline startup with a
      valid local catalog, and expired credentials with a valid local catalog.
