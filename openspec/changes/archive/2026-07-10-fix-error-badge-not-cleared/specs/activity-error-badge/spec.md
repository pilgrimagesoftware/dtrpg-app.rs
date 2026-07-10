## ADDED Requirements

### Requirement: The status bar error badge SHALL reflect the notifications view's contents

The status bar SHALL display the unread-error badge if and only if the notifications view
(the durable alert history log) is non-empty. The badge SHALL NOT be driven by any
transient, self-expiring activity list.

#### Scenario: An error appears in the notifications view
- **WHEN** a background operation fails and its error entry is added to the notifications
  view
- **THEN** the status bar displays the unread-error badge

#### Scenario: The notifications view is empty
- **WHEN** the notifications view contains no entries
- **THEN** the status bar does not display the unread-error badge

### Requirement: Clearing the notifications view SHALL clear the error badge

Clearing the notifications view SHALL immediately remove the status bar's unread-error
badge, in the same update, without requiring any other action or waiting for any timer.

#### Scenario: User clears the notifications view
- **WHEN** the user clears the notifications view while the error badge is displayed
- **THEN** the error badge is no longer displayed immediately after clearing

#### Scenario: Badge does not reappear on its own after being cleared
- **WHEN** the notifications view has been cleared and no new errors have occurred
- **THEN** the error badge remains absent, regardless of how much time has passed
