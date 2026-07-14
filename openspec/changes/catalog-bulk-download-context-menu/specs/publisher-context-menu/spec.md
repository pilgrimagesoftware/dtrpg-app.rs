## ADDED Requirements

### Requirement: Publisher group headers have a right-click context menu
Each publisher group header in the catalog's grouped-by-publisher view (Grid, Thumbs, and List
presentations) SHALL respond to a right-click (or equivalent) with a context menu.

#### Scenario: Context menu appears on right-click
- **WHEN** the user right-clicks a publisher group header in the catalog
- **THEN** a context menu appears with at least a "Download All" action

### Requirement: Context menu "Download All" action enqueues every not-yet-downloaded item under that publisher
The context menu "Download All" action on a publisher group header SHALL enqueue a download for
every catalog item under that publisher whose files are not already fully downloaded, using the
same per-item download entry point as the existing single-item "Download" action. Items that are
already fully downloaded, or whose files are already queued or actively downloading, MUST NOT be
re-enqueued or duplicated.

#### Scenario: Downloading a publisher group with nothing downloaded yet
- **WHEN** the user selects "Download All" from a publisher group header's context menu, and none
  of that publisher's items have any downloaded files
- **THEN** every item under that publisher is enqueued for download

#### Scenario: Downloading a publisher group with some items already downloaded
- **WHEN** the user selects "Download All" from a publisher group header's context menu, and some
  of that publisher's items are already fully downloaded
- **THEN** only the not-yet-downloaded items are enqueued; already-downloaded items are left
  untouched

#### Scenario: Selecting "Download All" while some items are already queued or downloading
- **WHEN** the user selects "Download All" from a publisher group header's context menu, and some
  of that publisher's items already have files queued or actively downloading
- **THEN** those files are not enqueued a second time; only files not yet downloaded and not
  already queued/active are enqueued
