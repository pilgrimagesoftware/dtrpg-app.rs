## ADDED Requirements

### Requirement: Catalog panel fills all available horizontal space between sidebar and detail
The catalog (multi-item) panel SHALL be a pure flex-fill panel with no independent initial size or size-range constraints of its own. It fills whatever horizontal space remains after the sidebar and detail panel are placed.

#### Scenario: Catalog fills space when detail is hidden
- **WHEN** no item is selected and the detail panel is hidden
- **THEN** the catalog panel spans from the sidebar's right edge to the right window edge with no gap

#### Scenario: Catalog fills remaining space when detail is visible
- **WHEN** an item is selected and the detail panel is visible
- **THEN** the catalog panel fills the space between the sidebar's right edge and the detail panel's left edge

### Requirement: Detail panel takes zero layout width when hidden
The detail (single-item) panel SHALL occupy zero horizontal space when no item is selected; it SHALL NOT leave a gap or placeholder column at the right of the window.

#### Scenario: No layout gap when detail is hidden
- **WHEN** no item is selected
- **THEN** the right edge of the catalog panel aligns with the right edge of the window; no blank column is visible

#### Scenario: Detail panel appears from the right edge when an item is selected
- **WHEN** an item is selected and the detail panel transitions from hidden to visible
- **THEN** the detail panel occupies its configured width from the right side; the catalog compresses accordingly

### Requirement: Catalog has an enforced minimum visible width when detail is present
When the detail panel is visible, the catalog panel SHALL enforce a minimum width of at least 280 px. The detail panel's left-edge handle SHALL not be dragable past the point that would compress the catalog below this minimum.

#### Scenario: Detail handle stops at catalog minimum
- **WHEN** the user drags the detail panel's left handle leftward
- **THEN** the handle stops when the catalog reaches its minimum width; the catalog's right edge remains visible

#### Scenario: Catalog never disappears behind detail panel
- **WHEN** the detail panel is at maximum width
- **THEN** at least 280 px of catalog content remains visible to the left of the detail panel

### Requirement: Resize handles appear only on the left edges of the sidebar and detail panels
The only resize handles in the main layout SHALL be: one on the right edge of the sidebar (= the left edge of the catalog panel, controlling sidebar width) and one on the left edge of the detail panel (controlling detail width). The catalog panel SHALL NOT have an independent handle on its right side.

#### Scenario: Sidebar handle controls sidebar width
- **WHEN** the user drags the handle between the sidebar and catalog
- **THEN** the sidebar width changes and the catalog adjusts to fill remaining space

#### Scenario: Detail handle controls detail width
- **WHEN** the user drags the handle on the detail panel's left edge
- **THEN** the detail panel width changes and the catalog adjusts to fill remaining space
