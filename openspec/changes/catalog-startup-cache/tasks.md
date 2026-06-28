## 1. Add serde Support to Data Types

- [x] 1.1 Add `serde_json = "1"` to the workspace `Cargo.toml` under `[workspace.dependencies]`
- [x] 1.2 Add `serde_json = { workspace = true }` to `dtrpg-ui/Cargo.toml`
- [x] 1.3 Add `#[derive(Serialize, Deserialize)]` to `LibraryItem` in `data/library.rs`; annotate `Arc<str>` fields with `#[serde(default)]` where appropriate
- [x] 1.4 Add `#[derive(Serialize, Deserialize)]` to `ItemStatus` in `data/enums.rs`
- [x] 1.5 Run `cargo check --all-targets` and confirm no new errors

## 2. Implement catalog_cache Module

- [x] 2.1 Create `dtrpg-ui/src/data/catalog_cache.rs` with a `CatalogCacheError` enum (derive `thiserror::Error`)
- [x] 2.2 Implement `load_catalog_cache(root: &Path) -> Option<Vec<LibraryItem>>`: read `{root}/catalog_cache.json`, deserialize with `serde_json::from_str`; return `None` on any error (log at `warn`)
- [x] 2.3 Implement `save_catalog_cache(root: &Path, items: &[LibraryItem]) -> Result<(), CatalogCacheError>`: write JSON to `{root}/catalog_cache.json.tmp`, then `fs::rename` to `catalog_cache.json`
- [x] 2.4 Register `mod catalog_cache;` in `data/mod.rs`
- [x] 2.5 Write unit tests for `load_catalog_cache` with a valid JSON file, a missing file, and a malformed file

## 3. Wire Cache into LibraryController

- [x] 3.1 Add `StorageConfig` as a parameter to `LibraryController::new` (or read it inside via `StorageConfig::load()`)
- [x] 3.2 Before spawning the API fetch task: call `load_catalog_cache` on the background executor and, if `Some(items)`, call `append_catalog_page` (or a new `set_catalog_from_cache` method) to pre-populate the catalog
- [x] 3.3 After all API pages arrive successfully: call `save_catalog_cache` with the final catalog; replace the pre-populated catalog with the live API data
- [x] 3.4 Ensure API failure path leaves the cache file and in-memory catalog from step 3.2 unchanged
- [x] 3.5 Update callers of `LibraryController::new` to pass `StorageConfig` if the signature changes

## 4. Verification

- [x] 4.5 Run `cargo test --all-features --workspace` with no failures
- [x] 4.6 Run `cargo clippy --all-targets --all-features -- -D warnings` with no new warnings
- [ ] 4.1 Build and run the app: confirm catalog appears instantly on second launch (from cache)
- [ ] 4.2 Confirm catalog updates to live API data after the fetch completes
- [ ] 4.3 Delete `catalog_cache.json` and relaunch: confirm graceful degradation to API-only load
- [ ] 4.4 Corrupt `catalog_cache.json` and relaunch: confirm no error is surfaced to the user
