# refresh-thumbnails-feedback Specification

## Purpose
TBD - created by archiving change fix-refresh-thumbnails-action. Update Purpose after archive.
## Requirements
### Requirement: Refresh Thumbnails action surfaces observable feedback

The "Refresh Thumbnails" catalog menu action SHALL surface start, completion, and no-op
feedback (toast or activity entry) so the user can observe that it ran.

#### Scenario: Refresh starts

- **WHEN** the user selects "Refresh Thumbnails" from the Catalog menu and at least one
  item has a `cover_url`
- **THEN** a notification appears indicating the refresh has started

#### Scenario: Refresh completes

- **WHEN** the refresh queue drains
- **THEN** a notification appears summarizing success/failure counts

#### Scenario: Nothing to refresh

- **WHEN** the user selects "Refresh Thumbnails" and no catalog item has a `cover_url`
- **THEN** a "No thumbnails to refresh" notice appears instead of a silent no-op

