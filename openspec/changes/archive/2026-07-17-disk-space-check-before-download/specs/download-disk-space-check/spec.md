## ADDED Requirements

### Requirement: Pending download size is checked against free disk space
Before queuing a download via the entry-level download action, a per-file download action, or a "Download All" action for a collection or publisher, the app SHALL calculate the total size in megabytes of the not-yet-downloaded files that action would enqueue and compare it against the free disk space available at the storage root (`StorageConfig::root_path()`).

#### Scenario: Sufficient free space queues immediately
- **WHEN** the user triggers a download-queuing action and the available free disk space is greater than or equal to the calculated size of the files to be queued
- **THEN** the files are queued immediately with no confirmation prompt

#### Scenario: Free disk space cannot be determined
- **WHEN** the app is unable to determine free disk space at the storage root (e.g. an I/O error or unsupported filesystem)
- **THEN** the files are queued immediately, identical to when free space is sufficient

### Requirement: Insufficient free space requires user confirmation
When the calculated size of the files a queuing action would enqueue exceeds the available free disk space, the app SHALL present a confirmation dialog naming the shortfall and SHALL NOT queue any of those files unless the user confirms.

#### Scenario: Insufficient space shows a warning dialog
- **WHEN** the user triggers a download-queuing action and the calculated size of the files to be queued exceeds the available free disk space
- **THEN** a confirmation dialog appears before anything is queued, stating that free space is insufficient

#### Scenario: Confirming proceeds with the download
- **WHEN** the user confirms the low-disk-space warning dialog
- **THEN** the originally requested files are queued exactly as if the space check had passed

#### Scenario: Cancelling aborts the queuing action
- **WHEN** the user cancels or dismisses the low-disk-space warning dialog
- **THEN** none of the files from that queuing action are queued, and no partial subset is enqueued

### Requirement: The check covers whole queuing actions, not individual files
The disk-space check SHALL be evaluated once per user-initiated queuing action, as an aggregate over every file that action would enqueue. A bulk action (a collection's or publisher's "Download All") SHALL be checked as a single aggregate across all of its matching items, not once per item.

#### Scenario: Multi-item entry checked as one total
- **WHEN** the user triggers the entry-level download action on a multi-item entry with several not-yet-downloaded files
- **THEN** the calculated size used for the check is the sum of every not-yet-downloaded file's size in that entry, not any single file's size alone

#### Scenario: Download All checked as one aggregate across the collection
- **WHEN** the user triggers "Download All" for a collection containing multiple items with not-yet-downloaded files
- **THEN** the calculated size used for the check is the sum across every not-yet-downloaded file in every matching item, evaluated once before any item is queued
