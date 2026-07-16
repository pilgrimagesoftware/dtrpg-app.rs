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

- [ ] 5.1 Detect fresh install: no catalog cache file, no downloaded items, no cached cover or
      avatar content.
- [ ] 5.2 Gate fresh-install initialization on valid credentials being available; wait rather
      than request if credentials are not yet acquired.
- [ ] 5.3 Add a totals request (item count, size) issued before the first paginated item-data
      request, feeding the existing `on_total` progress callback.
- [ ] 5.4 Persist a "last request time" alongside existing catalog cache metadata; skip a new
      fresh-install request when the recorded time is within the minimum interval.
- [ ] 5.5 Update `catalog-load-progress`'s total-count resolution to prefer the fresh-install
      totals request over the existing `links.last`-derived estimate when both are available.

## 6. Cache-Control Staleness Signal and Recurring Timer

- [ ] 6.1 Extend the remote-fetch call in `catalog-auto-load-policy`'s staleness check to also
      evaluate a cache-control header or update query parameter from the SDK response.
- [ ] 6.2 Add a recurring long-running-session timer, independent of startup, that re-runs the
      staleness check and triggers a fetch through the catalog-sync serial dispatch path.

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
