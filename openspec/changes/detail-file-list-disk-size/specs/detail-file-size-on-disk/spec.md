## ADDED Requirements

### Requirement: Downloaded files with a resolvable local path MUST show their actual on-disk size
The detail view MUST display a file's actual on-disk byte size, read from the filesystem at render time, when the file exists at its resolved path under `{storage_root}/items/{entry_id}/{file_name}`.

#### Scenario: Single-file entry with the file present on disk
- **WHEN** the detail view renders a downloaded single-file entry whose file exists at its resolved path
- **THEN** the file size field shows the file's actual on-disk size, not the catalog-reported size

#### Scenario: Multi-item entry file list with a file present on disk
- **WHEN** the detail view renders a multi-item entry's file list and a specific row's file exists at its resolved path
- **THEN** that row's file size field shows the file's actual on-disk size, not the catalog-reported size

### Requirement: Files with no resolvable local file MUST fall back to the catalog-reported size
The detail view MUST show the catalog-reported `size_mb` value, unchanged from current behavior, whenever a file's on-disk path cannot be resolved or the file does not exist there.

#### Scenario: Item not yet downloaded
- **WHEN** the detail view renders an item whose status is `Cloud`
- **THEN** the file size field shows the catalog-reported size, not an on-disk lookup

#### Scenario: Downloaded status but file missing on disk
- **WHEN** the detail view renders an item whose status is `Downloaded` but no file exists at its resolved on-disk path
- **THEN** the file size field shows the catalog-reported size, with no error or blank state
