## Why

`dtrpg-app/openspec`'s `define-catalog-maintenance-behavior` change defines shared,
language-agnostic behavior for catalog and resource-cache maintenance — fresh-install
initialization, connectivity awareness, a background work-queue topology, retry/backoff, and
logging conventions — that this Rust app must implement using its own primitives. Today,
`LibraryController::start_load`/`start_load_inner` (`crates/dtrpg-ui/src/controllers/library.rs`)
runs a single cache-then-fetch sequence with no totals-first progress step, no last-request-time
gating, and no connectivity check; downloads and thumbnails share an informal `VecDeque`-based
concurrency budget with no equivalent queue for catalog sync; and retry/backoff
(`download-retry-with-backoff`) exists only for file transfers, not for catalog or image-cache
requests.

## What Changes

- Extend `start_load_inner`'s fetch sequence with a totals-first request (item count, size)
  reported through the existing `on_total`/`on_page` progress callbacks, and a persisted
  "last request time" that gates a redundant remote fetch on subsequent starts.
- Add a lightweight network-monitor module queried before any remote request that needs
  connectivity (general reachability and, where relevant, endpoint-specific reachability), with
  an optional broadcast channel for state-change events.
- Formalize a serial catalog-sync execution path alongside the existing `download_queue` and
  `thumbnail_queue`, so catalog/remote-sync work is ordered and never interleaves with itself,
  while the download and thumbnail queues keep their own bounded concurrency.
- Generalize the `backoff_delay`/`on_retry` shape introduced by `download-retry-with-backoff`
  into a shared retry helper usable by catalog-sync and image-cache requests, without changing
  download-transfer retry itself.
- Establish a `tracing` convention distinguishing internal diagnostic logging (`debug!`/`warn!`/
  `error!` with detail: endpoint, retry reason, status code) from user-facing surfaces (activity
  panel/toast text: retry number only, no internal detail), extending the existing default-`WARN`
  filter setup in `crates/dtrpg-core/src/logging.rs`.
- Extend `catalog-auto-load-policy`'s staleness check to also trigger on a cache-control/
  query-parameter signal from the SDK response, and add a recurring long-running-session timer
  trigger alongside the existing startup trigger.
- Implement the three caveat scenarios: re-run fresh-install initialization when the local
  catalog is empty or relocated; keep serving cached data with a non-intrusive re-auth banner
  when credentials are invalid but a local catalog exists; and drive the long-running-session
  timer independent of any startup event.

## Capabilities

### New Capabilities

- `rust-catalog-fresh-install-initialization`: totals-first request, paginated fetch with
  real-time view updates via the existing progress callbacks, and last-request-time persistence,
  implemented in `start_load_inner`.
- `rust-resource-network-monitor`: the connectivity-check module and its query/subscribe
  contract, consulted by catalog sync, downloads, and image caching before making a request.
- `rust-resource-work-queue-topology`: the serial catalog-sync execution path plus the existing
  `download_queue`/`thumbnail_queue`/`check_queue` reframed as the concurrent and serial queue
  set the shared spec requires.
- `rust-resource-error-handling-and-retry`: the shared backoff/retry helper generalized from
  `download-retry-with-backoff`'s `backoff_delay` for catalog-sync and image-cache requests.
- `rust-resource-logging-conventions`: the internal-vs-user-facing `tracing` logging split.

### Modified Capabilities

- `catalog-auto-load-policy`: staleness check gains a cache-control/query-parameter signal and a
  recurring long-running-session timer trigger, in addition to the existing startup trigger.
- `catalog-load-progress`: progress reporting extends to cover the totals-first request before
  paginated item fetch begins.

## Impact

- `crates/dtrpg-ui/src/controllers/library.rs`: `start_load`/`start_load_inner`, `download_queue`/
  `thumbnail_queue`/`check_queue` dispatch, and the recurring refresh timer.
- `crates/dtrpg-core/src/services/sdk/library/`: cache-control/update-signal handling in
  `list_items_paged`, and the generalized retry helper alongside `download.rs`'s
  `backoff_delay`.
- `crates/dtrpg-core/src/logging.rs`: internal-vs-user-facing logging convention.
- `crates/dtrpg-ui/src/data/`: last-request-time persistence, network-monitor state.
- New module for the network monitor (location decided in design.md).
- Builds on `download-retry-with-backoff` (reuses its backoff shape, does not change download
  retry itself) and on the existing `catalog-auto-load-policy`/`catalog-load-progress`/
  `catalog-disk-cache`/`catalog-live-data-swap` specs (extends rather than replaces their fetch
  sequence).
- Parent change: `dtrpg-app/openspec/changes/define-catalog-maintenance-behavior/proposal.md`.
