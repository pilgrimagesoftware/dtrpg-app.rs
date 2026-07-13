## Requirements

### Requirement: Status bar activity indicator is a circular progress element

The system SHALL render the status bar's activity indicator as a `ProgressCircle`
reflecting the aggregate completion of all active background loaders, instead of a text
glyph and count.

#### Scenario: One or more loaders in progress with known totals

- **WHEN** at least one background loader (catalog load, download) is in progress and
  reports a total
- **THEN** the circle fills proportionally to the aggregate completed/total ratio

#### Scenario: Loader in progress with no known total

- **WHEN** a loader without a known total (e.g. the thumbnail queue) is in progress
- **THEN** the circle shows an indeterminate spinning state

#### Scenario: No activity

- **WHEN** there are no in-progress or recent activity items
- **THEN** the circle renders in an idle, non-animated state

#### Scenario: Clicking the indicator opens the activity panel

- **WHEN** the user clicks the activity indicator
- **THEN** the activity panel toggles open, unchanged from current behavior
