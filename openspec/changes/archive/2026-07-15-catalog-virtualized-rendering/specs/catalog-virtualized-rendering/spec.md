## ADDED Requirements

### Requirement: Catalog renders only visible items
The catalog view SHALL render only the items whose rows are currently visible in the scroll viewport. Items scrolled out of view SHALL NOT be present as GPUI elements in the current render. This applies to list layout and thumbs layout. The number of GPUI element nodes for the catalog list SHALL be proportional to the number of visible rows, not the total item count.

#### Scenario: Large catalog does not degrade frame performance
- **WHEN** the catalog contains 1 000 or more items
- **THEN** the GPUI element count for the catalog list is bounded by the viewport height divided by the row height, not by the total item count

#### Scenario: Scrolling reveals new items
- **WHEN** the user scrolls the catalog list
- **THEN** items that scroll into the viewport are rendered and items that scroll out of view are removed from the render tree

#### Scenario: All items remain accessible via scroll
- **WHEN** the catalog contains N items
- **THEN** the user can scroll to reach any item from item 1 to item N

### Requirement: Grid layout renders only visible rows
The catalog grid view SHALL render only the rows of cards currently visible in the scroll viewport. Each row contains multiple cards. Items whose row is scrolled out of view SHALL NOT be present as GPUI elements.

#### Scenario: Grid with many items does not render all rows
- **WHEN** the catalog contains more items than fit in the visible grid area
- **THEN** only the rows of cards visible in the viewport are rendered as GPUI elements

#### Scenario: Grid items are accessible via scroll
- **WHEN** the user scrolls the grid view
- **THEN** card rows that scroll into the viewport are rendered and card rows that scroll out of view are removed

### Requirement: Grouped layout retains full rendering
The catalog view in grouped-by-publisher mode SHALL render all items and group headers regardless of scroll position. Virtualization is NOT applied to grouped mode.

#### Scenario: Grouped mode shows all items
- **WHEN** the catalog is in grouped layout mode
- **THEN** all publisher group headers and all item rows are present in the render tree, including those scrolled out of view

### Requirement: Catalog items remain interactive in virtualized layouts
In list and thumbs layouts using virtualized rendering, each rendered item SHALL support the same interactions as the non-virtualized implementation: clicking an item opens its detail view, and a context menu is accessible on each item.

#### Scenario: Item click in virtualized list opens detail
- **WHEN** the user clicks an item in the virtualized list layout
- **THEN** the item detail view opens, identical to the behavior in non-virtualized mode

#### Scenario: Item click in virtualized thumbs opens detail
- **WHEN** the user clicks an item in the virtualized thumbs layout
- **THEN** the item detail view opens

#### Scenario: Context menu available in virtualized layouts
- **WHEN** the user right-clicks or long-presses an item in list or thumbs layout
- **THEN** the context menu appears with the same options as in non-virtualized mode
