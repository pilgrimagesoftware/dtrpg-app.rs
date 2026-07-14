## ADDED Requirements

### Requirement: Smart filter navigation

The sidebar SHALL display four smart-filter nav items (All Titles, Recently Added, On This Device, In the Cloud), each with an item count badge and an active highlight that reflects the current `SidebarFilter` selection.

#### Scenario: Filter item shows active state
- **WHEN** the active `SidebarFilter` matches an item
- **THEN** that item is rendered with sidebar accent background and accent foreground colour

#### Scenario: Count badge displays live count
- **WHEN** the sidebar renders
- **THEN** each smart-filter item's suffix shows the current section count from `SectionCounts`

#### Scenario: Clicking a filter item applies the filter
- **WHEN** the user clicks a smart-filter item
- **THEN** `LibraryController::set_filter` is called with the corresponding `SidebarFilter` variant

### Requirement: Collapsible publisher list

The sidebar SHALL display a "Publishers" section that can be expanded and collapsed. When expanded, it lists publisher items, each with a count badge and an active state.

#### Scenario: Publishers section toggles via click
- **WHEN** the user clicks the "Publishers" section header item
- **THEN** the section expands if collapsed, or collapses if expanded, without a round-trip through `LibraryController`

#### Scenario: Publisher item applies publisher filter
- **WHEN** the user clicks a publisher item
- **THEN** `LibraryController::set_filter` is called with `SidebarFilter::Publisher` for that publisher's name

#### Scenario: Collapsed publishers hides individual items
- **WHEN** the publishers section is collapsed
- **THEN** individual publisher items are not visible

### Requirement: Collapsible collection list

The sidebar SHALL display a "Collections" section with the same expand/collapse behaviour as the Publishers section.

#### Scenario: Collections section toggles via click
- **WHEN** the user clicks the "Collections" section header item
- **THEN** the section expands if collapsed, or collapses if expanded

#### Scenario: Collection item applies collection filter
- **WHEN** the user clicks a collection item
- **THEN** `LibraryController::set_filter` is called with `SidebarFilter::Collection` for that collection's name

### Requirement: Sidebar header with wordmark

The sidebar SHALL display a header containing the "Libri" wordmark.

#### Scenario: Header renders wordmark
- **WHEN** the sidebar renders
- **THEN** a `SidebarHeader` containing the "Libri" text is visible at the top of the sidebar

### Requirement: Footer with storage stats and activity button

The sidebar SHALL display a footer showing total title count, total storage used, and an activity status button.

#### Scenario: Footer shows storage summary
- **WHEN** the sidebar renders
- **THEN** the footer shows the total title count and formatted storage size (MB or GB)

#### Scenario: Activity button reflects activity state
- **WHEN** background operations are in progress or have recently completed
- **THEN** the activity button shows a count indicator; clicking it toggles the activity panel via `ActivityController::toggle_panel`
