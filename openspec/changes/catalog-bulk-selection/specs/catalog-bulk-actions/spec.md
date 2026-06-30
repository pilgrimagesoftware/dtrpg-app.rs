## ADDED Requirements

### Requirement: Bulk-action bar appears when selection is non-empty in selection mode
The system SHALL render a bulk-action bar only when selection mode is active and at least one item is selected. When the selection becomes empty, the bar SHALL be hidden.

#### Scenario: Bar appears on first selection
- **WHEN** selection mode is active and the user selects the first item
- **THEN** the bulk-action bar becomes visible

#### Scenario: Bar disappears when selection is cleared
- **WHEN** all items are deselected (or Deselect All is invoked)
- **THEN** the bulk-action bar is hidden

### Requirement: Bulk Download queues selected items for download
The system SHALL provide a Bulk Download action that enqueues each selected item that is not already downloaded.

#### Scenario: Bulk Download enqueues undownloaded items
- **WHEN** the user invokes Bulk Download with items selected
- **THEN** each selected item that is not yet locally present is queued for download via the activity system

#### Scenario: Bulk Download skips already-downloaded items
- **WHEN** a selected item is already present locally
- **THEN** that item is not re-enqueued

#### Scenario: Bulk Download clears selection on dispatch
- **WHEN** the Bulk Download action is dispatched
- **THEN** the selection set is cleared and selection mode is deactivated

### Requirement: Bulk Remove Download deletes local files for selected items
The system SHALL provide a Bulk Remove Download action that deletes the local download directory for each selected item that is currently downloaded.

#### Scenario: Bulk Remove Download removes local files
- **WHEN** the user invokes Bulk Remove Download with downloaded items selected
- **THEN** each selected item's local directory is removed from disk

#### Scenario: Bulk Remove Download skips cloud-only items
- **WHEN** a selected item has no local download
- **THEN** that item is skipped silently

#### Scenario: Bulk Remove Download clears selection on dispatch
- **WHEN** the Bulk Remove Download action is dispatched
- **THEN** the selection set is cleared and selection mode is deactivated

### Requirement: Bulk Fetch Thumbnail queues thumbnail retrieval for selected items
The system SHALL provide a Bulk Fetch Thumbnail action that re-triggers thumbnail retrieval for each selected item.

#### Scenario: Bulk Fetch Thumbnail queues selected items
- **WHEN** the user invokes Bulk Fetch Thumbnail with items selected
- **THEN** each selected item is queued for thumbnail fetch

#### Scenario: Bulk Fetch Thumbnail clears selection on dispatch
- **WHEN** the Bulk Fetch Thumbnail action is dispatched
- **THEN** the selection set is cleared and selection mode is deactivated

### Requirement: Bulk Add to Collection assigns selected items to a chosen collection
The system SHALL provide a Bulk Add to Collection action. When invoked, a collection picker popover lists all available collections; the user selects one and the selected catalog items are assigned to that collection.

#### Scenario: Collection picker opens on action invocation
- **WHEN** the user invokes Bulk Add to Collection
- **THEN** a collection picker popover appears listing all available collections

#### Scenario: Items are added to the chosen collection
- **WHEN** the user selects a collection in the picker
- **THEN** each selected item is added to that collection and the picker closes

#### Scenario: Picker dismissal cancels the action
- **WHEN** the user dismisses the picker without selecting a collection
- **THEN** no collection assignment is made and the selection is preserved

#### Scenario: Bulk Add to Collection clears selection on dispatch
- **WHEN** a collection is chosen and the action is dispatched
- **THEN** the selection set is cleared and selection mode is deactivated

### Requirement: Bulk Remove from Collection removes selected items from a chosen collection
The system SHALL provide a Bulk Remove from Collection action. When invoked, a collection picker lists collections that contain at least one selected item; choosing a collection removes the selected items from it.

#### Scenario: Collection picker shows only relevant collections
- **WHEN** the user invokes Bulk Remove from Collection
- **THEN** the picker lists only collections that contain at least one currently selected item

#### Scenario: Items are removed from the chosen collection
- **WHEN** the user selects a collection in the picker
- **THEN** each selected item that belongs to that collection is removed from it

#### Scenario: Bulk Remove from Collection clears selection on dispatch
- **WHEN** a collection is chosen and the action is dispatched
- **THEN** the selection set is cleared and selection mode is deactivated

### Requirement: Bulk Open opens each selected item in the system PDF viewer
The system SHALL provide a Bulk Open action that opens each selected item's downloaded PDF in the platform file manager / viewer. Items without a local download are skipped.

#### Scenario: Bulk Open opens downloaded items
- **WHEN** the user invokes Bulk Open with downloaded items selected
- **THEN** each downloaded item's file is opened via the platform reveal/open utility

#### Scenario: Bulk Open skips cloud-only items
- **WHEN** a selected item has no local file
- **THEN** that item is skipped silently

#### Scenario: Bulk Open clears selection on dispatch
- **WHEN** the Bulk Open action is dispatched
- **THEN** the selection set is cleared and selection mode is deactivated
