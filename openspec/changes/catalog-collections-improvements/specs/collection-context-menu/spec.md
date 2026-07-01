## ADDED Requirements

### Requirement: Collection sidebar items have a right-click context menu
Each collection entry in the sidebar SHALL respond to a right-click (or equivalent) with a context menu.

#### Scenario: Context menu appears on right-click
- **WHEN** the user right-clicks a collection entry in the sidebar
- **THEN** a context menu appears with at least "Reload" and "Delete" actions

### Requirement: Context menu Reload action refreshes the collection
The context menu "Reload" action on a collection entry SHALL trigger a live API fetch of that collection's items and update the catalog accordingly.

#### Scenario: Reload refreshes the collection
- **WHEN** the user selects "Reload" from a collection's context menu
- **THEN** the collection's items are re-fetched from the API and the catalog view updates

### Requirement: Context menu Delete action removes the collection
The context menu "Delete" action on a collection entry SHALL remove the collection via the API and remove it from the sidebar.

#### Scenario: Delete removes the collection
- **WHEN** the user selects "Delete" from a collection's context menu
- **THEN** the collection is deleted via the API call and the entry is removed from the sidebar

#### Scenario: Delete failure leaves the collection in place
- **WHEN** the user selects "Delete" and the API call fails
- **THEN** the collection entry remains in the sidebar and an error is logged to the activity panel
