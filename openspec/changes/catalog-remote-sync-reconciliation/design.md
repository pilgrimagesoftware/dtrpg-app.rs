## Context

`LibraryController`'s live load path (`crates/dtrpg-ui/src/controllers/library.rs`) already
distinguishes "catalog was empty" from "catalog was pre-populated from disk cache"
(`catalog_was_empty`, set before the fetch loop starts). The empty case streams pages
straight into the visible catalog via `append_catalog_page`; the non-empty case buffers
pages into a local `live_items: Vec<LibraryItem>` and, per `catalog-live-data-swap`, performs
one atomic `ctrl.set_catalog(live_items, cx)` once the fetch completes, unconditionally
overwriting `self.catalog`.

`LibraryItem` (`crates/dtrpg-ui/src/data/library.rs`) has a stable `id: Arc<str>` used
elsewhere for identity (e.g. `dedupe_files` keys on `(id, name)`). No availability concept
exists today; `ItemStatus` (`Downloaded` / `Cloud`) tracks local download state, not server
presence.

`catalog-auto-load-policy` already governs *whether* a live fetch runs at all for a passive
load: `start_load_inner`'s auto-load check skips the fetch when the cache is non-empty,
`CacheMetadata::is_stale()` reports fresh (written within 7 days, matching schema version),
and a cheap remote count call matches the cached count. `reload_catalog` (the "Catalog >
Reload" menu action, `crates/dtrpg-ui/src/controllers/library.rs`) bypasses that check
entirely via `force_reload = true` — today, pressing Reload always performs a full live
fetch no matter how recently the catalog was last refreshed. `CacheMetadata::saved_at_secs`
already records the timestamp of the last successful cache write, which happens after every
successful live fetch, forced or not.

`LibraryService::get_item(id: u64) -> Result<LibraryItem, LibraryServiceError>` already
exists and is the mechanism the (currently unused-by-`LibraryController`) `LibraryViewModel`
uses for detail fetches; `LibraryController::select_item` (single-click popover) and
`TabsController::open_detail_tab` (expanded detail tab) are the two places a user "views an
entry's details" today, and neither currently re-fetches anything — both just read from the
already-loaded `self.catalog`. A `get_item(id)` failure with
`LibraryServiceErrorKind::NotFound` is the existing signal for "the server no longer has
this item" (see `LibraryViewModel::select_item`'s handling of that error kind).

The thumbnail pipeline (`enqueue_thumbnails` / `drain_thumbnail_queue` /
`CoverCache::in_flight`, same file) is an existing precedent for exactly the shape of
mechanism item-level reconciliation needs: a `VecDeque`-backed queue, a single-flight guard
so only one fetch runs at a time, an in-flight set the UI can query per-item to render a
loading indicator, and a self-rescheduling drain loop (`ctrl.drain_thumbnail_queue(cx)`
called again from within the completion handler).

The DriveThruRPG SDK's `LibraryItemsParams` (`dtrpg-sdk` crate) already has an
`updated_date_after: Option<String>` field (ISO 8601, maps to the API's
`updatedDate[after]` query parameter) that is not currently used anywhere in this app —
exactly what a partial "items added or changed since X" fetch needs.

## Goals / Non-Goals

**Goals:**
- Reconcile `live_items` against the existing `self.catalog` by item id when a local
  baseline exists, instead of overwriting it.
- Preserve every existing `catalog-live-data-swap` guarantee: no partial/flashing state
  during the fetch, cached data untouched on fetch failure, one single visible update at the
  end.
- Keep the reconciliation itself cheap and synchronous (catalogs are small; no need for a
  streaming diff).
- Formalize the three catalog-load triggers as distinct, spec'd behaviors: no-local-data
  (incremental populate, no reconciliation), user-requested full reload (cooldown-gated
  full fetch + reconcile), and routine load with local data present (reconcile, same as a
  user-requested reload once it proceeds).
- Add a lightweight, item-scoped reconciliation path (single `get_item` call) that runs
  independently of any full catalog load, triggered either by the user viewing an entry's
  details or by a periodic background queue, with visible in-progress indicators and
  cooldowns that bound both triggers.
- Reduce startup network cost when only a few items changed, by using the SDK's
  `updated_date_after` filter for an additive partial fetch instead of always paginating
  the entire catalog.

**Non-Goals:**
- Deduplicating or diffing individual file entries within a `LibraryItemFile` bundle — out
  of scope, unrelated to top-level catalog membership.
- Any UI for manually re-adding or purging unavailable items (e.g. a "remove permanently"
  action) — this change only maintains the flag; user-facing management of unavailable
  items is a follow-up if wanted.
- Changing the mechanics of the empty-catalog incremental path (`append_catalog_page`) —
  first-launch and post-`clear_and_reload` loads stay a plain page-by-page populate, since
  there is nothing to reconcile against; this change only formalizes that existing behavior
  as a spec requirement, it doesn't alter it.
- Surfacing any UI feedback when a manual reload is suppressed by the cooldown (e.g. a
  toast) — the request is a silent no-op; adding user-visible feedback is a follow-up if
  the cooldown proves confusing in practice.
- Detecting removals via the partial date-filtered fetch — it is additive-only by design
  (see Decisions); removal detection still comes from a full reconciliation fetch or from
  item-level checks catching up over time.
- Exposing cooldown durations as user-configurable settings — fixed constants are
  sufficient; this is a follow-up if a real need for tuning emerges.
- Item-level checks adding or removing catalog entries — a single-item check only ever
  updates fields and `is_available` on an item that already exists locally.

## Decisions

### Reconcile by `id: Arc<str>` using a `HashMap` keyed lookup

Build a `HashMap<Arc<str>, LibraryItem>` from `live_items` (O(n)), then walk the existing
`self.catalog` in place:
- If an existing item's id is in the live map: replace it with the live version (refreshed
  fields) and set `is_available = true`; remove it from the map so it isn't re-appended.
