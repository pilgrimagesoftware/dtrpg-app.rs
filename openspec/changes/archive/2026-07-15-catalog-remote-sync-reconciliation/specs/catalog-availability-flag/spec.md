## ADDED Requirements

### Requirement: Catalog items SHALL carry a server-availability flag

Each catalog item SHALL carry an `is_available` flag indicating whether the item was present
in the most recent successful live catalog fetch. The flag SHALL default to available for
items that predate this flag (e.g. cache files written before this change) and for items
newly added from a live fetch.

#### Scenario: New item from a live fetch is marked available
- **WHEN** an item is added to the catalog because it appeared in a live fetch response but
  had no local counterpart
- **THEN** the item's `is_available` flag is `true`

#### Scenario: Pre-existing cache file without the flag loads as available
- **WHEN** a catalog cache file written before this flag existed is loaded from disk
- **THEN** every item it contains loads with `is_available` set to `true`

### Requirement: Items missing from a reconciled live fetch SHALL be flagged unavailable, not removed

The system SHALL keep a local item in the catalog with `is_available` set to `false`, and
SHALL NOT delete it from the catalog or the disk cache, when the catalog is reconciled
against a live fetch (per `catalog-live-data-swap`) and that item's id does not appear in
the live response.

#### Scenario: Server no longer lists a previously-cached item
- **WHEN** a reconciliation runs and a local item's id is absent from the live fetch results
- **THEN** the item remains in the catalog afterward with `is_available` set to `false`

#### Scenario: Unavailable items are persisted to disk
- **WHEN** the catalog is saved to disk after a reconciliation that flagged one or more
  items unavailable
- **THEN** the saved cache file includes those items with `is_available` set to `false`

### Requirement: Items reappearing in a live fetch SHALL have their unavailable flag cleared

The system SHALL reset a local item's `is_available` flag to `true` and refresh its other
fields from the live data whenever the catalog is reconciled against a live fetch and that
item, previously marked `is_available = false`, has an id that appears in the live response.

#### Scenario: Previously-flagged item reappears in a live fetch
- **WHEN** a reconciliation runs and a local item with `is_available = false` has an id
  present in the live fetch results
- **THEN** the item's `is_available` flag becomes `true` after reconciliation

### Requirement: Unavailable items SHALL remain visible in the catalog UI with a distinguishing indicator

Items flagged `is_available = false` SHALL continue to appear in catalog list, grid, and
search/filter views by default, visually distinguished from available items.

#### Scenario: Unavailable item appears in the catalog list
- **WHEN** the catalog contains an item with `is_available = false`
- **THEN** the item is rendered in the catalog list view with a visible unavailable
  indicator, not hidden

#### Scenario: Unavailable item appears in search results
- **WHEN** a search or filter query matches an item with `is_available = false`
- **THEN** the item appears in the results, consistent with available items

### Requirement: A single-item check SHALL set the availability flag based on the item's individual server response

The system SHALL set an item's `is_available` flag to `false` when an individual server
check for that item (see `catalog-item-level-reconciliation`) returns a not-found response,
and SHALL set it to `true` and refresh the item's other fields when the check returns the
item successfully. The system SHALL leave the flag unchanged when the check fails for any
other reason (network error, session error, or any error other than not-found).

#### Scenario: Single-item check confirms the item still exists
- **WHEN** an individual server check for a catalog item succeeds
- **THEN** the item's `is_available` flag is set to `true` and its other fields are
  refreshed from the response

#### Scenario: Single-item check reports the item is gone
- **WHEN** an individual server check for a catalog item returns a not-found response
- **THEN** the item's `is_available` flag is set to `false` and the item is not removed
  from the catalog

#### Scenario: Single-item check fails transiently
- **WHEN** an individual server check for a catalog item fails with a network or session
  error rather than a not-found response
- **THEN** the item's `is_available` flag and other fields remain unchanged

### Requirement: A partial date-filtered fetch SHALL NOT flag any item unavailable

The system SHALL NOT modify `is_available` on any item absent from a partial date-filtered
fetch, since that response is never a complete listing of the server's catalog. Items
present in a partial response SHALL be merged the same as in a full reconciliation (added if
absent locally, refreshed and set `is_available = true` if already present).

#### Scenario: Item absent from a partial fetch keeps its existing availability flag
- **WHEN** a partial date-filtered fetch completes and a local item's id does not appear in
  the response
- **THEN** that item's `is_available` flag is unchanged by the partial fetch

#### Scenario: Item present in a partial fetch is added or refreshed as available
- **WHEN** a partial date-filtered fetch returns an item, whether or not it previously
  existed locally
- **THEN** the item is present in the catalog afterward with `is_available` set to `true`
  and its fields refreshed from the response
