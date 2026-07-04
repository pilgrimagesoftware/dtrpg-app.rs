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

#### Scenario: Date added within the last month
- **WHEN** the item's `date_added` is 30 or more days in the past but less than 12 calendar months in the past
- **THEN** the "Added" row shows "N months ago" (e.g., "5 months ago")

#### Scenario: Date added a year or more in the past
- **WHEN** the item's `date_added` is 12 or more calendar months in the past
- **THEN** the "Added" row shows "N years ago" (e.g., "2 years ago")

#### Scenario: Tooltip shows absolute date and time
- **WHEN** the user hovers the "Added" row value
- **THEN** a tooltip appears showing the full date and time in the format "Month D, YYYY at H:MM AM/PM" (e.g., "January 5, 2024 at 3:42 PM") — never a raw RFC 3339 / ISO 8601 string

#### Scenario: No date available
- **WHEN** the item's `date_added` is `None`
- **THEN** no "Added" row is displayed in the metadata table

### Requirement: Detail panel shows a human-readable "Updated" date
When an item has a `date_updated` timestamp (the API's `fileLastModified`), the detail panel metadata table SHALL display an "Updated" row using the same relative-label-plus-tooltip presentation as the "Added" row.

#### Scenario: Updated row mirrors the Added row's formatting
- **WHEN** the item's `date_updated` is set
- **THEN** the "Updated" row shows a relative label (e.g., "Updated" label with value "5 months ago") and hovering the value shows a tooltip with the absolute date and time

#### Scenario: No update date available
- **WHEN** the item's `date_updated` is `None`
- **THEN** no "Updated" row is displayed in the metadata table

### Requirement: SDK adapter never surfaces raw timestamp strings
The Rust SDK adapter (`services::sdk::map_order_product`) SHALL parse the API's `datePurchased`/`fileLastModified` RFC 3339 strings into `date_added`/`date_updated` Unix timestamps rather than embedding the raw strings in `LibraryItem.desc` or any other user-facing text.

#### Scenario: Description never contains a raw timestamp
- **WHEN** an ordered product has `datePurchased` and/or `fileLastModified` set
- **THEN** `LibraryItem.desc` does not contain either raw value, and the parsed timestamps are available on `date_added` / `date_updated` instead

### Requirement: `format_relative` is unit-tested
The `util::datetime::format_relative` function SHALL be covered by unit tests verifying each relative label bucket boundary.

#### Scenario: Boundary tests cover all label transitions
- **WHEN** `format_relative` is called with timestamps at the boundary of each relative bucket (59s, 60s, 59m, 1h, 23h, 24h, 47h, 48h, 6d, 7d, 29d, 1 month, 11 months, 12 months, 25 months)
- **THEN** each boundary produces the expected label without off-by-one errors, using calendar month/year arithmetic rather than fixed day-count division
