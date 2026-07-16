## MODIFIED Requirements

### Requirement: Catalog load SHALL report estimated total item count
Before delivering any catalog items to the UI, the catalog load pipeline SHALL determine the
total item count either from a dedicated fresh-install totals request (see
`rust-catalog-fresh-install-initialization`) or, when that request has not been made, by
parsing the `last` page URL from the first API page response. If neither source yields a
count, the count SHALL be treated as unknown and progress SHALL remain indeterminate.

#### Scenario: Total count derived from a fresh-install totals request
- **WHEN** fresh-install initialization has issued its totals request and received a total
  item count
- **THEN** the system uses that count as `estimated_total` and makes it available to the
  progress tracker before requesting any page of item data

#### Scenario: Total count derived from first page
- **WHEN** no fresh-install totals request applies and the first page of library items is
  received with a `links.last` URL containing a page number parameter
- **THEN** the system computes `estimated_total = last_page_number * items_per_page` and makes
  it available to the progress tracker

#### Scenario: Total count unknown when no source is available
- **WHEN** no fresh-install totals request applies and the first page response has no
  `links.last` URL or the URL has no parseable page number
- **THEN** the system treats the total as unknown and the progress indicator remains
  indeterminate

### Requirement: Catalog load SHALL update activity panel progress after each page
The catalog load activity entry SHALL update its progress value after each page of items is
received. Progress SHALL be computed as `items_loaded / estimated_total`, clamped to the range
[0.0, 1.0].

#### Scenario: Progress increments per page
- **WHEN** a page of N items arrives during catalog load and the estimated total is T
- **THEN** the activity entry progress is updated to `min(items_loaded / T, 1.0)` and the
  activity panel reflects the new value

#### Scenario: Progress reaches 100% on completion
- **WHEN** the final page has been received and the fetch task completes successfully
- **THEN** the activity entry is resolved as complete, which clears the progress bar

### Requirement: Catalog load progress SHALL NOT regress
The reported progress value SHALL be non-decreasing. If a computed progress value is less than
or equal to the previously reported value, the update SHALL be skipped.

#### Scenario: Progress is monotonically increasing
- **WHEN** successive pages arrive
- **THEN** each progress update is greater than the previous one

### Requirement: Catalog load progress is indeterminate when total is unknown
If the total item count could not be determined, the activity entry SHALL show an
indeterminate spinner rather than a 0% progress bar.

#### Scenario: Indeterminate state when no total
- **WHEN** catalog load starts and no total count is available
- **THEN** the activity entry has `progress = None` and the panel renders a spinner
