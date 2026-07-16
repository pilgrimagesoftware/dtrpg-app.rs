## ADDED Requirements

### Requirement: Fresh install is detected before catalog initialization begins
The app SHALL treat catalog initialization as a fresh install whenever no local catalog cache
file, downloaded items, or cached cover/avatar content exists, regardless of whether valid
credentials are present.

#### Scenario: No local catalog data present
- **WHEN** `start_load` runs and finds no catalog cache file, no downloaded items, and no
  cached cover or avatar content
- **THEN** `LibraryController` treats the session as a fresh install and does not render a
  catalog until credentials are available

### Requirement: Fresh-install initialization waits for valid credentials
The app SHALL NOT make any remote-API request for catalog initialization until valid
credentials are available.

#### Scenario: Credentials not yet acquired
- **WHEN** a fresh install is detected and no valid credentials are available
- **THEN** `start_load_inner` does not issue any remote-API request

#### Scenario: Credentials become available
- **WHEN** valid credentials are acquired during a fresh-install session
- **THEN** `start_load_inner` proceeds with catalog initialization

### Requirement: Fresh-install initialization begins with a totals request
The app SHALL make an initial remote-API request to retrieve the user's catalog totals (total
item count and total size) before requesting any page of item data, distinct from the
existing count-only staleness check used by `catalog-auto-load-policy`.

#### Scenario: Totals request precedes item data requests
- **WHEN** fresh-install initialization begins
- **THEN** `start_load_inner` requests catalog totals before requesting the first page of item
  data

#### Scenario: Totals are used to report progress
- **WHEN** the totals request completes
- **THEN** the returned total item count and size are passed to the existing `on_total`
  progress callback used by `catalog-load-progress`

### Requirement: Last request time gates redundant fresh-install requests
The app SHALL persist a "last request time" for catalog initialization requests, alongside the
existing catalog cache metadata, and SHALL NOT issue a new fresh-install request if the
recorded time is within a minimum interval.

#### Scenario: Last request time is recorded after a fresh-install request
- **WHEN** `start_load_inner` makes a catalog initialization request to the remote API
- **THEN** it records the current time as the catalog cache's "last request time"

#### Scenario: Redundant request is skipped
- **WHEN** `start_load` runs and the recorded "last request time" is within the minimum
  interval
- **THEN** fresh-install initialization does not issue a new remote-API request
