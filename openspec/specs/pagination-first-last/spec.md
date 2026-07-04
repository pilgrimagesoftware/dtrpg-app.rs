# pagination-first-last Specification

## Purpose
TBD - created by archiving change ui-layout-fixes. Update Purpose after archive.
## Requirements
### Requirement: Pagination bar includes First and Last buttons
The pagination bar SHALL render a "First" button before the page picker and a "Last" button after it. Both buttons SHALL be disabled when the user is already on the first or last page respectively.

#### Scenario: First button navigates to page 1
- **WHEN** the user is on any page other than 1
- **THEN** clicking "First" sets the current page to 1

#### Scenario: Last button navigates to the final page
- **WHEN** the user is on any page other than the last page
- **THEN** clicking "Last" sets the current page to `total_pages`

#### Scenario: First button is disabled on page 1
- **WHEN** `current_page == 1`
- **THEN** the "First" button is rendered in a disabled state and clicking it has no effect

#### Scenario: Last button is disabled on last page
- **WHEN** `current_page == total_pages`
- **THEN** the "Last" button is rendered in a disabled state and clicking it has no effect

