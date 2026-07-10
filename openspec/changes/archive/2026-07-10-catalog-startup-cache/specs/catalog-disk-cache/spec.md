## ADDED Requirements

### Requirement: Catalog is populated from disk cache before API fetch begins
On startup, the app SHALL attempt to load catalog items from a local cache file before initiating any API request. If the cache contains valid data, it SHALL be used to immediately populate the catalog view.

#### Scenario: Cache exists and is valid on startup
- **WHEN** the app launches and a valid `catalog_cache.json` file exists in the storage root
- **THEN** the catalog view is populated with the cached items before any API request is made

#### Scenario: No cache file exists on first launch
- **WHEN** the app launches and no cache file is present
- **THEN** the catalog starts empty and is populated incrementally as API pages arrive (existing behavior)

#### Scenario: Cache file is unreadable or malformed
- **WHEN** the app launches and the cache file exists but cannot be parsed
- **THEN** the cache is silently ignored, no error is shown to the user, and the catalog is populated from the API as normal

### Requirement: Catalog cache is written after every successful API load
After all API pages have been received without error, the app SHALL atomically write the complete catalog to `catalog_cache.json` in the configured storage root, replacing any prior cache file.

#### Scenario: API load completes successfully
- **WHEN** all catalog pages are received from the API without error
- **THEN** the full catalog is written to `{storage_root}/catalog_cache.json` using an atomic temp-file rename

#### Scenario: API load fails
- **WHEN** the API fetch encounters an error
- **THEN** the existing cache file is left unchanged and the user sees their cached catalog alongside an error in the activity panel

#### Scenario: Cache write fails due to storage error
- **WHEN** the atomic write fails (e.g., storage root is on an unmounted volume)
- **THEN** the failure is logged at warn level and the app continues without surfacing an error to the user

### Requirement: Cached data is replaced by fresh API data when the API load succeeds
After a successful API fetch, the catalog view SHALL reflect the live API data, not the stale cache.

#### Scenario: API load completes after cache was shown
- **WHEN** the user sees their cached catalog and the API fetch subsequently completes
- **THEN** the catalog view is updated to reflect the full API result, replacing the cached items
