# rust-resource-logging-conventions Specification

## Purpose
TBD - created by archiving change implement-catalog-maintenance-behavior. Update Purpose after archive.
## Requirements
### Requirement: Catalog and cache activity is logged via tracing at an appropriate level
The app SHALL log catalog-sync, cover-cache, and avatar-cache activity — state changes, network
requests, errors, and warnings — via `tracing`, at a level appropriate to the event (`debug!`
for routine activity, `warn!` for recoverable issues, `error!` for failures), consistent with
the existing default-`WARN` filter in `crates/dtrpg-core/src/logging.rs`.

#### Scenario: Routine catalog or cache activity is logged at debug level
- **WHEN** the app performs a routine catalog-sync or cache state change or network request
- **THEN** it logs that activity via `tracing::debug!`

#### Scenario: An error or warning during catalog or cache activity is logged at the matching level
- **WHEN** an error or warning occurs during catalog-sync or cache activity
- **THEN** it is logged via `tracing::warn!` or `tracing::error!` as appropriate to severity

### Requirement: User-facing and internal log messages are distinct
Log messages surfaced to the user (activity panel, toast) SHALL be clear, concise, and free of
internal implementation detail (endpoint paths, status codes, retry reasons); messages recorded
only via `tracing` MAY include that detail.

#### Scenario: A message is shown to the user
- **WHEN** the app surfaces a catalog-sync or cache message to the user
- **THEN** that message is clear, concise, and free of internal implementation detail

#### Scenario: A message is recorded for internal use only
- **WHEN** the app logs a catalog-sync or cache message via `tracing` for internal diagnostic
  use only
- **THEN** that message may include verbose detail not suitable for user-facing display
