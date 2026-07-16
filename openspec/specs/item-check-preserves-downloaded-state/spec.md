## ADDED Requirements

### Requirement: A single-item availability check SHALL preserve per-file downloaded state
When a single-item availability check (on-demand or from the periodic
background batch) completes successfully, the system SHALL preserve the
`downloaded` flag of any of the item's files that also appears (matched by
file `id`) in the fresh check response's file list, and the item's
aggregate status SHALL be recomputed from the merged file list rather than
taken from the fresh response.

#### Scenario: A downloaded item's status survives an on-demand availability check
- **WHEN** a catalog item has `status: Downloaded` with all files
  `downloaded: true`, and the user opens its details, triggering a
  single-item check whose response returns the same file ids with the API's
  default `downloaded: false`
- **THEN** after the check completes, the item's files retain
  `downloaded: true` and its status remains `Downloaded`

#### Scenario: A downloaded item's status survives a periodic background check batch
- **WHEN** a catalog item has `status: Downloaded` and is swept into a
  periodic check batch
- **THEN** after the batched check completes, the item's downloaded files
  remain `downloaded: true` and its status remains `Downloaded`

#### Scenario: A fresh file with no downloaded counterpart is not downloaded
- **WHEN** a single-item check's fresh response returns a file id not present
  in the existing item's file list
- **THEN** that file's `downloaded` flag is `false` after the check

### Requirement: The sidebar smart-section counts SHALL stay synchronized after a single-item check
Completing a single-item availability check (on-demand or batched) SHALL
recompute the sidebar's smart-section counts (`SectionCounts`) from the
current catalog, so the section badges never lag the actual filtered result
set.

#### Scenario: On-device count reflects status changes from a check
- **WHEN** a single-item check completes and changes the checked item's
  status (e.g. from `Downloaded` to `Cloud`, or the reverse)
- **THEN** the "On This Device" section count reflects the new status
  immediately, without requiring an unrelated catalog load or download to
  recompute it
