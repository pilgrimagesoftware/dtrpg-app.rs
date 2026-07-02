# alphabetical-publisher-list Specification

## Purpose
TBD - created by archiving change sort-publisher-list. Update Purpose after archive.
## Requirements
### Requirement: Publisher list is sorted alphabetically
The `publisher_entries` function SHALL return entries sorted by publisher name in
case-insensitive ascending order (A → Z). The grouped catalog view, which uses
`publisher_entries` for its ordering, SHALL reflect the same sequence.

#### Scenario: Single publisher
- **WHEN** the catalog contains items from one publisher
- **THEN** that publisher SHALL appear as the only entry

#### Scenario: Multiple publishers ordered correctly
- **WHEN** the catalog contains items from publishers "Wizards of the Coast", "Paizo", and "Kobold Press"
- **THEN** the publisher list SHALL be ordered: "Kobold Press", "Paizo", "Wizards of the Coast"

#### Scenario: Case-insensitive comparison
- **WHEN** the catalog contains publishers "a publisher" and "B Publisher"
- **THEN** "a publisher" SHALL appear before "B Publisher"

#### Scenario: Alphabetical order is independent of item count
- **WHEN** publisher "Zyborg Games" has 50 items and "Aaeon Press" has 1 item
- **THEN** "Aaeon Press" SHALL appear before "Zyborg Games"

