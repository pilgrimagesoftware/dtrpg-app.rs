# rust-library-ui-implementation Specification

## Purpose
TBD - created by archiving change implement-rust-library-baseline-ui-stubs. Update Purpose after archive.
## Requirements
### Requirement: Rust frontend baseline implementation MUST satisfy shared desktop library layout behavior
The Rust frontend MUST implement the shared desktop library baseline layout and interaction behavior defined by the app meta-repository.

#### Scenario: Rendering baseline library layout in Rust
- **WHEN** the Rust app renders the library screen in baseline mode
- **THEN** it presents the shared top-level layout regions and interactions defined by shared app specs

### Requirement: Rust baseline implementation MUST use stubbed backend adapters
The Rust frontend baseline implementation MUST keep backend communication stubbed while exercising list/detail/filter/refresh flows.

#### Scenario: Loading library data in Rust baseline mode
- **WHEN** the Rust frontend loads or refreshes library content in baseline phase
- **THEN** it uses stubbed adapters and no live backend SDK calls

### Requirement: Catalog load activity entry SHALL display a progress bar when total is known
When the estimated total item count is available, the catalog load activity entry SHALL carry a numeric progress value so the activity panel can render a progress bar rather than an indeterminate spinner.

#### Scenario: Progress bar visible during catalog load with known total
- **WHEN** the catalog load activity entry has a non-None progress value
- **THEN** the activity panel renders a progress bar for that entry reflecting the current fraction complete

#### Scenario: Spinner visible during catalog load with unknown total
- **WHEN** the catalog load activity entry has a None progress value
- **THEN** the activity panel renders an indeterminate spinner for that entry (existing behavior, no change)

