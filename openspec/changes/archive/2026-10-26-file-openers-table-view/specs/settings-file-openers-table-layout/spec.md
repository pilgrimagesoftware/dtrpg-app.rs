## ADDED Requirements

### Requirement: File Openers section renders entries in a column-aligned table
The File Openers settings section SHALL render its entries using `gpui-component` table primitives
(`Table`/`TableHeader`/`TableBody`/`TableRow`/`TableHead`/`TableCell`) with a header row labeling the
Extension and Application columns, rather than hand-rolled flex rows.

#### Scenario: Header row labels the columns
- **WHEN** the File Openers section is displayed with one or more configured entries
- **THEN** a header row is shown above the entries labeling the Extension and Application columns

#### Scenario: Entry rows align to the header columns
- **WHEN** the File Openers section displays configured entries
- **THEN** each entry's extension and application name render in cells aligned under their respective
  header columns, and the remove control renders in a trailing cell

### Requirement: Stale-app warning renders within the table layout
An entry whose application path no longer exists on disk SHALL continue to show a warning indicator,
rendered inline within the Application cell of that entry's row.

#### Scenario: Missing application shows inline warning
- **WHEN** an entry's `app_path` does not exist on disk
- **THEN** the entry's row displays the existing "app not found" warning text inside the Application
  cell, without adding a separate warning column

### Requirement: Pending add row aligns with the table columns
The in-progress "add file opener" row SHALL render as a table row aligned to the same Extension and
Application columns as committed entries, with the extension input in the Extension cell and the
picked application's name in the Application cell.

#### Scenario: Pending row matches column alignment
- **WHEN** a file opener add is in progress (an application has been picked but no extension confirmed)
- **THEN** the pending row's extension input renders in the Extension column and the app name renders
  in the Application column, aligned with committed entry rows

### Requirement: Empty state remains outside the table
When there are no configured file opener entries and no add is in progress, the section SHALL continue
to show the existing empty-state message instead of rendering an empty table.

#### Scenario: No entries and no pending add
- **WHEN** the file openers list is empty and no add is in progress
- **THEN** the empty-state message is shown and no table header or rows are rendered
