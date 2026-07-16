## Context

The app's concurrency model is gpui's `cx.spawn`/`background_executor()`, not raw `tokio::spawn`
or channel-based queues. `tokio` only exists as a per-connection bridge
(`Builder::new_multi_thread().enable_all().block_on(...)` in
`crates/dtrpg-core/src/services/sdk/connection.rs` and `services/login.rs`) so the synchronous
service layer can call the async `dtrpg-sdk` client. `LibraryController`
(`crates/dtrpg-ui/src/controllers/library.rs`) already holds three informal `VecDeque` queues —
`download_queue`, `thumbnail_queue` (sharing a `max_concurrent_downloads` slot budget), and
`check_queue` (serial) — drained by dispatch functions that spawn work on gpui's background
executor. Catalog fetch itself is a single `start_load`/`start_load_inner` sequence: disk cache
loads instantly, then a live SDK fetch via `list_items_paged` reports progress through
`on_page`/`on_total` callbacks, skippable per `catalog-auto-load-policy`, swapped in atomically
per `catalog-live-data-swap`. `download-retry-with-backoff` (in progress) already defines a pure
`backoff_delay(attempt, jitter_source)` helper and an `on_retry` callback for download-transfer
retries only. No network-connectivity monitor exists anywhere in the codebase.

## Goals / Non-Goals

**Goals:**
- Extend the existing `start_load_inner` cache-then-fetch sequence with a totals-first request
  and last-request-time gating, without introducing a parallel fetch path.
- Add a serial catalog-sync execution path that composes with the existing `download_queue`/
  `thumbnail_queue`/`check_queue`, using the same gpui-executor idiom rather than a new
  concurrency primitive (channels, thread pools).
- Add a network-monitor module with a pull-query API (mandatory) and an optional broadcast
  channel for push notifications, consulted by catalog sync, downloads, and image caching.
- Generalize `download-retry-with-backoff`'s `backoff_delay`/`on_retry` shape into a shared
  helper usable by catalog-sync and image-cache requests.
- Establish the internal-vs-user-facing `tracing` logging convention and apply it across
  catalog, download, cover-cache, and avatar-cache code paths.

**Non-Goals:**
- Changing download-transfer retry behavior itself (owned by `real-file-download-transfer`/
  `download-retry-with-backoff`); this change only lifts the same backoff shape to other
  resource kinds.
- Introducing a new async runtime, channel-based task queue, or thread-pool crate; the existing
  gpui-executor + `VecDeque` pattern is retained and extended, not replaced.
- Reconciliation/merge-by-identity semantics (owned by `catalog-remote-sync-reconciliation`) or
  the startup sequence itself (owned by `define-app-startup-workflow`'s Rust child change, not
  yet created).
- Recurring-timer behavior while the app is backgrounded or the machine is asleep — flagged as
  an open question in the parent spec; this change implements the timer for the foreground/
  active case only.

## Decisions

