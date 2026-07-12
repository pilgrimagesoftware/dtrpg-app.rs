## ADDED Requirements

### Requirement: File size fields MUST always show the catalog-reported size
The detail view MUST always display the catalog-reported size for a file or entry, regardless of download or on-disk state.

#### Scenario: Item not yet downloaded
- **WHEN** the detail view renders an item whose status is `Cloud`
- **THEN** the file size field shows only the catalog-reported size

#### Scenario: Downloaded status but file missing on disk
- **WHEN** the detail view renders an item whose status is `Downloaded` but no file exists at its resolved on-disk path
- **THEN** the file size field shows only the catalog-reported size, with no error or blank state

### Requirement: Downloaded files with a resolvable local path MUST show their actual on-disk size alongside the catalog size
When a file exists at its resolved path under `{storage_root}/items/{entry_id}/{file_name}`, the detail view MUST display the file's actual on-disk byte size, read from the filesystem at render time, appended to the catalog-reported size (e.g. `"12.0 MB (11.8 MB on disk)"`).

#### Scenario: Single-file entry with the file present on disk
- **WHEN** the detail view renders a downloaded single-file entry whose file exists at its resolved path
- **THEN** the file size field shows the catalog-reported size followed by the file's actual on-disk size in parentheses

#### Scenario: Multi-item entry file list with a file present on disk
- **WHEN** the detail view renders a multi-item entry's file list and a specific row's file exists at its resolved path
- **THEN** that row's file size field shows the catalog-reported size followed by the file's actual on-disk size in parentheses

### Requirement: Multi-file entries MUST show a combined top-level file size
For an entry with more than one file, the detail view's top-level file size field MUST show the combined (summed) catalog-reported size across all of the entry's files, and MUST use a label that distinguishes it from a single file's size.

#### Scenario: Multi-file entry's top-level field shows the combined catalog size
- **WHEN** the detail view renders a multi-file entry
- **THEN** the top-level file size field shows the sum of every file's catalog-reported size, labeled to indicate it is a total

#### Scenario: Multi-file entry's top-level field shows the combined on-disk size
- **WHEN** the detail view renders a multi-file entry where at least one file exists at its resolved on-disk path
- **THEN** the top-level file size field's on-disk suffix shows the sum of the on-disk sizes of whichever files are actually present

#### Scenario: Single-file entry keeps the existing label
- **WHEN** the detail view renders an entry with exactly one file
- **THEN** the top-level file size field uses the existing "File size" label, not the combined-total label

### Requirement: The multi-item entry's Items table MUST show a Size column
The Items table in a multi-item entry's file list MUST include a "Size" column showing each row's catalog-reported size, plus its on-disk size in parentheses when resolvable.

#### Scenario: Items table shows a Size column with catalog size
- **WHEN** the detail view renders a multi-item entry's Items table
- **THEN** each row displays a Size column with that row's catalog-reported size

#### Scenario: Items table's Size column shows on-disk size when present
- **WHEN** a specific row's file exists at its resolved on-disk path
- **THEN** that row's Size column shows the catalog-reported size followed by the actual on-disk size in parentheses
