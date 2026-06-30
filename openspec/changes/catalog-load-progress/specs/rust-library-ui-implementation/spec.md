## ADDED Requirements

### Requirement: Catalog load activity entry SHALL display a progress bar when total is known
When the estimated total item count is available, the catalog load activity entry SHALL carry a numeric progress value so the activity panel can render a progress bar rather than an indeterminate spinner.

#### Scenario: Progress bar visible during catalog load with known total
- **WHEN** the catalog load activity entry has a non-None progress value
- **THEN** the activity panel renders a progress bar for that entry reflecting the current fraction complete

#### Scenario: Spinner visible during catalog load with unknown total
- **WHEN** the catalog load activity entry has a None progress value
- **THEN** the activity panel renders an indeterminate spinner for that entry (existing behavior, no change)
