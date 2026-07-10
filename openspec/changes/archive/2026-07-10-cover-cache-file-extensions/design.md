## Context

`data/cover_cache.rs` (`load_cached_cover`/`save_cached_cover`) persists raw cover
thumbnail bytes fetched by `controllers/library.rs`'s thumbnail queue. Filenames are
currently `{hash}.cover` where `{hash}` is an FNV-1a hash of the owning item's id — the
extension is a fixed literal, unrelated to the actual bytes.

`ui/library/cover.rs::sniff_image_format` already inspects the leading bytes of a cover
to pick a `gpui::ImageFormat` for decoding (`Png`, `Jpeg`, `Webp`, `Gif`, `Bmp`, defaulting
to `Jpeg`). `data/cover_cache.rs` cannot depend on `ui::library::cover` — the data layer
must stay UI-independent per this crate's module layout (`src/ui/` owns view composition;
`src/data/` owns data/serialization concerns) — so the sniffing logic needs a home neither
module currently occupies.

## Goals / Non-Goals

**Goals:**
- On-disk cover filenames carry the correct extension for their actual byte content.
- Sniffing logic lives in exactly one place, shared by both the data-layer cache writer
  and the UI-layer decoder, rather than duplicated (as `util::hash::fnv1a_32` already
  established as the pattern for this crate).
- `load_cached_cover` keeps working without directory scans or an out-of-band format
  record.

**Non-Goals:**
- No migration/rewrite of already-cached `.cover` files.
- No change to the thumbnail queue's fetch/cache orchestration in `controllers/library.rs`.
- No change to `CoverCache` (the in-memory decoded-image cache) or its `ImageFormat`
  handling — only the disk cache's filename derivation changes.

## Decisions

- **Extract sniffing into `util::image_format` (new module).** Mirrors the
  `util::hash::fnv1a_32` precedent: a small, dependency-free helper both a data-layer and
  a UI-layer module can call. Define a local `ImageKind` enum (`Jpeg`, `Png`, `Webp`,
  `Gif`, `Bmp`) with `fn sniff(bytes: &[u8]) -> ImageKind` and `fn extension(&self) ->
  &'static str`, so neither `data/cover_cache.rs` nor `util::image_format` needs to depend
  on `gpui`. `ui/library/cover.rs` maps `ImageKind` to `gpui::ImageFormat` locally (a
  one-line `match`), keeping the gpui dependency confined to the UI layer.
  - Alternative considered: move the existing `gpui::ImageFormat`-returning function as-is
    into a shared module. Rejected — it would pull a `gpui` dependency into `data/`,
    violating the existing UI/data separation for no benefit (the data layer never needs
    a decoded `Image`, only a file extension).
- **Extension-bounded lookup, not a directory scan.** `load_cached_cover` tries each of
  the five known extensions in a fixed order (`jpg`, `png`, `webp`, `gif`, `bmp`) via a
  direct `{hash}.{ext}` path check, returning the first hit. Five `fs::read` attempts
  (most failing fast on "file not found") is cheap and keeps the lookup O(1) in the
  number of cached items, unlike a `fs::read_dir` scan for a matching hash prefix.
  - Alternative considered: store `{hash}.bin` and keep sniffing on load (status quo,
    just renamed). Rejected — doesn't address the actual ask (a *correct* extension for
    external tooling/inspection).
  - Alternative considered: encode the extension in a sidecar file or the hash itself.
    Rejected as needless complexity next to a bounded 5-branch check.
- **No migration of existing `.cover` files.** They're orphaned but harmless: this is a
  regenerable cache directory (`covers_dir()`, under `app_cache_dir()`), not user data.
  Deleting them proactively would require walking the directory at startup (a scan this
  design otherwise avoids) for a one-time, low-stakes cleanup. Leaving them is equivalent
  in effect to what already happens on any cache-clearing event: the first post-upgrade
  launch re-downloads every thumbnail once (a cache miss under the new extension-aware
  filenames), then caches correctly from then on.

## Risks / Trade-offs

- [Risk] Old `.cover` files accumulate as permanent dead disk space for users who never
  clear their cache → Mitigation: thumbnails are small (typically tens of KB each); the
  existing cache directory has no eviction policy at all today, so this doesn't introduce
  a new class of problem, only adds to an already-accepted one. Worth revisiting if/when
  cache eviction is designed.
- [Risk] `ImageKind::sniff` and `ui::library::cover`'s existing byte-pattern matches must
  stay in exact agreement, or the UI layer could decode a file the cache layer wrote with
  a mismatched extension → Mitigation: this design removes the duplication entirely (one
  function, two callers), so the two can no longer drift apart.
