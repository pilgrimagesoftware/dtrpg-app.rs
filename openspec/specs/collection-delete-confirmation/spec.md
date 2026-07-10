# collection-delete-confirmation Specification

## Purpose
TBD - created by archiving change confirm-collection-delete. Update Purpose after archive.
## Requirements
### Requirement: Deleting a collection MUST be confirmed before the request is sent
The app MUST show a confirmation dialog naming the collection before sending the delete request. The collection MUST NOT be deleted unless the user confirms.

#### Scenario: Confirming deletion
- **WHEN** the user selects "Delete" from a collection's context menu and confirms the dialog
- **THEN** the collection is deleted (server request sent, sidebar and filter updated as today)

#### Scenario: Cancelling deletion
- **WHEN** the user selects "Delete" from a collection's context menu and dismisses or cancels the dialog
- **THEN** no delete request is sent and the collection remains unchanged

