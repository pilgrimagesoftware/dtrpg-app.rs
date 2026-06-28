## Context

`LibraryController::new` currently starts with an empty catalog and performs a paginated API fetch on a background thread. The UI shows a loading state until the first API page arrives. `LibraryItem` derives no serde traits; there is no disk persistence for catalog data.

The `StorageConfig` type (already implemented) resolves the user-configured storage root and exposes `root_path() -> PathBuf`. This is the natural anchor for the cache file location.

`serde = { version = "1", features = ["derive"] }` is in the workspace; `serde_json` is not.

## Goals / Non-Goals

**Goals:**
- Populate the catalog from a disk cache on startup before the API fetch begins.
- Replace the cache file with fresh data after every successful API load.
- Degrade gracefully when no cache exists or the cache is unreadable.

**Non-Goals:**
- Cache invalidation beyond "replace on successful API load" (no TTL, no version stamping).
- Partial/incremental cache writes during the API fetch — write once when all pages are done.
- Encrypting or compressing the cache file.
- Showing a "loaded from cache" indicator in the UI.

## Decisions

### Cache format: JSON via `serde_json`

JSON is human-inspectable, already familiar in this repo (avatar disk cache uses it), and `serde_json` is a lightweight addition. Binary formats (bincode, MessagePack) are faster but add opaque blobs that are harder to debug. For catalog sizes in the thousands of items, JSON parse time is negligible at startup.

`serde_json` goes into the workspace `Cargo.toml` and is referenced from `dtrpg-ui/Cargo.toml`.

### Cache location: `{storage_root}/catalog_cache.json`

`StorageConfig::root_path()` is already the designated home for catalog-related data. Storing the cache there keeps all catalog data co-located. A subdirectory is unnecessary for a single file.

### Atomic write: temp file + rename

Write to `catalog_cache.json.tmp` then `fs::rename` to `catalog_cache.json`. On POSIX systems, rename is atomic within the same filesystem. This prevents a partial write from leaving a corrupted cache file on crash or power loss.

### Load ordering in `LibraryController::new`

```
startup
  │
  ├─ load cache synchronously on bg thread
  │     └─ if Some(items): populate catalog + emit LibraryChanged
  │
  └─ begin API fetch (existing incremental flow)
        └─ on all pages received: replace catalog + emit LibraryChanged + save cache
```

Cache load is synchronous on the same background executor that already handles API I/O, so no additional threading is introduced. The API fetch begins immediately after — there is no "wait for cache" gate.

### `LibraryItem` serde

Add `#[derive(Serialize, Deserialize)]` to `LibraryItem`. `Arc<str>` serializes as a plain string with serde's default implementation. `ItemStatus` and `SortMethod` enums also need the derives.

`cover_url: Option<Arc<str>>` serializes naturally. No custom serializers are needed.

### Cache replace on API success, ignore on API failure

If the API fetch errors after loading from cache, the stale cache is preserved and the user sees their last-known catalog with an error in the activity panel. If the API fetch succeeds, the cache is atomically replaced. This is the simplest correct behavior.

## Risks / Trade-offs

- **Stale cache on schema change**: Adding or removing fields from `LibraryItem` will cause `serde_json` deserialization to either ignore unknown fields (fine) or fail on missing required fields (panic-free with `Option` defaults). Fields that become required after a schema change should default via `#[serde(default)]`. → Mitigate by using `#[serde(default)]` on any field that may be absent in older cache files.
- **Cache grows unbounded**: Users who never change their library will accumulate a single file that grows with their library. For a typical DTRPG library (hundreds to low thousands of items) this is tens of KB — not a concern. → No action needed.
- **Cache on a missing or unmounted volume**: If the `StorageConfig` root is on an external drive that isn't mounted, cache load silently returns `None` and the API fetch proceeds normally. The write after a successful API load will fail silently (logged at `warn`). → Acceptable; matches existing behavior for download storage.
