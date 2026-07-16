## Why

"Catalog > Reload" currently forces a full live catalog fetch unconditionally, bypassing the
7-day cache-freshness policy that governs every other catalog load path. The only guard against
repeated invocation is a 60-second anti-double-click throttle (`FORCE_RELOAD_COOLDOWN_SECS`),
which does nothing to stop a user from deliberately hammering the API by invoking Reload every
minute. The intent of gating Reload was always to prevent abuse of the API, not merely to absorb
an accidental double press — the manual action should defer to the same freshness policy the app
already trusts for passive loads.

## What Changes

- **BREAKING**: "Catalog > Reload" no longer unconditionally forces a full live paginated fetch.
  It instead re-runs the same freshness check `catalog-auto-load-policy` already applies to
  passive/startup loads (cache non-empty, saved within 7 days, remote count matches) and only
  performs a live fetch when that check determines one is actually needed.
- The existing 60-second `FORCE_RELOAD_COOLDOWN_SECS` double-click throttle is retained
  unchanged — it still absorbs an accidental repeated invocation within the same interaction,
  independent of the freshness policy now also gating the underlying fetch.
- When the freshness policy determines the cache is still valid, "Reload" performs the same
  lightweight remote count check the passive policy uses (not a full paginated fetch), then
  completes without changing the catalog — mirroring the existing skip-fetch behavior at
  startup and on the recurring long-running-session timer.

## Capabilities

### New Capabilities

*(none)*

### Modified Capabilities

- `catalog-menu`: The "Catalog menu contains Reload action" requirement changes from
  "unconditionally triggers a full live catalog fetch" to "invokes the catalog auto-load
  freshness policy, performing a live fetch only when the policy determines one is needed."
- `catalog-auto-load-policy`: Adds a requirement that user-initiated reload (the "Catalog >
  Reload" menu action / `cmd-r`) is subject to the same freshness policy as passive and
  timer-triggered loads, rather than being a separate unconditional-fetch code path.

## Impact

- `dtrpg-ui/src/controllers/library.rs`: `LibraryController::reload_catalog` currently calls
  `start_load_inner(cx, true)` (force bypass); changes to call `start_load_inner(cx, false)` so
  the auto-load policy's freshness/count checks apply, while `reload_cooldown_active`'s
  60-second throttle guard stays as-is ahead of that call.
- `dtrpg-ui/src/data/constants.rs`: No changes expected; `FORCE_RELOAD_COOLDOWN_SECS` remains
  the double-click throttle, distinct from the 7-day freshness window it now works alongside.
- No API, SDK, or settings panel changes.
