## ADDED Requirements

### Requirement: Removing a file opener entry requires confirmation
The app SHALL display a confirmation dialog when the user activates the remove button on a file opener entry. The dialog SHALL identify the entry being removed (extension and app name). The entry SHALL only be removed from the list if the user confirms. If the user cancels, the entry SHALL remain unchanged.

#### Scenario: User confirms removal
- **WHEN** the user clicks the remove button on a file opener entry and confirms in the dialog
- **THEN** the entry is removed from the file openers list

#### Scenario: User cancels removal
- **WHEN** the user clicks the remove button on a file opener entry and dismisses or cancels the dialog
- **THEN** the entry remains in the file openers list unchanged

#### Scenario: Confirmation dialog identifies the entry
- **WHEN** the confirmation dialog is shown
- **THEN** the dialog displays the extension and/or app name of the entry being removed so the user can verify the action
