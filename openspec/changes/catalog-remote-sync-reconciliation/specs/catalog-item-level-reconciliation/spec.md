## ADDED Requirements

### Requirement: Viewing a catalog entry's details SHALL trigger a single-item server check when not recently checked

The system SHALL check an individual catalog item against the server when the user views
that item's details (via the single-click popover or the expanded detail tab), unless that
item was already checked within a fixed per-item cooldown period. The check SHALL NOT block
the details view from opening or displaying already-known data.

#### Scenario: Viewing an item not recently checked triggers a check
- **WHEN** the user opens the details of a catalog item whose last check was longer ago than
  the per-item cooldown, or has never been checked
- **THEN** a single-item server check for that item starts in the background while the
  details remain visible with the currently-known data

#### Scenario: Viewing an item checked recently does not trigger a redundant check
- **WHEN** the user opens the details of a catalog item that was already checked within the
  per-item cooldown period
- **THEN** no new server check starts for that item

### Requirement: A catalog entry undergoing a single-item check SHALL show a visual checking indicator

The system SHALL display a distinct visual indicator on a catalog entry, in the catalog list
and detail views, for as long as a single-item check for that entry is in progress, and
SHALL remove the indicator when the check completes (successfully or with an error).

#### Scenario: Checking indicator appears while a check is in flight
- **WHEN** a single-item check starts for a catalog entry
- **THEN** that entry displays a checking indicator in the catalog view until the check
  completes

#### Scenario: Checking indicator clears when the check completes
- **WHEN** a single-item check for a catalog entry completes, regardless of outcome
- **THEN** the checking indicator for that entry is removed

### Requirement: A background queue SHALL perform periodic single-item checks across the catalog

The system SHALL maintain a queue of catalog items pending an individual server check and
SHALL process that queue one item at a time in the background, applying the same
availability-update behavior as an on-demand single-item check to each result.

#### Scenario: Queued items are checked one at a time
- **WHEN** the periodic check queue contains more than one item
- **THEN** only one single-item check is in flight at any time, with subsequent items
  checked only after the current one completes

#### Scenario: A queued check updates availability the same way an on-demand check does
- **WHEN** a queued single-item check completes
- **THEN** the item's `is_available` flag and fields are updated per
  `catalog-availability-flag`'s single-item check requirement

### Requirement: Enqueueing a periodic check batch SHALL be available as a manual user action, gated by a cooldown

The system SHALL let the user manually request a periodic check batch, which enqueues catalog
items overdue for an individual check. The system SHALL suppress this request (performing no
enqueueing and no server calls) when the last check batch, manual or automatic, was enqueued
more recently than a fixed batch cooldown period.

#### Scenario: Manual check batch request enqueues overdue items
- **WHEN** the user manually requests a check batch and the last check batch was enqueued
  longer ago than the batch cooldown period, or none has ever been enqueued
- **THEN** catalog items overdue for an individual check are added to the periodic check
  queue

#### Scenario: Manual check batch request within the cooldown is suppressed
- **WHEN** the user manually requests a check batch and the last check batch was enqueued
  more recently than the batch cooldown period
- **THEN** no items are enqueued and no server calls are made

### Requirement: Automatic periodic check batches SHALL be gated by the same cooldown as manual requests

The system SHALL periodically attempt to enqueue a check batch automatically in the
background, without user action, using the same batch cooldown check used for manual
requests, so that automatic and manual triggers cannot together exceed the cooldown's rate
limit.

#### Scenario: Automatic trigger enqueues a batch when the cooldown has elapsed
- **WHEN** the automatic periodic trigger fires and the last check batch was enqueued longer
  ago than the batch cooldown period
- **THEN** catalog items overdue for an individual check are added to the periodic check
  queue

#### Scenario: Automatic trigger is suppressed within the cooldown, including after a recent manual request
- **WHEN** the automatic periodic trigger fires and a manual check batch was enqueued more
  recently than the batch cooldown period
- **THEN** the automatic trigger enqueues nothing
