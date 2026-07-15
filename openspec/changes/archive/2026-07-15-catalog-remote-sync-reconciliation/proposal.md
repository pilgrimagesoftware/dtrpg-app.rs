## Why

`catalog-live-data-swap` replaces the entire in-memory catalog with whatever the server
returns on every load, once cached data already exists. If DriveThruRPG's API is flaky, rate
limited, paginates incompletely, or a title is briefly delisted, the local catalog silently
loses items the user still owns. A local catalog that already exists should be reconciled
against the server response, not overwritten by it — new items get added, items no longer
returned by the server get flagged instead of deleted, and previously-flagged items that
reappear get un-flagged.

This behavior is not Rust-specific: `dtrpg-app/openspec/changes/catalog-remote-sync-reconciliation`
defines it as a shared, language-neutral capability
(`shared-catalog-remote-sync-reconciliation`) that both the Rust and Swift frontends must
satisfy. This change is the Rust-specific child implementation of that shared capability —
the reconciliation model, availability flag, load-trigger cooldowns, item-level checks, and
startup fetch-strategy decision below are all constrained by the parent's outcome-oriented
scenarios; the concrete data structures, field names, constants, and gpui-specific mechanics
are this repository's own implementation choices.

## What Changes

- Add an `is_available` flag (default `true`) to `LibraryItem`, persisted in the disk cache.
- Define three distinct triggers for a catalog load and the reconciliation behavior each
  one uses:
  1. **No local data** (first launch, or after an explicit `clear_and_reload`): a full live
     fetch runs, and items are appended to the visible catalog incrementally as each page
     arrives — the user is never blocked on the entire fetch completing before seeing
     anything. No reconciliation is needed since there is nothing to reconcile against.
  2. **User-requested full reload** (the "Catalog > Reload" menu action): if the last
     recorded reload was not too recent, a full live fetch runs and the result is
     reconciled against the existing local catalog by id (same reconciliation as case 3).
     If the last reload was too recent, the request is a no-op — no network fetch, no
     change to the catalog.
  3. **Local data exists, routine load** (startup with a pre-populated cache, or a
     stale-cache auto-refresh): the live dataset is reconciled against the local catalog by
     item id instead of replacing wholesale:
     - Items present in the live response but not locally are added.
     - Items present in the live response and locally have their fields refreshed from the
       live data, and `is_available` is set to `true` if it was previously `false`.
     - Items present locally but absent from the live response are kept, with
       `is_available` set to `false` rather than removed.
- Surface unavailable items in the catalog UI so users can distinguish "not returned by the
  server" from a normal entry (visual treatment is an implementation detail for design.md).
- **BREAKING**: the on-disk catalog cache schema gains a required reconciliation-relevant
  field (`is_available`); old cache files remain loadable via a `serde` default of `true`.
- Add item-level reconciliation, independent of a full catalog load:
  - Opening a catalog entry's details (single-click popover or expanded detail tab) checks
    that one item against the server if it hasn't been checked recently, with a visible
    "checking" indicator on the entry while the check is in flight.
  - A background queue works through catalog items over time, re-checking each one against
    the server individually, with a visible indicator in the catalog view for whichever
    entry is currently being checked. The user can manually request a check pass; both the
    manual trigger and any automatic/periodic enqueueing are gated by cooldowns so neither
    the user nor a background timer can flood the server with per-item requests.
  - A single-item check can only ever set `is_available` — it never adds or removes items
    from the catalog.
- Add a startup count-based decision between three fetch strategies: skip the fetch
  (existing behavior, cache is fresh and count matches), a partial date-filtered fetch when
  the remote count suggests only additions occurred (merged in additively, without touching
  `is_available` on items it doesn't return), or a full paginated fetch + reconciliation
  when the count mismatch can't be explained by pure growth.

## Capabilities

### New Capabilities

- `catalog-availability-flag`: `LibraryItem` gains a persisted `is_available` flag and the
  data model/UI surface for representing an item the server no longer lists; also covers
  the effect of item-level checks and partial fetches on that flag.
- `catalog-item-level-reconciliation`: on-demand single-item checks triggered by viewing an
  entry's details, a background queue of periodic per-item checks with a manual trigger,
  cooldowns on both manual and automatic enqueueing, and catalog-view indicators for
  in-flight checks.

### Modified Capabilities

- `catalog-live-data-swap`: replaces the "replace atomically" requirement with a
  reconcile-by-id merge whenever local data already exists, keeping the "no partial state
  visible mid-fetch" and "cached data preserved on fetch failure" guarantees but changing
  what happens to items missing from the live response; also formalizes the existing
  incremental-population behavior for the no-local-data case as a spec requirement.
- `catalog-auto-load-policy`: adds a cooldown gate on user-requested full reloads, distinct
  from the existing passive 7-day cache-staleness check, so repeated manual reload requests
  don't each trigger a new full live fetch; also adds a startup count-based choice between
  skipping the fetch, a partial date-filtered fetch, or a full reconciliation fetch.

## Impact

- `crates/dtrpg-ui/src/data/library.rs` — `LibraryItem` struct gains `is_available` and
  `availability_last_checked` (not persisted, like `thumbnail_last_attempted`).
- `crates/dtrpg-ui/src/controllers/library.rs` — `set_catalog` reconciliation logic,
  replacing the unconditional `self.catalog = items` swap; `append_catalog_page` unaffected
  (still used only for the empty-catalog incremental path); `reload_catalog` gains a
  cooldown check against the existing cache metadata timestamp; `select_item`/detail-tab
  open trigger a single-item check; a new check queue mirroring the existing
  `thumbnail_queue`/`drain_thumbnail_queue` pattern.
- `crates/dtrpg-ui/src/data/constants.rs` — new constants for the manual-reload cooldown,
  per-item re-check cooldown, and periodic-check-batch cooldown.
- `crates/dtrpg-ui/src/data/catalog_cache.rs` — `CacheMetadata` gains an optional
  last-item-check-batch timestamp so the periodic-check cooldown survives app restarts.
- `crates/dtrpg-ui/src/services/mod.rs` (`LibraryService` trait) — new optional
  `list_items_updated_since` method for the partial date-filtered fetch, following the same
  `Option`-return "unsupported" convention as `count_items`.
- Disk cache format (`save_catalog_cache` / load path) — additive field, backward
  compatible via `#[serde(default)]`.
- Catalog list/detail views that render `LibraryItem` — need a way to show unavailable and
  checking-in-progress status (exact treatment decided in design.md).
