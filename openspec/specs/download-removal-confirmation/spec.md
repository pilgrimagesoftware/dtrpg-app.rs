# download-removal-confirmation Specification

## Purpose
TBD - created by archiving change confirm-download-removal. Update Purpose after archive.

## Requirements
### Requirement: Removing a downloaded item MUST be confirmed before it happens
The app MUST show a confirmation dialog naming the item before reverting a `Downloaded` item's status to `Cloud`. The item's status MUST NOT change unless the user confirms.

#### Scenario: Confirming removal
- **WHEN** the user triggers "Remove Download" for a downloaded item and confirms the dialog
- **THEN** the item's status reverts to `Cloud`

#### Scenario: Cancelling removal
- **WHEN** the user triggers "Remove Download" for a downloaded item and dismisses or cancels the dialog
- **THEN** the item's status remains `Downloaded` and no other state changes

### Requirement: The confirmation MUST be shown from every UI entry point that can remove a download
The app MUST show the confirmation dialog regardless of which control triggered the removal: the catalog context menu, the item popover, or the detail panel's download button.

#### Scenario: Catalog context menu
- **WHEN** the user selects "Remove Download" from a catalog item's context menu
- **THEN** the confirmation dialog appears before the item's status changes

#### Scenario: Item popover
- **WHEN** the user clicks the download toggle button in the item popover for a downloaded item
- **THEN** the confirmation dialog appears before the item's status changes

#### Scenario: Detail panel
- **WHEN** the user clicks the download button in the detail panel for a downloaded item
- **THEN** the confirmation dialog appears before the item's status changes
