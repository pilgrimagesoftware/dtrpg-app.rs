## Context

Retry behavior in this app runs through one shared primitive,
`retry::retry_with_backoff` (`crates/dtrpg-ui/src/services/retry.rs`), used
by three call sites: catalog sync (`LibraryServiceError`-typed, SDK-mediated,
`controllers/library.rs`), cover/avatar image fetches (raw `String`-typed,
non-SDK, `controllers/library.rs`), and file downloads
(`LibraryServiceError`-typed, non-SDK for the actual transfer,
`crates/dtrpg-core/.../download.rs`). All three retry on the same
`backoff_delay`-computed schedule regardless of *why* the attempt failed.

Two of the three failure sources can, on a 429, tell the caller exactly how
long to wait: the SDK-mediated path once `dtrpg-sdk`'s
`ClientError::ApiError.retry_after` ships (a separate, sequenced dependency —
see `dtrpg/openspec/changes/handle-http-429-retry-after`'s Rollout Order),
and the two raw-`reqwest` paths, which can read the `Retry-After` response
header directly today without waiting on anything.

## Goals / Non-Goals

**Goals:**
- Classify HTTP 429 distinctly (`RateLimited`) from generic `Network`
  failures in both `LibraryServiceErrorKind` and `CollectionsServiceErrorKind`.
- Carry a `retry_after: Option<Duration>` value on the error type from
  whichever layer detected the 429, through to `retry_with_backoff`.
- Make `retry_with_backoff` honor that value when present, falling back to
  its existing computed backoff otherwise — every existing call site keeps
  working unchanged if it never supplies a value.
- Implement the two raw-`reqwest` paths (thumbnail fetch, file download)
  now, independent of the `dtrpg-sdk` version bump.
- Implement the SDK-mediated error-mapping paths in the same change, gated
  behind the `dtrpg-sdk` dependency bump (this task cannot start until
  `expose-retry-after-header` ships, per the umbrella's Rollout Order, but
  both halves belong in one child proposal since they share the same
  `retry_with_backoff` extension and error-type additions).

**Non-Goals:**
- Capping how long the app will wait on a server-specified `Retry-After`
  value that's implausibly large (e.g. an hour). If DriveThruRPG ever sends
  an unreasonable value, that's an API-behavior problem the client
  shouldn't silently override — worth revisiting if observed in practice.
- Changing `max_concurrent_downloads` or any queue-dispatch behavior — 429
  handling is about retry *timing* for an individual failed request, not
  concurrency.
- Surfacing rate-limit state in the UI (e.g. a "rate limited, waiting Ns"
  activity-panel message) — the existing `on_retry` callback already logs
  and could be extended to do this, but no requirement calls for it and
  it's a separate, presentation-layer concern.

## Decisions

- **Add `retry_after: Option<Duration>` as a field on `LibraryServiceError`/
  `CollectionsServiceError`, not as data on the `RateLimited` enum variant.**
  Consistent with how `message: String` already lives on the struct rather
  than on each `LibraryServiceErrorKind` variant — keeps the kind enum a
  plain classification tag (still cheap `Copy`), and every existing
  exhaustive match on the kind enum (e.g. `panel_detail`'s hint lookup)
  needs only one new arm, not a signature change.
- **`retry_with_backoff` gains an `extract_retry_after: impl Fn(&E) ->
  Option<Duration>` parameter** (analogous to the existing `is_retryable`
  predicate), called once per failed attempt; when it returns `Some(d)`,
  `d` is used as the wait instead of `backoff_delay(...)`, still subject to
  the same cancellation-aware `wait_cancelable` loop. Existing call sites
  pass `|_| None` to preserve today's behavior unchanged — this is an
  additive signature change, not a breaking one, since Rust closures make
  the extra parameter a simple `|_| None` one-liner at each existing site.
- **The two raw-`reqwest` paths read `Retry-After` inline, independent of
  the SDK.** `reqwest::blocking::Response::headers()` is available
  identically whether the request went through the SDK or not; there is no
  reason to block this half of the change on the `dtrpg-sdk` version bump.
- **`RateLimited` is retryable by the same predicate that already treats
  `Network` as retryable**, i.e. `is_retryable` closures at each call site
  need `e.kind == LibraryServiceErrorKind::Network || e.kind ==
  LibraryServiceErrorKind::RateLimited` (or equivalent) rather than a bare
  `Network` check, since 429 was previously folded into `Network` and thus
  already retried — this change must not silently make 429 non-retryable
  by mistake when the kind check becomes more specific.

## Risks / Trade-offs

- [The `dtrpg-sdk` half of this change is blocked until
  `expose-retry-after-header` releases] → Mitigation: split tasks into "raw
  HTTP paths" (unblocked, do first) and "SDK-mediated paths" (blocked),
  matching this design's task breakdown, so the change can land iteratively
  rather than as one all-or-nothing PR.
- [A malformed or absent `Retry-After` value silently falls back to
  exponential backoff] → Accepted: this is the same "fail safe to existing
  behavior" property the SDK design already established, and matches this
  app's existing tolerance for degraded-but-functional retry behavior
  elsewhere (e.g. `partial_fetch_since` falling back gracefully when
  timestamps are absent).
- [`retry_with_backoff`'s new parameter adds a fourth closure to an already
  four-parameter-plus-generics function] → Accepted: `RetryConfig`,
  `is_retryable`, and `on_retry` are already separate parameters for
  separable concerns; adding `extract_retry_after` keeps that one-concern-
  per-parameter shape rather than folding it into an existing closure's
  responsibility.
