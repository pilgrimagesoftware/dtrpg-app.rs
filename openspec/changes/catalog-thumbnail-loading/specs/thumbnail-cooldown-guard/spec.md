## ADDED Requirements

### Requirement: Record last thumbnail load attempt
`LibraryItem` SHALL store the timestamp of the most recent thumbnail load attempt as `thumbnail_last_attempted: Option<std::time::SystemTime>`. This field SHALL be set to `Some(SystemTime::now())` whenever a thumbnail fetch begins for the item (auto-enqueue or manual trigger).

#### Scenario: Timestamp set on fetch start
- **WHEN** a thumbnail fetch begins for an item
- **THEN** `thumbnail_last_attempted` is set to the current system time on that item

#### Scenario: Timestamp absent for never-attempted items
- **WHEN** an item has never had a thumbnail fetch attempted
- **THEN** `thumbnail_last_attempted` is `None`

### Requirement: Context menu item disabled within cooldown window
The "Load Thumbnail" context menu item SHALL be disabled if the item's `thumbnail_last_attempted` is within 5 minutes of the current time. It SHALL be enabled if the timestamp is absent or older than 5 minutes.

#### Scenario: Menu item disabled when within cooldown
- **WHEN** a catalog entry's `thumbnail_last_attempted` is less than 5 minutes ago
- **THEN** the "Load Thumbnail" menu item is rendered in a disabled state

#### Scenario: Menu item enabled when cooldown has elapsed
- **WHEN** a catalog entry's `thumbnail_last_attempted` is 5 or more minutes ago
- **THEN** the "Load Thumbnail" menu item is enabled

#### Scenario: Menu item enabled when never attempted
- **WHEN** a catalog entry's `thumbnail_last_attempted` is `None`
- **THEN** the "Load Thumbnail" menu item is enabled

#### Scenario: Clock skew handled gracefully
- **WHEN** `SystemTime::now()` is earlier than `thumbnail_last_attempted` (clock went backwards)
- **THEN** the cooldown is treated as elapsed and the menu item is enabled
