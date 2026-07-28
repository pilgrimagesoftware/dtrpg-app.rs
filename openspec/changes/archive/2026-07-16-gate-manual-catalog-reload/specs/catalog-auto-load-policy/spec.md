## ADDED Requirements

### Requirement: User-initiated reload shares the freshness policy
The "Catalog > Reload" menu action (see `catalog-menu`) SHALL be subject to the same freshness policy defined above (cache non-empty, saved within 7 days, remote count matches) rather than unconditionally forcing a full paginated fetch. The existing 60-second double-click throttle (`FORCE_RELOAD_COOLDOWN_SECS`) SHALL continue to gate the action independently of this policy.

#### Scenario: Manual reload skips the live fetch when the cache is fresh
- **WHEN** the user selects "Catalog > Reload" and the catalog cache is non-empty, was written within the last 7 days, and a count-only API call returns the same count as the cached item count
- **THEN** the full paginated catalog fetch is skipped, mirroring the passive-load skip-fetch behavior

#### Scenario: Manual reload runs a live fetch when the cache is stale
- **WHEN** the user selects "Catalog > Reload" and the catalog cache is stale, empty, or the remote count no longer matches
- **THEN** the full paginated catalog fetch runs

#### Scenario: Double-click throttle still applies independently
- **WHEN** the user selects "Catalog > Reload" twice within `FORCE_RELOAD_COOLDOWN_SECS` of each other
- **THEN** the second invocation is a silent no-op, regardless of what the freshness policy would otherwise decide
