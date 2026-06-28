## Why

Every launch fetches the full catalog from the API before showing the user anything, adding noticeable latency even for users with a slow or unavailable connection. Persisting catalog data to disk and loading it at startup gives users immediate access to their library while the API refresh runs in the background.

## What Changes

- `LibraryItem` gains `serde::Serialize` / `serde::Deserialize` derives so it can be round-tripped through JSON.
- A new `catalog_cache` data module writes the full catalog to `{storage_root}/catalog_cache.json` after a successful API load, and reads it back on startup.
- `LibraryController::new` is updated to a two-phase load: first populate from cache (instant), then fetch from API and replace (background). The UI is populated from cache before the API request begins.
- If no cache file exists (first launch, file deleted), the sequence degrades gracefully to the current behavior — blank state followed by incremental API population.
- On a successful API load the cache file is atomically replaced with fresh data. On API failure the cached data remains visible.

## Capabilities

### New Capabilities

- `catalog-disk-cache`: Catalog items are persisted to disk after each successful API load and served from that cache on the next startup before the API refresh begins.

### Modified Capabilities

<!-- none — incremental-catalog-population still applies; the cache is a pre-population step before the API fetch begins -->

## Impact

- `dtrpg-ui/src/data/library.rs` — add `#[derive(Serialize, Deserialize)]` to `LibraryItem`; add serde attribute annotations for `Arc<str>` fields
- `dtrpg-ui/src/data/catalog_cache.rs` (new) — `load_catalog_cache(root: &Path) -> Option<Vec<LibraryItem>>` and `save_catalog_cache(root: &Path, items: &[LibraryItem]) -> Result<(), CatalogCacheError>`; uses `{root}/catalog_cache.json`; write via temp-file + rename for atomicity
- `dtrpg-ui/src/controllers/library.rs` — `LibraryController::new` loads cache before spawning the API fetch; after all API pages arrive, calls `save_catalog_cache`; `StorageConfig` is passed in to supply the root path
- `dtrpg-ui/Cargo.toml` — `serde_json` already likely present; confirm or add; no new heavy dependencies
- `ItemStatus` enum may also need `Serialize`/`Deserialize` if not already derived
