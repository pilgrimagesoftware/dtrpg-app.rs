## MODIFIED Requirements

### Requirement: Catalog pane fills remaining horizontal space without a resize handle
The catalog pane SHALL occupy all horizontal space between the sidebar and the right window edge. There SHALL be no resize splitter between the catalog and the detail panel. The detail panel SHALL be an overlay or fixed-width panel that does not reduce catalog width.

#### Scenario: Catalog fills window width when detail panel is hidden
- **WHEN** no item is selected and the detail panel is not shown
- **THEN** the catalog occupies the full width from the sidebar right edge to the window right edge with no visible resize handle

#### Scenario: Detail panel appears without shrinking the catalog
- **WHEN** an item is selected and the detail panel opens
- **THEN** the detail panel is displayed at a fixed width overlapping or adjacent to the catalog without pushing catalog content to resize
