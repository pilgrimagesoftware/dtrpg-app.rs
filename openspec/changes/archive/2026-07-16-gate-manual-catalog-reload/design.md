## Context

`LibraryController::reload_catalog` (`dtrpg-ui/src/controllers/library.rs`) backs "Catalog >
Reload" (`cmd-r`). It checks `reload_cooldown_active` (a 60-second throttle keyed off
`CacheMetadata::saved_at_secs`, via `FORCE_RELOAD_COOLDOWN_SECS`) and, if not throttled, calls
`start_load_inner(cx, true)` — `force_reload = true` unconditionally skips the auto-load
policy's freshness/count checks (`catalog-auto-load-policy`) and always runs a full paginated
live fetch. Passive loads (startup, the recurring long-running-session timer) call
`start_load_inner(cx, false)`, which applies that policy: skip the live fetch when the cache is
non-empty, was saved within 7 days, and a lightweight remote count check matches; otherwise
fetch.

## Goals / Non-Goals

**Goals:**

- Make "Catalog > Reload" subject to the same freshness policy as every other load path, so
  repeated manual invocations cost at most a lightweight count check, not a full paginated
  fetch, once the cache is confirmed fresh.
- Preserve the existing 60-second double-click throttle unchanged.

**Non-Goals:**

- Removing or altering `FORCE_RELOAD_COOLDOWN_SECS` / `reload_cooldown_active`.
- Adding a separate "true force reload, bypass everything" action — this change removes the
  only code path that did that; if a genuine bypass is wanted later, it's a new, explicit
  proposal, not a side effect of this one.
- Changing `clear_and_reload` (used after an explicit cache-clear settings action), which
  legitimately needs an unconditional fetch since the disk cache itself was just deleted —
  `start_load_inner`'s freshness check already falls through to a full fetch on an empty cache
  regardless of `force_reload`, so no change is needed there.

## Decisions

### `reload_catalog` calls `start_load_inner(cx, false)` instead of `true`

Single-line change. With `force_reload = false`, `start_load_inner` runs its existing
freshness/count-match logic unmodified — the same logic already exercised by startup and the
periodic timer. No new gating logic needs to be written; this change is purely about which
existing code path Reload feeds into.

**Alternative considered**: keep `force_reload = true` but add a separate, shorter cooldown
gate in front of the live fetch specifically for manual reload. Rejected — that duplicates
policy that `catalog-auto-load-policy` already owns, and risks the two policies drifting apart
(e.g. a future change to the 7-day window not being reflected in a second, hand-rolled reload
gate).

### `catalog_loading = true` before the call stays as-is

`reload_catalog` sets `self.catalog_loading = true` and emits `LibraryChanged` before calling
`start_load_inner`, so the toolbar shows a loading indicator immediately regardless of which
branch `start_load_inner` ultimately takes. The skip-fetch branch already resets
`catalog_loading = false` on its own completion (existing behavior, unrelated to this change),
so a Reload that turns out to be a no-op still clears the indicator promptly rather than
leaving it spinning.

## Risks / Trade-offs

- **A user who genuinely wants to force a live check now sees no fetch at all when the cache is
  already fresh.** This is the intended behavior change (see proposal's Why) — the 7-day/count
  check already trusts the cache in that state everywhere else in the app. → Mitigation: none
  needed; this is the point of the change. If it proves too conservative in practice, the
  freshness window itself (`catalog-auto-load-policy`) is the place to tune, not a Reload-only
  carve-out.
- **BREAKING for anyone relying on Reload always hitting the network** (e.g. to confirm
  connectivity or force-sync after an out-of-band library change on another device). →
  Mitigation: none in this change; flagged as a known trade-off in the proposal.
