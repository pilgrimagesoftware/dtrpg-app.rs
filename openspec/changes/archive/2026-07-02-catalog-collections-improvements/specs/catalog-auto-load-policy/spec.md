## ADDED Requirements

### Requirement: Collections load on startup for authenticated sessions
The application SHALL load the collections list from the API on startup whenever an authenticated session is available, not only after sign-in.

#### Scenario: Collections load when already signed in at startup
- **WHEN** the application starts and a valid session credential is present
- **THEN** the collections list is fetched from the API and displayed in the sidebar without requiring the user to sign out and back in

#### Scenario: Collections cache is pre-populated before live fetch
- **WHEN** a cached collections list exists on disk
- **THEN** the sidebar is pre-populated from cache while the live API fetch runs in the background

### Requirement: Catalog auto-load skips the live fetch when the cache is fresh and matches remote count
The catalog load logic SHALL skip the live API fetch when the local cache is non-empty, was saved within the last 7 days, and the remote item count matches the cached item count.

#### Scenario: Fresh cache with matching count skips full fetch
- **WHEN** the catalog cache is non-empty, was written within the last 7 days, and a count-only API call returns the same count as the cached item count
- **THEN** the full paginated catalog fetch is skipped and the cached data is used as-is

#### Scenario: Empty cache triggers full fetch
- **WHEN** the catalog cache is empty or does not exist
- **THEN** the full paginated catalog fetch runs regardless of time since last load

#### Scenario: Stale cache triggers full fetch
- **WHEN** the catalog cache was last written more than 7 days ago
- **THEN** the full paginated catalog fetch runs

#### Scenario: Count mismatch triggers full fetch
- **WHEN** the catalog cache is present and within the staleness window but the remote count differs from the cached item count
- **THEN** the full paginated catalog fetch runs and the cache is updated

### Requirement: Cache metadata is persisted alongside the catalog cache
The catalog cache SHALL be accompanied by a sidecar metadata file recording the timestamp of the last save and the item count at save time.

#### Scenario: Metadata written after successful cache save
- **WHEN** a live catalog fetch completes successfully and is saved to disk
- **THEN** a sidecar metadata file is written with the current timestamp and item count

#### Scenario: Missing metadata treated as stale
- **WHEN** the catalog cache file exists but no sidecar metadata file is present
- **THEN** the cache is treated as stale and a full fetch runs
