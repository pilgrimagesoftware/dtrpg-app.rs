# activity-panel-button-legibility Specification

## Purpose
TBD - created by archiving change activity-panel-button-icon-size. Update Purpose after archive.
## Requirements
### Requirement: Activity button symbols render at small text size
The activity panel button's symbol and count text SHALL render at `text_sm` size (not `text_xs`) so the `↻`, `●`, and `○` glyphs are legible at a glance.

#### Scenario: Idle state symbol is readable
- **WHEN** no downloads are in progress and no recent activity exists
- **THEN** the `○` symbol in the sidebar activity button is rendered at `text_sm` size

#### Scenario: Active state symbol and count are readable
- **WHEN** downloads are in progress
- **THEN** the `↻ (N)` text in the activity button is rendered at `text_sm` size

