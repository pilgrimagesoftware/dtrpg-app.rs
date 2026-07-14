# toast-notifications Specification

## Purpose
TBD - created by archiving change adopt-gpui-component-primitives. Update Purpose after archive.

## Requirements

### Requirement: Download complete events surface as toast notifications

The app SHALL display an auto-dismissing `Notification` toast when a background download completes successfully.

#### Scenario: Successful download triggers success toast
- **WHEN** `ActivityController` emits a download-complete event for an item
- **THEN** a `Notification` toast with `NotificationType::Success` and the item title appears in the root view
- **AND** the toast auto-dismisses after its configured duration

### Requirement: Download error events surface as error toast notifications

The app SHALL display an auto-dismissing `Notification` toast when a background download fails.

#### Scenario: Failed download triggers error toast
- **WHEN** `ActivityController` emits a download-error event for an item
- **THEN** a `Notification` toast with `NotificationType::Error` and the error message appears in the root view
- **AND** the toast auto-dismisses after its configured duration

### Requirement: Toast notifications do not replace the activity panel

Toast notifications provide ephemeral event feedback only. The activity panel continues to show the full download history and in-progress items.

#### Scenario: Toast and activity panel coexist
- **WHEN** a toast is visible and the activity panel is open
- **THEN** both are displayed simultaneously without conflict
