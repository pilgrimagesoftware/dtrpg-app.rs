## MODIFIED Requirements

### Requirement: Catalog loads are protected against being superseded
When a new catalog load is started (via reload, cache clear, or startup), any earlier catalog load still in flight SHALL NOT be allowed to overwrite the catalog, collections, or loading-state changes made by the newer load once it starts.

#### Scenario: Superseded load does not overwrite a fresh reload
- **WHEN** a catalog load is in flight and a new load is started (e.g. via "Clear Cache") before the first one completes
- **THEN** the first load's eventual completion does not modify the catalog, and only the newer load's result is reflected

#### Scenario: Queued thumbnail fetches are dropped on cache clear
- **WHEN** "Clear Cache" is triggered while thumbnail fetches are queued but not yet started
- **THEN** those queued fetches are removed from the queue and their in-flight markers are cleared, so they do not populate `CoverCache` for a catalog that was just cleared

### Requirement: Expected pre-auth session errors do not raise a user alert
When a catalog load fails because no authenticated session is available yet (expected at startup before sign-in completes), the load SHALL be treated as a quiet completion rather than raising a user-facing alert. Other load failures continue to raise an alert as before.

#### Scenario: Session error at startup produces no alert
- **WHEN** a catalog load attempt fails with `LibraryServiceErrorKind::Session` before authentication has completed
- **THEN** the load's activity entry is marked complete and no entry appears in the alert history

#### Scenario: Non-session load failures still raise an alert
- **WHEN** a catalog load fails for a reason other than `LibraryServiceErrorKind::Session` (e.g. a network error)
- **THEN** the load's activity entry is marked as an error and an alert appears in the alert history, as before
