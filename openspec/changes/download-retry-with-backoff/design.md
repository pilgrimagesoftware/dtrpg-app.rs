## Context

`download_item` (`crates/dtrpg-core/src/services/sdk/library/download.rs`)
is a single-attempt, synchronous, blocking function: `prepare_download`
resolves a short-lived pre-signed URL, then `stream_to_file` streams the
response body to a `.part` file in `CHUNK_SIZE` (64 KB) increments,
checking `cancel: &AtomicBool` between each chunk, and finally renames the
`.part` file to its final destination. Every error path — a failed
`prepare_download` call, a failed `reqwest::blocking::get`, a non-success
HTTP status, a read or write failure mid-stream, or an explicit
cancellation — currently returns immediately as a `LibraryServiceError`
with `kind: Network` (cancellation included, via the message `"download
cancelled"` — there's no distinct `Cancelled` kind).

The caller, `dispatch_download`
(`crates/dtrpg-ui/src/controllers/library.rs`), runs this blocking call on
`async_cx.background_executor()`, owns the `cancel_flag: Arc<AtomicBool>`
for this download, and already has an `activity_id` it updates via
`weak_activity.update(async_cx, |a, cx| a.update_label(activity_id, ..., cx))`
— the same mechanism `start_load_inner` uses to show "page N…" progress
during a catalog fetch, and the same shape `list_items_paged`'s
`on_page`/`on_total` callback parameters already use to report incremental
progress across the SDK-trait boundary.

## Goals / Non-Goals

**Goals:**
- Retry a failed download transfer automatically, with exponential backoff,
  before surfacing a final failure to the user.
- Never retry past an explicit cancellation — a user who cancels a download
  must see it stop immediately, not continue retrying in the background.
- Keep the backoff wait interruptible at short intervals, so cancelling
  during a backoff pause is as responsive as cancelling mid-transfer
  already is.
- Surface retry-in-progress state to the UI (activity label), reusing the
  existing callback-parameter idiom rather than inventing a new mechanism.

**Non-Goals:**
- Distinguishing HTTP status codes into finer-grained retryable/non-retryable
  categories (e.g. retry 503 but not 404). `download.rs` currently
  classifies nearly every failure as `LibraryServiceErrorKind::Network`;
  refining that classification is a separate, pre-existing limitation this
  change doesn't attempt to fix. The retry gate is "was this a `Network`
  failure and was it not a cancellation," matching what the error kind
  already distinguishes today.
- Retrying `prepare_download` separately from the transfer itself with a
  different policy — both share one retry loop and one backoff schedule,
  since a `prepare_download` failure and a transfer failure are both just
  "this attempt didn't work, try again."
- Persisting retry state across app restarts, or resuming a partial `.part`
  file across attempts (each retry re-downloads from scratch) — the
  existing `real-file-download-transfer` contract already guarantees no
  partial file survives a failed attempt, so there's nothing to resume from.
- Making the retry count or backoff schedule user-configurable via
  Settings. Fixed constants match how every other cooldown/threshold in
  this codebase (`ITEM_CHECK_COOLDOWN_SECS`, `STALE_SECS`, etc.) is a fixed
  constant, not a setting.

## Decisions

- **Retry loop lives inside `dtrpg-core`'s `download_item`, not in
  `dispatch_download`.** The trait contract
  (`LibraryService::download_item`: "Returns error on any failure") is
  unchanged; retrying is an implementation detail of how the real
  SDK-backed adapter fulfills that contract. The stub implementation
  doesn't need retry logic at all (it never fails transiently), which
  would be awkward to express if retry orchestration lived above the trait
  boundary in the shared UI controller.
- **A pure, testable backoff function**: `fn backoff_delay(attempt: u32,
  jitter_source: u64) -> Duration`, exponential (`base * 2^(attempt - 1)`,
  attempt starting at `1` for the delay before the first retry), capped at
  a max delay, with jitter applied deterministically from `jitter_source`
  rather than an internal RNG call. Production calls it with
  `SystemTime::now()`'s sub-second nanoseconds as the jitter source; unit
  tests pass fixed values and assert exact output. This avoids adding a new
  `rand`-family direct dependency (it's only ever a transitive dependency
  today) for what only needs "enough variance to avoid synchronized retries
  across concurrent downloads," not cryptographic randomness.
- **Constants**: `MAX_DOWNLOAD_ATTEMPTS = 4` (1 initial + 3 retries),
  `DOWNLOAD_RETRY_BASE_DELAY_SECS = 2`, `DOWNLOAD_RETRY_MAX_DELAY_SECS =
  30`, added to `crates/dtrpg-core/src/constants.rs` alongside this crate's
  other fixed thresholds.
- **Retry gate**: retry only when the failed attempt's error has
  `kind == LibraryServiceErrorKind::Network` AND `!cancel.load(Ordering::SeqCst)`
  at the moment of failure. A `Session`/`NotFound`/`NeedsReauth` error from
  `prepare_download` never retries (retrying won't change an auth or
  not-found outcome); an explicit cancellation never retries regardless of
  its (currently mislabeled) `Network` kind.
- **Backoff wait composed of short sleep ticks** (e.g. 200ms), checking
  `cancel` between ticks and returning immediately (without retrying
  further) if cancellation is observed — mirroring `stream_to_file`'s
  existing per-chunk cancellation check, so cancelling during a backoff
  pause is no less responsive than cancelling mid-transfer.
- **Optional retry-progress callback**: `download_item` gains an
  `on_retry: Option<&mut dyn FnMut(u32, Duration)>` parameter (attempt
  number about to be retried, delay before it), called once per retry
  right before the backoff wait starts — the same `Option<&mut dyn
  FnMut(...)>` shape `list_items_paged` already uses for `on_page`/`on_total`,
  so this isn't a new pattern in the codebase. `dispatch_download` passes a
  closure that updates the activity label (e.g. "Retrying (2/3) in 4s…");
  the stub implementation ignores the parameter (`_on_retry`).
- **Each retry re-attempts the full transfer from scratch**, including a
  fresh `prepare_download` call — the pre-signed URL a prior attempt
  resolved may have expired (observed ~30s expiry per `download.rs`'s
  existing comment), so a retry cannot safely reuse it.

## Risks / Trade-offs

- [Retrying a large file's full transfer from scratch on every attempt
  wastes bandwidth for a failure that occurs near the end of a big
  download] → Accepted: resumable/ranged downloads would require object
  storage backend support (`Range` requests) this codebase doesn't
  currently use anywhere, and is out of scope for "retry with backoff";
  the existing `.part`-file contract already discards partial data on any
  failure, retryable or not.
- [A download that's genuinely doomed (e.g. persistent server outage) now
  takes up to `2 + 4 + 8 = 14` extra seconds (plus jitter) before finally
  failing, instead of failing immediately] → Bounded by
  `MAX_DOWNLOAD_ATTEMPTS` and `DOWNLOAD_RETRY_MAX_DELAY_SECS`; the activity
  label's retry-progress callback keeps the user informed of what's
  happening during that window rather than looking stuck.
- [The retry-progress callback changes `LibraryService::download_item`'s
  trait signature] → Both current implementors (`dtrpg-core`'s SDK-backed
  service and `dtrpg-ui`'s stub) are in this same workspace and get updated
  in the same change; there's no external consumer of this trait to break.
