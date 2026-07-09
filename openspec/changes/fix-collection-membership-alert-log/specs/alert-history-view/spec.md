## ADDED Requirements

### Requirement: Collection membership failures are recorded in the alert history
Failures to add or remove a catalog item from a collection (from the Manage Collections
dialog) SHALL be recorded in the durable alert log, in addition to the toast notification
already shown.

#### Scenario: Failed add-to-collection appears in alert history
- **WHEN** adding an item to a collection fails and a toast notification is shown
- **THEN** the failure also appears as an entry in "Window > Show Alert History"

#### Scenario: Failed remove-from-collection appears in alert history
- **WHEN** removing an item from a collection fails and a toast notification is shown
- **THEN** the failure also appears as an entry in "Window > Show Alert History"