- If an existing item's id is *not* in the live map: keep it, set `is_available = false`.
- After the walk, append whatever remains in the map (genuinely new items, `is_available =
  true`).

This keeps existing catalog ordering stable for known items (important since
`added_order`/list position is otherwise meaningful) and only appends new items at the end,
consistent with how `append_catalog_page` already grows the catalog.

**Why not diff by a composite key (id + publisher)?** `id` is already documented as the
stable unique identifier and used for identity elsewhere in this file (`dedupe_files`
comment). Introducing a second key would only matter if the API were known to reuse ids
across distinct products, which isn't the case here.

**Why not keep a separate "removed" list instead of a flag on `LibraryItem`?** A flag keeps
existing item lookups (id → item) working unchanged for every other feature (collections,
thumbnails, detail view) instead of requiring every consumer to also check a parallel
removed-set.

### `is_available: bool` on `LibraryItem`, defaulting to `true`

`#[serde(default = "default_true")]` (serde's `bool` default is `false`, so an explicit
default fn is needed) so cache files written before this change load with every item marked
available — correct, since those items were current as of the last successful sync.

### Reconciliation only runs when a local baseline exists

Gate reconciliation on the same `catalog_was_empty` check already computed for the
append-vs-buffer branch: if the catalog was empty when the fetch started, `set_catalog` is
called with a plain replace (today's `self.catalog = items` behavior) since there's nothing
to reconcile against and the incremental `append_catalog_page` path already populated it
page-by-page anyway. If the catalog was non-empty, `set_catalog` runs the reconcile-by-id
merge described above instead of a replace.

Practically this means `set_catalog` takes an extra `reconcile: bool` parameter (or is split
into `set_catalog` / `reconcile_catalog`), threaded from the one call site in the fetch
completion handler.

### Cache write includes `is_available`

No separate decision needed — `save_catalog_cache` already serializes whatever is in
`self.catalog` after `set_catalog` completes, so the flag persists for free once it's part
of the struct.

### Three load triggers map to two code paths, not three

`start_load_inner` already branches on `catalog_was_empty` to choose between the incremental
append path and the buffer-then-swap path. That branch is sufficient for all three
user-facing triggers:

- **No local data**: `catalog_was_empty == true` → incremental append path, no
  reconciliation (unchanged from today).
- **Routine load with local data**: `catalog_was_empty == false`, reached via the normal
  auto-load flow (either the freshness check lets the fetch through, or the cache was
  stale) → reconcile-by-id path.
- **User-requested full reload**: also `catalog_was_empty == false` in the overwhelmingly
  common case (a reload only makes sense once there's something to reload) → same
  reconcile-by-id path, gated by the new cooldown check described below before
  `start_load_inner` is even called.

No third code path is needed — "full reload" and "routine reconciled load" already produce
identical behavior once the decision to fetch has been made. The only genuinely new logic is
the cooldown gate in front of the manual trigger.

### Manual reload cooldown: reuse `CacheMetadata::saved_at_secs`, gated in `reload_catalog`

Add a new constant, `FORCE_RELOAD_COOLDOWN_SECS` (`crates/dtrpg-ui/src/data/constants.rs`),
set to 60 seconds — long enough to absorb accidental double-invocations of the menu action
(e.g. a stuck keybinding or an impatient double-click) without meaningfully delaying a
deliberate second reload a user actually wants. This is a distinct constant from
`STALE_SECS` (7 days): `STALE_SECS` answers "is the cached data old enough that a *passive*
load should refresh it," while `FORCE_RELOAD_COOLDOWN_SECS` answers "was a *manual* reload
already attempted moments ago."

`reload_catalog` reads the existing on-disk `CacheMetadata` (via `load_cache_metadata`, same
helper `start_load_inner` already uses) before calling `start_load_inner`. If
`saved_at_secs` is within `FORCE_RELOAD_COOLDOWN_SECS` of now, the call returns immediately
without touching `catalog_loading`, `load_generation`, or the network — the catalog and UI
are left exactly as they were. Otherwise it proceeds as today
(`self.catalog_loading = true`, emit `LibraryChanged`, `start_load_inner(cx, true)`).

**Why key the cooldown off `CacheMetadata::saved_at_secs` instead of a new in-memory
"last reload attempt" timestamp?** `saved_at_secs` is already updated after every
successful live fetch (forced or passive) and already persists across app restarts, which
is the correct semantics here — the cooldown should track "how long ago did we last
actually get fresh data," not "how long ago did the user last click the button." A
transient in-memory timestamp would reset the cooldown on every app restart and wouldn't
protect against restart-mash-reload sequences.

**Why does `force_reload = true` still bypass the freshness/count-match check inside
`start_load_inner`?** That check is what lets a *passive* load skip the network entirely
when the cache already matches the remote count — it's an optimization for loads the user
didn't ask for. A reload the user explicitly requested, once past the cooldown, should
still go to the network: skipping it because the count happens to match would make the
"Reload" action silently do nothing on the (common) case where nothing changed, which is
indistinguishable from a bug. The cooldown itself is now the only "not too recently" gate
that applies to manual reload.

### UI treatment: dim + badge, not hide

Unavailable items stay visible in the catalog list (owners may still want to open/download
previously-fetched files) but are visually deemphasized with a small "unavailable" badge,
consistent with how `catalog-title-tooltip`/`catalog-menu` already surface item metadata.
Exact styling is an implementation detail for the tasks/spec work, not a design constraint.

### Single-item check: one `get_item` call, gated by a per-item cooldown timestamp

`LibraryItem` gains `#[serde(skip)] availability_last_checked: Option<SystemTime>`, mirroring
the existing `thumbnail_last_attempted` field exactly (same rationale: ephemeral, not worth
persisting, reset naturally on process restart). A new constant `ITEM_CHECK_COOLDOWN_SECS`
(5 minutes) gates re-checks of the *same* item: `select_item` and the detail-tab open path
call a new `LibraryController::maybe_check_item(id, cx)` that no-ops if
`availability_last_checked` is within the cooldown, otherwise calls `get_item` on the
background executor and applies the result:
- `Ok(item)` → replace the catalog entry's fields with the fresh data **except**
  `id`/`numeric_id`/`order_product_id`/`product_id`, which are preserved from the existing
  entry — a single-item re-check is not authoritative for identity/collection-membership
  ids, and letting the fresh response overwrite them was found (during manual verification)
  to silently break `collection_member_id` lookups for the checked item whenever the
  single-item endpoint's response didn't carry the exact same id values the list fetch
  populated. Set `is_available = true`, set `availability_last_checked = Some(now)`.
- `Err(e)` where `e.kind == NotFound` → leave other fields as-is, set `is_available = false`,
  set `availability_last_checked = Some(now)`.
- Any other error (network, session, rate limit) → leave the item and its flag entirely
  unchanged except for logging; a transient failure is not evidence of removal. This mirrors
  the full-reconciliation risk already noted below: only an explicit "not found" response is
  trusted to flip availability.

**Why a per-item cooldown instead of always checking on every detail view?** Users often
reopen the same item's detail panel repeatedly while browsing (e.g. comparing two products
back and forth). Without a cooldown this would issue a redundant network call on every click.
5 minutes is short enough that a check still feels "fresh" relative to a browsing session,
long enough to absorb repeated clicks.

### Checking-in-progress indicator: `checking_items: HashSet<Arc<str>>` on `LibraryController`

Not persisted — mirrors `CoverCache::in_flight`'s role for thumbnails. An id is inserted when
a single-item check (on-demand or queued) starts and removed when it completes, with
`LibraryChanged` emitted on both transitions so catalog card/row rendering can query
`ctrl.is_checking(id)` and show a small spinner/overlay, the same way thumbnail rendering
already queries in-flight state.

### Periodic per-item check queue: mirrors `thumbnail_queue` / `drain_thumbnail_queue`

Add `check_queue: VecDeque<Arc<str>>` and reuse the single-flight pattern (a queue drain
that only starts the next check once the current one's `this.update(...)` completion handler
calls itself again) rather than `thumbnail_loading`'s single bool, since checks should
process one at a time to stay gentle on the API — a `checking_items` with len-based
single-flight guard (drain only pops when `checking_items.is_empty()`) is enough; no separate
bool needed.

`enqueue_checks(ids: impl Iterator<Item = Arc<str>>, cx)` pushes ids not already queued or
in-flight onto `check_queue` and calls `drain_check_queue(cx)`, which pops one id, calls the
same single-item check logic `maybe_check_item` uses (bypassing its own cooldown check, since
queue population already filters by staleness — see below), and on completion recurses into
itself for the next queued id.

### Manual "check for updates" and automatic periodic enqueueing share one cooldown, persisted in `CacheMetadata`

Add `last_item_check_batch_secs: Option<u64>` to `CacheMetadata`
(`crates/dtrpg-ui/src/data/catalog_cache.rs`, `#[serde(default)]` so existing metadata files
deserialize with `None`) and a new constant `ITEM_CHECK_BATCH_COOLDOWN_SECS` (15 minutes).
Both triggers funnel through one method, `LibraryController::request_check_batch(cx)`:
1. Load `CacheMetadata`; if `last_item_check_batch_secs` is within
   `ITEM_CHECK_BATCH_COOLDOWN_SECS` of now, no-op (silent, same rationale as the reload
   cooldown).
2. Otherwise, select up to a bounded batch size (e.g. 50) of catalog items whose
   `availability_last_checked` is `None` or older than `ITEM_CHECK_COOLDOWN_SECS`,
   oldest-checked first, and pass them to `enqueue_checks`.
3. Persist `last_item_check_batch_secs = Some(now)` immediately (not after the batch
   finishes) so a slow-draining queue doesn't leave the cooldown window open to a second
   trigger firing mid-batch.

The "manual" trigger (a new menu action or catalog-view control) and the "automatic" trigger
(a recurring `cx.spawn` loop using `background_executor().timer(Duration)` in a loop, the
same primitive `ActivityController::complete`/`error` already use for one-shot delays — here
looped, checking `request_check_batch` on each wake) both just call
`request_check_batch(cx)`; the shared cooldown is what makes "automatic checks also conform
to cooldowns" true by construction rather than by parallel bookkeeping.

**Why one shared cooldown for manual and automatic instead of two separate ones?** The
concern in both cases is identical — bounding total per-item-check request volume against
the API — so a single timestamp is sufficient and avoids the two triggers stacking (e.g. an
automatic tick firing moments after a manual trigger, each thinking it's within its own
separate allowance).

**Why persist in `CacheMetadata` rather than a new sidecar file?** `CacheMetadata` already
exists specifically to hold small, frequently-read timestamp/count bookkeeping for the
catalog; a second near-identical file would duplicate the load/save plumbing for no benefit.

### Startup count check chooses skip / partial fetch / full fetch

Extends the existing auto-load-policy count check (`svc.count_items()` vs. cached item
count) with a third outcome. Today: match → skip; mismatch → full fetch. New: match → skip
(unchanged); `remote_count > cached_count` → partial fetch; any other mismatch (including
`remote_count < cached_count`, which a pure-addition partial fetch cannot explain or repair)
→ full fetch (unchanged behavior for that branch).

The partial fetch calls a new `LibraryService::list_items_updated_since(since_iso8601,
on_page) -> Option<Result<(), LibraryServiceError>>` — `None` means unsupported (mirrors
`count_items`'s convention), in which case the caller falls back to a full fetch. The real
SDK-backed implementation sets `LibraryItemsParams::updated_date_after` and paginates exactly
like `list_items_paged`, calling `on_page` incrementally. `since_iso8601` is derived from the
local catalog's most recent `date_updated` (falling back to `date_added`) across all cached
items.

Partial results are merged additively only — reusing the "new item" half of the
reconcile-by-id logic (append if absent, refresh fields + set `is_available = true` if
present) but skipping the "mark absent items unavailable" sweep entirely, since a partial
response is never a complete listing and absence from it means nothing.

**Why not attempt to infer removals from a count *decrease*?** A decrease only proves *some*
item is gone, not which one — a full fetch is the only way to identify it via reconciliation.
Rather than build a heuristic for a case reconciliation already handles correctly, mismatches
that aren't explained by pure growth fall through to the existing full-fetch path.

**Why does this compose safely with item-level checks and full reconciliation rather than
race them?** A partial fetch, a full fetch, and item-level checks all funnel through the same
underlying catalog mutation primitives (`reconcile_catalog`'s additive half, or direct
per-item field/flag updates) and the same `load_generation` guard already used to discard
stale in-flight loads — no new coordination mechanism is needed.

## Risks / Trade-offs

- **Stale local metadata on flaky server responses**: if the server returns an incomplete
  page set due to a transient error that doesn't surface as a hard failure, unaffected
  items get incorrectly flagged unavailable → mitigated by the existing `fetch.await`
  error path already aborting `set_catalog` entirely on a hard failure; a *partial success*
  that silently drops pages is a pre-existing pagination-integrity risk this change doesn't
  introduce or worsen.
- **Reconciliation cost grows with catalog size**: `O(n)` `HashMap` build + walk is
  negligible for realistic catalog sizes (hundreds to low thousands of items); no need for
  incremental/streaming reconciliation.
- **Cache schema change**: additive-only (`#[serde(default)]`), so old cache files remain
  loadable without a migration step.
- **Silent cooldown no-op may confuse a user who reloads twice in quick succession
  deliberately** (e.g. after fixing a network issue) → mitigated by keeping the cooldown
  short (60s) relative to the 7-day passive staleness window; a user who waits a minute
  gets a real reload. Adding explicit feedback is a documented non-goal, revisit if this
  proves confusing in practice.
- **Cache metadata missing or unreadable when `reload_catalog` checks it** (e.g. first
  reload attempt on a fresh install with no prior successful sync) → treated as "not
  recently reloaded," so the cooldown never blocks a reload when there's no prior
  timestamp to compare against, consistent with `load_cache_metadata` returning `None`
  meaning "treat as stale" elsewhere in this file.
- **Partial fetch misses a removal that happens to coincide with an addition, keeping
  `remote_count > cached_count` true** (e.g. one item removed, two added, net +1) → the
  removed item stays flagged available until the next full reconciliation or until its own
  item-level check catches it. This is an accepted approximation in exchange for a cheaper
  common-case startup; the item-level check queue is exactly the backstop that bounds how
  long this staleness can persist.
- **Automatic periodic check timer runs indefinitely while the app is open, even with
  nothing new to check** → bounded by `request_check_batch`'s cooldown gate (a wasted timer
  tick when nothing is due is cheap — no network call happens, just a metadata read) and by
  only selecting items actually overdue for a check.
- **`checking_items` never clears if a queued check's `this.update` closure never runs**
  (e.g. the controller entity is dropped mid-check) → same class of risk the existing
  thumbnail queue already accepts (`thumbnail_loading` has the identical failure mode); no
  new mitigation needed beyond what the precedent already tolerates.

## Migration Plan

No data migration required. Existing cache files gain `is_available: true` on next load via
`serde` default; the next successful sync reconciles them against live data normally.
Existing `CacheMetadata` sidecar files gain `last_item_check_batch_secs: None` via the same
`#[serde(default)]` mechanism, so the first post-upgrade check batch is never blocked by a
cooldown it has no record of.

## Open Questions

None — unavailable items stay in search/filter results by default (no separate toggle),
matching the proposal's "flag, don't remove" framing and keeping this change's scope to
data reconciliation rather than new filter UI.
