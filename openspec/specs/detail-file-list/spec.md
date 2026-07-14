# detail-file-list Specification

## Purpose
Renders the list of files bundled with a downloaded catalog item in the expanded detail tab,
one row per file with name and size, and flags which rows are Zip archives so
`zip-content-preview` can offer its interaction only on those rows.

## Requirements
### Requirement: Detail tab renders a per-item file list
The expanded detail tab SHALL render one row per file bundled with the catalog item,
showing at minimum the file's name and size, using the item's `files` list instead of a
single implicit download target.

#### Scenario: Item with a single bundled file
- **WHEN** the detail tab is opened for an item whose `files` list contains exactly one
  entry
- **THEN** the detail tab renders exactly one file row for that entry

#### Scenario: Item with multiple bundled files
- **WHEN** the detail tab is opened for an item whose `files` list contains more than one
  entry (e.g. a "PDF + EPUB" format item)
- **THEN** the detail tab renders one row per entry, each showing that entry's name and
  size

#### Scenario: Item with no file metadata
- **WHEN** the detail tab is opened for an item whose `files` list is empty (e.g. cached
  catalog data predating this capability)
- **THEN** the detail tab renders no file rows and does not panic or show a broken row

### Requirement: File rows identify Zip archives
Each rendered file row SHALL indicate whether that file is a Zip archive, so that
Zip-specific interactions (see `zip-content-preview`) are only offered for Zip rows.

#### Scenario: Zip file row
- **WHEN** a file row's underlying `ItemFile.is_zip` is `true`
- **THEN** the row is eligible for the Zip content preview interaction

#### Scenario: Non-Zip file row
- **WHEN** a file row's underlying `ItemFile.is_zip` is `false`
- **THEN** the row renders name and size only, with no Zip preview interaction offered