**The catalog-sync "serial queue" is a `bool`-gated single-slot dispatch on
`LibraryController`, not a new queue type.**
`download_queue`/`thumbnail_queue` are `VecDeque`s with a numeric concurrency budget
(`max_concurrent_downloads`). Catalog sync needs exactly one thing: never let a second sync task
start while one is in flight. A single `catalog_sync_in_flight: bool` (or an in-flight `Option`
holding the current sync's cancel handle) checked before `start_load_inner` dispatches, mirroring
the existing dispatch functions' style, achieves the "serial queue" requirement without a new
abstraction. Alternative considered: a generic `WorkQueue<T>` type shared by all four queues.
Rejected — the existing three queues have already diverged in slot-sharing rules
(`download_queue`/`thumbnail_queue` share a budget, `check_queue` doesn't), and a generic
abstraction would force them into a shape none of them naturally fit; matching the existing
per-queue style keeps the diff small and consistent with current code.

**The network monitor performs a lightweight, on-demand reachability check rather than
maintaining a persistent OS-level path monitor.**
The app has no OS-level connectivity API integration today (unlike Swift's `NWPathMonitor`).
Given the query-first contract is the required behavior and push notification is explicitly
optional in the parent spec, the monitor implements: a cheap "general connectivity" check (e.g.
a short-timeout TCP connect or DNS resolution to a well-known host) and a per-endpoint check
(reusing the existing `reqwest`/SDK client with a short timeout), both callable synchronously
from the calling queue's background-executor task, with results cached briefly to avoid
re-checking on every request. Push notifications are deferred (see Open Questions) since the
spec only requires the pull-query contract to be present.
Alternative considered: a background polling loop pushing state on every tick. Rejected for v1 —
adds a persistent background task and unbounded resource use for a capability the spec marks
optional; on-demand query satisfies every mandatory requirement.

**Retry helper lives in `dtrpg-ui`, not `dtrpg-core`, since `dtrpg-core` depends on `dtrpg-ui`
(not the reverse) and the primary callers — `LibraryController`'s catalog-sync dispatch and
cover/avatar image caching — are `dtrpg-ui` code.**
`crates/dtrpg-core/Cargo.toml` depends on `dtrpg-ui`; `dtrpg-ui` has no dependency back on
`dtrpg-core`. A shared helper usable by both catalog-sync (`dtrpg-ui`) and, eventually,
download-transfer retry (`dtrpg-core`, behind the `LibraryService` trait implemented there)
must live on the `dtrpg-ui` side of that boundary, the same side `LibraryServiceError` and the
`LibraryService` trait itself already live on. Corrected from an earlier draft of this design
that placed it in `dtrpg-core/src/services/retry.rs`, which would have made it unreachable from
`LibraryController`.
**`download-retry-with-backoff`'s `backoff_delay` did not exist yet at implementation time (0/22
tasks, no code)**, so it was built here from scratch per that change's own design.md algorithm —
exponential, base 2s, cap 30s, deterministic jitter, no `rand` dependency — generalized with
explicit `base_secs`/`max_secs` parameters so catalog-sync and image-cache can use their own
constants. `download-retry-with-backoff` will call this same `dtrpg_ui::services::retry` module
when it's implemented, rather than either change duplicating the backoff math.

**Logging convention: `tracing::debug!`/`warn!`/`error!` carry full internal detail (endpoint,
status code, retry reason); user-facing surfaces (activity panel, toast) receive only a
pre-formatted short string (operation name + retry number, no reason).**
This matches the parent spec's "retry number is user-facing, reason is not" requirement exactly
and requires no new logging crate — only a convention plus, where retry surfaces to the UI, a
small `RetryStatus { attempt: u32 }` type (no reason field) passed to the UI layer instead of the
full error.

**Last-request-time and network-monitor cache state persist through the existing
`crates/dtrpg-ui/src/data/` cache-file mechanism (`catalog_cache.json`-style JSON), not a new
storage backend.**
`catalog-disk-cache` already establishes the pattern of a JSON file under the app's storage
directory. Adding a `last_catalog_request_time` field (and, if needed, a short-lived
connectivity-check cache) to that same mechanism avoids introducing a new persistence layer.

## Risks / Trade-offs

- [On-demand network monitor may add latency to every gated request] → Mitigated by a short
  in-memory cache of the last check result (a few seconds), so a burst of requests (e.g. paginated
  fetch) triggers at most one reachability check.
- [Generalizing `backoff_delay` touches `download.rs`, a file with active in-progress work in
  `download-retry-with-backoff`] → Coordinate before implementation: confirm
  `download-retry-with-backoff`'s status and rebase this change's retry-module extraction onto
  its final shape rather than racing it.
- [`bool`/`Option`-based single-slot dispatch for catalog sync is less general than a queue] →
  Acceptable per the parent spec's requirement (serial execution, not queueing depth); revisit
  only if a second serial-catalog-sync producer emerges.
- [No push-notification network state] → Matches the parent spec's optional framing; if a future
  child need arises (e.g. auto-retry on reconnect), add a broadcast channel then rather than now.

## Migration Plan

No data migration — this adds new fields to existing cache files (backward compatible with
`serde` defaults) and new code paths gated behind the existing fetch sequence. Rollout is a
normal feature PR; no feature flag needed since behavior is additive (fresh-install and
long-running-session paths that don't exist today) or narrowly extends existing staleness checks
(cache-control signal is an additional trigger condition, not a replacement).

## Open Questions

- Should the network monitor eventually push connectivity-change events (parent spec marks this
  optional)? Deferred until a concrete consumer needs it.
- Recurring-timer behavior while backgrounded/asleep is explicitly deferred to whichever child
  change (Rust or Swift) implements it first, per the parent spec's own open question; this
  change implements the foreground/active case only and will feed back into the parent spec if
  the answer turns out to be cross-platform.
