## MODIFIED Requirements

### Requirement: Catalog auto-load skips the live fetch when the cache is fresh and matches remote count
The catalog load logic SHALL skip the live API fetch when the local cache is non-empty, was
saved within the last 7 days, and the remote item count matches the cached item count, or when
a cache-control header or update query parameter from the remote API indicates the cached data
is still current, or when a recurring long-running-session timer check finds the resource is
not yet stale.

#### Scenario: Fresh cache with matching count skips full fetch
- **WHEN** the catalog cache is non-empty, was written within the last 7 days, and a count-only
  API call returns the same count as the cached item count
- **THEN** the full paginated catalog fetch is skipped and the cached data is used as-is

#### Scenario: Empty cache triggers full fetch
- **WHEN** the catalog cache is empty or does not exist
- **THEN** the full paginated catalog fetch runs regardless of time since last load

#### Scenario: Stale cache triggers full fetch
- **WHEN** the catalog cache was last written more than 7 days ago
- **THEN** the full paginated catalog fetch runs

#### Scenario: Count mismatch triggers full fetch
- **WHEN** the catalog cache is present and within the staleness window but the remote count
  differs from the cached item count
- **THEN** the full paginated catalog fetch runs and the cache is updated

#### Scenario: Cache-control signal indicates newer remote data
- **WHEN** the app checks the remote API's cache-control header or update query parameter and
  it indicates the remote data is newer than the cached data
- **THEN** the full paginated catalog fetch runs, even if the 7-day staleness window has not
  elapsed

#### Scenario: Long-running-session timer finds the resource not yet stale
- **WHEN** the recurring long-running-session timer checks the catalog cache and neither the
  staleness window nor a cache-control signal indicates a refresh is due
- **THEN** the catalog load logic does not trigger a fetch

## ADDED Requirements

### Requirement: A recurring timer triggers staleness checks during a long-running session
`LibraryController` SHALL maintain a recurring timer, independent of the startup sequence,
that re-runs the staleness check defined above and triggers a catalog fetch if the staleness
window has elapsed or a cache-control signal indicates newer remote data.

#### Scenario: Long-running session reaches the staleness threshold
- **WHEN** the app has been running long enough that the 7-day staleness window has elapsed
  since the last catalog load, without the app having been restarted
- **THEN** the recurring timer triggers a catalog fetch

#### Scenario: Long-running session has not yet reached the staleness threshold
- **WHEN** the recurring timer fires but neither the staleness window nor a cache-control
  signal indicates a refresh is due
- **THEN** the app does not fetch the catalog
