## Context

`app_cache_dir()` (`dtrpg-ui/src/data/paths.rs`) is the root of all regenerable app cache data, holding three kinds of content today:
- `cache_dir()` (`{app_cache_dir}/metadata`): the catalog/collections metadata cache, a JSON file plus a sidecar `catalog_cache_meta.json` (`CacheMetadata { item_count, saved_at_secs, schema_version }`, see `data/catalog_cache.rs`) that already records the item count without needing to parse the full catalog.
- `covers_dir()` (`{app_cache_dir}/covers`): one file per cached cover thumbnail, written/read via `save_cached_cover`/`load_cached_cover` in `controllers/library.rs`.
- The avatar cache: a single file at `{app_cache_dir}/{AVATAR_CACHE_FILE}` (`data/avatar.rs`), present or absent — not a count, a boolean.

`SettingsController::clear_cache` already deletes all of `app_cache_dir()` wholesale and emits `CacheCleared`. `reveal_storage_location` already demonstrates the reveal-in-file-manager pattern this change reuses for the cache folder.

## Goals / Non-Goals

**Goals:**
- Show the user, without leaving Settings, roughly how much cached data exists: metadata item count, cover thumbnail count, whether the avatar is cached.
- Let the user open the cache folder directly in Finder/Explorer/file manager to inspect it manually.
- Keep counts cheap to compute (metadata count is a JSON sidecar read; covers count is a directory listing) so they can be recomputed on section show/refresh without a background job.

**Non-Goals:**
- No persistent/live-updating counter — counts are read fresh each time the Advanced section is shown or right after Clear Cache runs, not tracked incrementally as items are added.
- No per-item cache browsing or selective deletion — "Clear cache" remains all-or-nothing; this change only adds visibility and folder access.
- No change to what gets cached or where — `app_cache_dir()`/`cache_dir()`/`covers_dir()` layout is unchanged.
- No settings UI to *edit* the timeout/cooldown values — they're displayed as fixed constants, not configuration. Making them user-tunable is a separate, much larger change (validation, persistence, interaction with the existing hardcoded call sites) and isn't needed to satisfy "why is the app behaving this way."

## Decisions

- **Compute counts via a new `SettingsController` method (`cache_counts` or similar) rather than a background-refreshed field.** Metadata count comes from `load_cache_metadata(&cache_dir()).map(|m| m.item_count)` (already-written data, no need to re-parse the full catalog JSON). Covers count comes from `std::fs::read_dir(covers_dir())` counting entries. Avatar presence is `avatar_cache_path().exists()` — but `avatar_cache_path()` is private to `data/avatar.rs` today, so either expose a small `pub fn avatar_cached() -> bool` there, or duplicate the path join (`app_cache_dir().join(AVATAR_CACHE_FILE)`) in the settings controller. Prefer exposing `avatar_cached()` from `data/avatar.rs` to avoid duplicating the path logic.
  - Alternative considered: track counts incrementally as items/covers are cached, avoiding a directory read. Rejected — added bookkeeping across every cache-write call site for a value only displayed in a rarely-visited settings panel; a `read_dir` count is cheap enough (covers directories are at most a few hundred entries) to compute on demand.
- **Counts are computed synchronously on section render, matching the existing pattern where `SettingsSnapshot` is rebuilt on each render pass.** No `cx.spawn` needed since all three reads are fast local filesystem operations (JSON sidecar read, one `read_dir`, one `exists()` check) — consistent with how `storage_path_exists` is the one thing in this controller that *does* go through `cx.spawn` because it can involve slower/networked filesystems (user-configured storage root), whereas the cache directories are always local app-managed paths.
- **"Open cache folder" reuses `reveal_in_file_manager(&app_cache_dir())` directly**, following `reveal_storage_location`'s pattern: create the directory first if missing (a fresh install may have no cache yet), then reveal it. No new platform-specific code needed.
- **After "Clear cache" runs, the Advanced section's displayed counts must reflect zero/empty immediately.** Since counts are computed on render (not cached in state), the counts naturally read as zero/empty on the next render after `CacheCleared` fires and the view re-renders — no explicit invalidation needed beyond what `clear_cache` already does (emitting `CacheCleared`, which the view already reacts to).
- **Timing constants are read directly from `data::constants` by the view — no controller method, no `SettingsSnapshot` field.** Unlike the counts, they require no I/O and never change at runtime, so routing them through the controller would add indirection with no benefit. `settings_advanced_view.rs` imports the six constants directly and renders them.
- **Consolidate `STALE_SECS` (`catalog_cache.rs`) and `THUMBNAIL_COOLDOWN_SECS` (`library.rs`) into `data/constants.rs`, alongside the four cooldown constants already there.** This repo's Rust conventions call for a single constants location per crate; these two were the last stragglers. Both become `pub`. Call sites (`catalog_cache.rs::is_stale`, `library.rs::thumbnail_cooldown_elapsed`) update their imports; behavior is unchanged, only the constant's location and visibility move.
- **Every stat (count or timing) renders as: bold label, secondary-color value, tertiary-color one-line description underneath** — matching this file's existing pattern for `clear_cache_description` (a description line directly above its associated control) rather than introducing tooltips. The proposal allows either tooltip or accompanying text; accompanying text was chosen because `DescriptionList` (this repo's other label/value component) has no description slot, and building a custom tooltip-on-label interaction for six-plus rows is meaningfully more code than a `div` for comparable clarity. A small local helper, `stat_row(label, value, description, colors) -> impl IntoElement`, is shared by both counts and timings to avoid repeating the three-line layout six-plus times.
- **A local `format_static_duration(secs: u64) -> String` helper renders timing constants as "60 seconds" / "5 minutes" / "15 minutes" / "7 days"**, distinct from `activity_panel_view.rs`'s existing `format_duration` (which renders elapsed time as "Xm Ys" and would print "7 days" as "10080m 0s"). Lives in `settings_advanced_view.rs` since it's presentation-only and has exactly one caller site (six values, one function).

## Risks / Trade-offs

- [Risk] `read_dir` on `covers_dir()` could be slow if the directory accumulates a very large number of files over long-term use. → Mitigation: cover thumbnails are bounded by catalog size (hundreds, not tens of thousands, for a typical DriveThruRPG library); acceptable for a settings-panel display. Revisit only if this proves slow in practice.
- [Risk] Exposing `avatar_cached()` from `data/avatar.rs` slightly widens that module's public surface. → Mitigation: it's a single boolean-returning function with no new state; consistent with the module's existing narrow purpose.
- [Risk] Counts could be momentarily wrong if a background load is actively writing to the cache while the section is open. → Mitigation: acceptable — counts are advisory ("roughly how much is cached"), not a precise live gauge; refreshing the section (e.g. reopening settings) picks up the current state.

## Migration Plan

- Implemented as a single PR within `dtrpg-app/rust`: add the count-computation method(s), add `avatar_cached()` if needed, wire the new UI subsection, verify counts and folder-open manually.
- No data migration; purely additive UI/read-only behavior.
- Rollback is a straight revert if issues surface.

## Open Questions

- Should the avatar cache be shown as a boolean ("cached" / "not cached") or omitted from the counts list entirely since it's not really a "count"? Proposal: show it as a simple presence indicator alongside the two numeric counts, since it's still cache data users may want visibility into.
