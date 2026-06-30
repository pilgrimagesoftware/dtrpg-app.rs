## MODIFIED Requirements

### Requirement: Rust sync and thumbnail loading MUST be non-blocking
The Rust desktop app MUST keep background library sync and thumbnail loading from blocking main-window interaction. The number of concurrent background fetch operations (thumbnail loads and file downloads combined) MUST be bounded by a user-configurable limit, defaulting to 3, so that aggressive fetching does not degrade UI responsiveness.

#### Scenario: Syncing or loading thumbnails in Rust
- **WHEN** the Rust app syncs library metadata or resolves grid thumbnails
- **THEN** the user can continue interacting with library controls and visible title/size metadata

#### Scenario: Concurrency limit enforced during heavy load
- **WHEN** more thumbnail fetches or downloads are pending than the configured limit allows
- **THEN** the excess requests wait in their respective queues and the main window remains responsive
