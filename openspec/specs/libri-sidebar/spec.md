# libri-sidebar Specification

## Purpose
TBD - created by archiving change implement-libri-ui-in-gpui. Update Purpose after archive.
## Requirements

### Requirement: Sidebar MUST display the Libri wordmark and logo mark
The sidebar MUST render a fixed header region containing a logo mark and the application name "Libri".

#### Scenario: Wordmark is always visible
- **WHEN** the main window is open
- **THEN** the sidebar header shows the logo mark and the text "Libri"

### Requirement: Sidebar MUST provide four smart library section items
The sidebar MUST render a "Library" section group containing four nav items: All Titles, Recently Updated, On This Device, and In the Cloud. Each item MUST display a badge showing the count of library items matching that section's filter.

#### Scenario: All Titles shows the total library count
- **WHEN** the library has been loaded
- **THEN** the "All Titles" nav item badge shows the total number of library items

#### Scenario: Recently Updated shows items added or updated within the configured window
- **WHEN** the library has been loaded
- **THEN** the "Recently Updated" nav item badge shows the count of items whose `date_added` or `date_updated` (whichever is more recent) falls within the user-configured window (default 30 days) of the current time

#### Scenario: Recently Updated window is a bounded, editable user preference
- **WHEN** the user types a value into, or uses the +/- controls on, the "Recently Updated window" field at the top of Settings > Advanced
- **THEN** the value is clamped to between 7 and 90 days, persisted, and takes effect immediately for both the sidebar badge count and the filtered item list without restarting the app

#### Scenario: On This Device shows downloaded item count
- **WHEN** the library has been loaded
- **THEN** the "On This Device" nav item badge shows the count of items whose status is downloaded

#### Scenario: In the Cloud shows cloud-only item count
- **WHEN** the library has been loaded
- **THEN** the "In the Cloud" nav item badge shows the count of items whose status is cloud-only

#### Scenario: Active section item is visually distinguished
- **WHEN** a sidebar nav item is the active filter
- **THEN** the nav item renders in the active/selected visual state and other nav items render in the default state

### Requirement: Sidebar MUST provide a scrollable publisher nav section
The sidebar MUST render a "Publishers" section group containing one nav item per publisher present in the loaded library, ordered by item count descending then name ascending. Each publisher nav item MUST display a badge showing the number of library items from that publisher.

#### Scenario: Publisher list reflects loaded library
- **WHEN** the library has been loaded with items from multiple publishers
- **THEN** the sidebar shows one nav item per publisher with the correct item count badge

#### Scenario: Selecting a publisher filters the catalog
- **WHEN** the user activates a publisher nav item
- **THEN** the catalog shows only items from that publisher and the publisher nav item is in the active state

#### Scenario: Publisher list is scrollable when it overflows
- **WHEN** the publisher list is taller than the available sidebar height
- **THEN** the publisher section scrolls independently without affecting the sidebar header or footer

### Requirement: Sidebar MUST display a storage summary footer
The sidebar MUST render a persistent footer showing the total number of library items and the combined file size of all library items in a human-readable format.

#### Scenario: Footer reflects current library totals
- **WHEN** the library has been loaded
- **THEN** the sidebar footer shows the total item count and the total file size (e.g., "46 titles" and "4.8 GB")
