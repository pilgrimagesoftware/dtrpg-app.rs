## ADDED Requirements

### Requirement: Detail panel shows a human-readable "Added" date
When an item has a `date_added` timestamp, the detail panel metadata table SHALL display an "Added" row. The row value SHALL be a relative label. Hovering the value SHALL show a tooltip with the absolute date and time.

#### Scenario: Date added within the last minute
- **WHEN** the item's `date_added` is less than 60 seconds in the past
- **THEN** the "Added" row shows "just now"

#### Scenario: Date added within the last hour
- **WHEN** the item's `date_added` is between 60 seconds and 59 minutes in the past
- **THEN** the "Added" row shows "N minutes ago" (e.g., "5 minutes ago")

#### Scenario: Date added within the last 24 hours
- **WHEN** the item's `date_added` is between 1 hour and 23 hours in the past
- **THEN** the "Added" row shows "N hours ago" (e.g., "3 hours ago")

#### Scenario: Date added yesterday
- **WHEN** the item's `date_added` is between 24 and 47 hours in the past
- **THEN** the "Added" row shows "yesterday"

#### Scenario: Date added within the last week
- **WHEN** the item's `date_added` is between 2 days and 6 days in the past
- **THEN** the "Added" row shows "N days ago" (e.g., "4 days ago")

#### Scenario: Date added within the last 30 days
- **WHEN** the item's `date_added` is between 7 days and 29 days in the past
- **THEN** the "Added" row shows "N weeks ago" (e.g., "2 weeks ago")

#### Scenario: Date added within the current year
- **WHEN** the item's `date_added` is 30 or more days in the past but within the current calendar year
- **THEN** the "Added" row shows the abbreviated month and day (e.g., "Jan 5")

#### Scenario: Date added in a previous year
- **WHEN** the item's `date_added` is in a previous calendar year
- **THEN** the "Added" row shows the abbreviated month, day, and year (e.g., "Jan 5, 2023")

#### Scenario: Tooltip shows absolute date and time
- **WHEN** the user hovers the "Added" row value
- **THEN** a tooltip appears showing the full date and time in the format "Month D, YYYY at H:MM AM/PM" (e.g., "January 5, 2024 at 3:42 PM")

#### Scenario: No date available
- **WHEN** the item's `date_added` is `None`
- **THEN** no "Added" row is displayed in the metadata table

### Requirement: `format_relative` is unit-tested
The `util::datetime::format_relative` function SHALL be covered by unit tests verifying each relative label bucket boundary.

#### Scenario: Boundary tests cover all label transitions
- **WHEN** `format_relative` is called with timestamps at the boundary of each relative bucket (59s, 60s, 59m, 1h, 23h, 24h, 47h, 48h, 6d, 7d, 29d, 30d)
- **THEN** each boundary produces the expected label without off-by-one errors
