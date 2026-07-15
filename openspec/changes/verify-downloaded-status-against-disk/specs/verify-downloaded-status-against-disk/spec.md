## ADDED Requirements

### Requirement: A catalog load SHALL verify every file's downloaded status against disk
After a catalog load settles into its final displayed state — whether the
live fetch was skipped because the cache was fresh, a partial fetch merged
in updates, or a full fetch reconciled the entire catalog — the system SHALL
verify each item's files against the filesystem and set each file's
`downloaded` flag to whether a file actually exists at its expected on-disk
path, regardless of what the flag previously said.

#### Scenario: A file present on disk is marked downloaded even if the flag said otherwise
- **WHEN** a catalog load settles and an item's file exists at its expected
  on-disk path but that file's `downloaded` flag was `false`
- **THEN** after verification, the file's `downloaded` flag is `true` and
  the item's status is recomputed accordingly

#### Scenario: A file missing from disk is marked not-downloaded even if the flag said otherwise
- **WHEN** a catalog load settles and an item's file does not exist at its
  expected on-disk path but that file's `downloaded` flag was `true`
- **THEN** after verification, the file's `downloaded` flag is `false` and
  the item's status is recomputed accordingly

#### Scenario: Verification runs even when the live fetch is skipped
- **WHEN** the auto-load policy skips the live fetch because the on-disk
  catalog cache is fresh and the remote count matches
- **THEN** file-presence verification still runs against the cache's
  contents before the load is considered complete

### Requirement: Selecting an item SHALL verify that item's files against disk on demand
When the user selects a catalog item (the single-click popover path),
the system SHALL verify that specific item's files against disk in the
background, independent of and in addition to the existing per-item
availability check, so an external change to that item's files is reflected
without waiting for the next catalog load.

#### Scenario: Selecting an item with an externally deleted file updates its status
- **WHEN** the user selects an item whose file was deleted outside the app
  since the last verification
- **THEN** the item's status updates to reflect the file's absence without
  requiring a full catalog reload

### Requirement: Section counts SHALL stay synchronized after a verification pass
Completing a file-presence verification pass (catalog-wide or single-item)
that changes any item's status SHALL recompute the sidebar's smart-section
counts (`SectionCounts`) from the current catalog.

#### Scenario: On-device count reflects a verification-driven status change
- **WHEN** a verification pass changes an item's status between
  `Downloaded` and `Cloud`
- **THEN** the "On This Device" section count reflects the change
  immediately
