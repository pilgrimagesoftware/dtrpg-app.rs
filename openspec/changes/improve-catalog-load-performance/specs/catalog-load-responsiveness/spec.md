## ADDED Requirements

### Requirement: Catalog UI updates are batched during load
While the catalog is loading from the API, the UI SHALL update at most once every 500 milliseconds, regardless of how many API pages have arrived in that interval. Items from all pages received within the interval SHALL appear together in the next batch update.

#### Scenario: Multiple pages received before flush
- **WHEN** two or more API pages arrive within a 500 ms window
- **THEN** the catalog view updates once with all items from those pages combined, not once per page

#### Scenario: Single page in interval still appears
- **WHEN** exactly one API page arrives in a 500 ms window
- **THEN** the catalog view updates with those items at the next flush

#### Scenario: Final batch flushed immediately on load complete
- **WHEN** the last API page arrives and no more pages are coming
- **THEN** the remaining buffered items are added to the catalog immediately, without waiting for the next 500 ms window

### Requirement: Visible items list not recomputed on unchanged state
The filtered and sorted visible items list SHALL be computed at most once per mutation of catalog content, filter, sort method, or search query. Repeated renders of the same state SHALL use the cached result.

#### Scenario: No recomputation on repeated renders
- **WHEN** the controller state has not changed since the last `visible_items()` call
- **THEN** subsequent calls to `visible_items()` return the cached result without filtering or sorting

#### Scenario: Cache invalidated on filter change
- **WHEN** the active sidebar filter changes
- **THEN** the next call to `visible_items()` recomputes the filtered and sorted list

#### Scenario: Cache invalidated on sort change
- **WHEN** the active sort method changes
- **THEN** the next call to `visible_items()` recomputes the sorted list

#### Scenario: Cache invalidated on search query change
- **WHEN** the search query changes
- **THEN** the next call to `visible_items()` recomputes the filtered list

#### Scenario: Cache invalidated on new catalog items
- **WHEN** a batch of new items is appended to the catalog
- **THEN** the next call to `visible_items()` recomputes the list against the full updated catalog
