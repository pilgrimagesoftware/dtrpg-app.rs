# catalog-cache-checkpointing Specification

## Purpose
Protects against data loss from an interrupted catalog load by periodically checkpointing
the accumulating page buffer to disk during a live fetch, independent of the one-time
post-fetch save.
## Requirements
### Requirement: Catalog cache is checkpointed periodically during a live load

The system SHALL periodically write the accumulating catalog page buffer to disk during a
live load, in addition to the existing save after the full fetch completes, so an
interrupted load still leaves a recent cache on disk.

#### Scenario: App quits mid-load

- **WHEN** the app is closed after several checkpoints have been written but before the
  live fetch completes
- **THEN** the next startup loads the most recent checkpointed cache instead of an empty
  or fully stale one

#### Scenario: Load completes normally

- **WHEN** a live load completes successfully
- **THEN** the final on-disk cache matches the complete fetched dataset, not a stale
  intermediate checkpoint

#### Scenario: Checkpoint write is atomic

- **WHEN** a checkpoint write is interrupted (e.g. by a crash)
- **THEN** the existing cache file on disk is left intact rather than corrupted, per the
  existing `.tmp`-then-rename write pattern
