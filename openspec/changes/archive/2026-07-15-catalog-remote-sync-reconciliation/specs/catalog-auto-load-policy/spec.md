## MODIFIED Requirements

### Requirement: Catalog auto-load chooses between skipping, a partial fetch, or a full fetch based on a remote count comparison

The catalog load logic SHALL skip the live API fetch when the local cache is non-empty, was
saved within the last 7 days, and the remote item count matches the cached item count. When
the remote item count is greater than the cached item count, the catalog load logic SHALL
perform a partial date-filtered fetch (see `catalog-availability-flag`'s partial-fetch
requirement) instead of a full paginated fetch, if the service supports it. When the remote
item count is less than the cached item count, or a partial fetch is unsupported, or no cache
exists, the catalog load logic SHALL perform a full paginated catalog fetch.

#### Scenario: Fresh cache with matching count skips full fetch
- **WHEN** the catalog cache is non-empty, was written within the last 7 days, and a
  count-only API call returns the same count as the cached item count
- **THEN** the full paginated catalog fetch is skipped and the cached data is used as-is

#### Scenario: Empty cache triggers full fetch
- **WHEN** the catalog cache is empty or does not exist
- **THEN** the full paginated catalog fetch runs regardless of time since last load

#### Scenario: Stale cache triggers a count check rather than an unconditional full fetch
- **WHEN** the catalog cache was last written more than 7 days ago
- **THEN** the remote count comparison still runs to choose between a partial and a full
  fetch, rather than always performing a full fetch

#### Scenario: Remote count greater than cached count triggers a partial fetch
- **WHEN** the catalog cache is present, a count-only API call returns a count greater than
  the cached item count, and the service supports a date-filtered partial fetch
- **THEN** a partial fetch runs instead of a full paginated fetch, and its results are
  merged additively into the cache

#### Scenario: Remote count less than cached count triggers a full fetch
- **WHEN** the catalog cache is present and a count-only API call returns a count less than
  the cached item count
- **THEN** the full paginated catalog fetch runs and the result is reconciled against the
  cache

#### Scenario: Partial fetch unsupported falls back to a full fetch
- **WHEN** a partial fetch would otherwise run but the underlying service does not support
  the date-filtered fetch
- **THEN** the full paginated catalog fetch runs instead

## ADDED Requirements

### Requirement: User-requested full reloads SHALL be gated by a cooldown independent of the passive staleness window

The application SHALL track the timestamp of the last successfully completed catalog load
and SHALL suppress a user-requested full reload (bypassing the network fetch entirely,
leaving the catalog and cache unchanged) when that timestamp is more recent than a fixed
cooldown period. This cooldown is independent of the 7-day passive cache-staleness check and
applies only to the user-initiated reload action.

#### Scenario: Reload requested shortly after the last successful load is suppressed
- **WHEN** the user triggers the full-reload action and the last successful catalog load
  completed less than the cooldown period ago
- **THEN** no live fetch is started and the catalog remains unchanged

#### Scenario: Reload requested after the cooldown has elapsed proceeds
- **WHEN** the user triggers the full-reload action and the last successful catalog load
  completed at or before the cooldown period ago (or no successful load timestamp exists)
- **THEN** a full live fetch runs and its result is reconciled against the existing local
  catalog by item id

#### Scenario: A reload that proceeds past the cooldown still bypasses the passive freshness/count-match skip
- **WHEN** a user-requested full reload proceeds past the cooldown check
- **THEN** the live fetch runs unconditionally, even if the cache would otherwise be
  considered fresh and count-matching under the passive auto-load skip
