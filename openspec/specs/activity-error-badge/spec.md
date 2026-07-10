# activity-error-badge Specification

## Purpose
Keeps the status bar's notification affordances (the unread-alert dot and the "n unread
errors" tooltip count) consistent with the durable notifications view (alert history log),
rather than either drifting from it via a separate, self-expiring transient list.

## Requirements
### Requirement: The unread-alert dot SHALL reflect unseen notifications-view activity

The status bar SHALL display an unread-alert dot on the notifications button when an alert
has been logged since the notifications view was last opened, and SHALL clear it when the
view is opened or its contents are cleared.

#### Scenario: An error is logged
- **WHEN** a background operation fails and its error entry is added to the notifications
  view
- **THEN** the status bar displays the unread-alert dot

#### Scenario: Notifications view is opened or cleared
- **WHEN** the user opens the notifications popover, or clears its contents
- **THEN** the unread-alert dot is no longer displayed

### Requirement: The notifications tooltip count SHALL reflect the notifications view's contents

The status bar's notifications button tooltip SHALL report the number of entries currently
in the notifications view (the durable alert history log), not a count derived from any
transient, self-expiring activity list.

#### Scenario: Notifications view has entries
- **WHEN** the notifications view contains N entries
- **THEN** the notifications tooltip reports N unread errors

#### Scenario: User clears the notifications view
- **WHEN** the user clears the notifications view
- **THEN** the notifications tooltip immediately reports 0, in the same update, without
  waiting for any transient item to expire on its own timer

#### Scenario: Transient activity list expiry does not affect the count
- **WHEN** an item in the transient, self-expiring recent-activity list expires
- **THEN** the notifications tooltip count is unaffected, since it reflects the durable
  notifications view (alert history log) rather than that transient list

