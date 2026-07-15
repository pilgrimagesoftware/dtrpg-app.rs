# alert-history-view Specification

## Purpose
TBD - created by archiving change copy-alert-error-message. Update Purpose after archive.
## Requirements
### Requirement: Alert history entries support copying the error message
Each row in the alert history panel SHALL provide a control that copies that entry's error
message text to the system clipboard.

#### Scenario: Copying an alert's error message
- **WHEN** the user hovers an alert history row and clicks its copy control
- **THEN** the entry's error message text is copied to the system clipboard exactly as
  displayed, and the label or timestamp are not included

#### Scenario: Copy control only visible on hover
- **WHEN** the user is not hovering an alert history row
- **THEN** that row's copy control is not visible

### Requirement: Error activity is retained in a durable alert log
Every activity item resolved as an error (via `ActivityController::error`) SHALL be recorded
in a durable alert log, independent of the transient activity panel's expiry timers. The
alert log SHALL be capped at a fixed size; once full, the oldest entry SHALL be evicted when
a new entry is added.

#### Scenario: Error activity is logged
- **WHEN** an in-progress activity item is resolved with `error(...)`
- **THEN** an `AlertEntry` with the same label and error message is appended to the alert log

#### Scenario: Alert log survives activity panel expiry
- **WHEN** an error activity item has expired out of the activity panel's `recent` list
- **THEN** the corresponding `AlertEntry` remains present in the alert log

#### Scenario: Alert log survives service replacement
- **WHEN** the library service is replaced (e.g. sign-out then sign-in)
- **THEN** the alert log is not cleared

#### Scenario: Alert log is capped
- **WHEN** the alert log already contains `ALERT_LOG_CAP` entries and a new error occurs
- **THEN** the oldest entry is evicted before the new entry is appended

### Requirement: "Window > Show Alert History" opens the alert history panel
The `ShowAlertHistory` menu action SHALL toggle a panel listing all entries in the alert log,
newest first.

#### Scenario: Opening the panel with entries present
- **WHEN** the user selects "Window > Show Alert History" and the alert log is non-empty
- **THEN** the panel displays each entry's label, error message, and a relative timestamp,
  ordered newest first

#### Scenario: Opening the panel with no entries
- **WHEN** the user selects "Window > Show Alert History" and the alert log is empty
- **THEN** the panel displays an empty state

#### Scenario: Hovering a timestamp shows the absolute date and time
- **WHEN** the user hovers an alert entry's relative timestamp
- **THEN** a tooltip shows the absolute date and time

#### Scenario: Clearing the alert log
- **WHEN** the user clicks "Clear" in the alert history panel header
- **THEN** all entries are removed from the alert log and the panel shows the empty state

#### Scenario: Toggling closes the panel
- **WHEN** the user selects "Window > Show Alert History" again while the panel is open
- **THEN** the panel closes

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

