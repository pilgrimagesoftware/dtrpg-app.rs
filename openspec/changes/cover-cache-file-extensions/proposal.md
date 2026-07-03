## Why

`data/cover_cache.rs` writes every cached cover thumbnail to disk with a fixed `.cover`
extension regardless of the actual image format (JPEG, PNG, WebP, GIF, or BMP, as sniffed
from the leading bytes when decoding). The bytes on disk are correct, but the filename lies
about their type — this makes the cache directory unreadable/unusable with any external
tool (image viewers, `file`, Finder QuickLook) and duplicates the format-sniffing logic
that already exists in `ui::library::cover::sniff_image_format` in spirit but not in code,
since the cache layer never needed to know the format until now.

## What Changes

- Cached cover files are now named `{hash}.{ext}`, where `{ext}` is the real extension
  for the sniffed format (`jpg`, `png`, `webp`, `gif`, `bmp`) instead of the generic
  `.cover` suffix.
- The byte-sniffing logic is extracted into a shared, UI-independent helper so
  `data/cover_cache.rs` (data layer) and `ui/library/cover.rs` (UI layer, decodes into a
  `gpui::Image`) both call the same detection code instead of maintaining separate copies
  — matching the project's existing "reuse code, don't duplicate it" convention applied
  to `util::hash::fnv1a_32` in the prior thumbnail-disk-cache change.
- `load_cached_cover` no longer assumes a single fixed extension: since the extension is
  now derived from file content rather than fixed, lookups check each known extension in
  turn (bounded, no directory scan) to find whichever file exists for a given item's hash.
- **No migration of existing `.cover` files.** They become orphaned dead weight in the
  cache directory (cheap thumbnails, regenerable cache location) rather than being
  rewritten or deleted; the first launch after this change re-downloads every thumbnail
  once (same one-time cost as clearing the cache), then caches correctly going forward.
  See design.md for the rationale.

## Capabilities

### New Capabilities

- `cover-thumbnail-disk-cache`: the on-disk cover thumbnail cache has never had a spec
  (it was implemented ad hoc, outside the openspec workflow). This change is the first
  to touch it, so it introduces the baseline spec — cache hit/miss/fetch behavior plus
  the correct-extension requirement this change actually delivers — rather than leaving
  the capability permanently undocumented.

### Modified Capabilities

<!-- none — no existing spec'd capability changes; see New Capabilities above -->

## Impact

- `crates/dtrpg-ui/src/data/cover_cache.rs`: filename derivation and `load_cached_cover`
  changed to be extension-aware.
- `crates/dtrpg-ui/src/ui/library/cover.rs`: `sniff_image_format` replaced with a call
  into the new shared sniffing helper.
- New shared module (exact location decided in design.md) for image-format sniffing,
  used by both of the above.
- No changes to `controllers/library.rs`'s thumbnail queue/fetch orchestration, which
  only calls `load_cached_cover`/`save_cached_cover` and doesn't care about the on-disk
  filename shape.
