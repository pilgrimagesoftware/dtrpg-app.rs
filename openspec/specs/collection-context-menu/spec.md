# collection-context-menu Specification

## Purpose
TBD - created by archiving change catalog-collections-improvements. Update Purpose after archive.
## Requirements
### Requirement: Collection sidebar items have a right-click context menu
Each collection entry in the sidebar SHALL respond to a right-click (or equivalent) with a context menu.

#### Scenario: Context menu appears on right-click
- **WHEN** the user right-clicks a collection entry in the sidebar
- **THEN** a context menu appears with at least "Download All", "Reload", and "Delete" actions

### Requirement: Context menu "Download All" action enqueues every not-yet-downloaded item in the collection
The context menu "Download All" action on a collection entry SHALL enqueue a download for every
catalog item that is a member of that collection and is not already fully downloaded, using the
same per-item download entry point as the existing single-item "Download" action. Items that are
already fully downloaded, or whose files are already queued or actively downloading, MUST NOT be
re-enqueued or duplicated.

#### Scenario: Downloading a collection with nothing downloaded yet
- **WHEN** the user selects "Download All" from a collection's context menu, and none of the
  collection's member items have any downloaded files
- **THEN** every member item is enqueued for download

#### Scenario: Downloading a collection with some items already downloaded
- **WHEN** the user selects "Download All" from a collection's context menu, and some member
  items are already fully downloaded
- **THEN** only the not-yet-downloaded member items are enqueued; already-downloaded items are
  left untouched

#### Scenario: Selecting "Download All" while some items are already queued or downloading
- **WHEN** the user selects "Download All" from a collection's context menu, and some member
  items already have files queued or actively downloading
- **THEN** those files are not enqueued a second time; only files not yet downloaded and not
  already queued/active are enqueued

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

