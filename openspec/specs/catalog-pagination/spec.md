# catalog-pagination Specification

## Purpose
TBD - created by archiving change adopt-gpui-component-primitives. Update Purpose after archive.

## Requirements

### Requirement: Catalog view is paginated

The catalog view SHALL display items in pages rather than as a single scrollable list. The active page and page size are owned by `LibraryController` and apply to all three presentation modes (list, thumbs, grid).

#### Scenario: First page shows first N items
- **WHEN** the user opens the catalog with `page_size` set to N and `current_page` set to 1
- **THEN** only the first N items from the filtered, sorted result set are visible

#### Scenario: Navigating to a different page updates content
- **WHEN** the user clicks a page number or next/prev in the `Pagination` component
- **THEN** `LibraryController::set_page` is called and the catalog shows the corresponding slice

#### Scenario: Changing the filter or search resets to page 1
- **WHEN** the user changes the sidebar filter or search query
- **THEN** `current_page` is reset to 1 so the first result page is shown

### Requirement: Page size is user-configurable

The user SHALL be able to choose a page size of 10, 25, 50, 100, or 200 items per page via a picker in the pagination bar.

#### Scenario: Selecting a page size updates visible item count
- **WHEN** the user selects a page size from the picker
- **THEN** `LibraryController::set_page_size` is called, `current_page` resets to 1, and the catalog shows at most that many items

#### Scenario: Page size choice persists across sessions
- **WHEN** the app relaunches
- **THEN** the previously selected page size is restored from persistent settings

### Requirement: Pagination bar shows total pages and current position

The pagination bar SHALL display a `Pagination` component that indicates the current page, total page count, and allows navigation to any page.

#### Scenario: Total pages reflects current filter result count
- **WHEN** the filtered result set has N items and page size is P
- **THEN** total pages equals ceil(N / P)

#### Scenario: Pagination bar is hidden when all items fit on one page
- **WHEN** the total filtered item count is less than or equal to the selected page size
- **THEN** the pagination bar is not shown
