# storage-auto-create Specification

## Purpose
TBD - created by archiving change create-default-download-directory. Update Purpose after archive.
## Requirements
### Requirement: The default download directory is created automatically
The application SHALL create the platform default download directory automatically at startup when no storage override is configured and the directory does not already exist, rather than surfacing a "does not exist" warning for a location the app itself owns.

#### Scenario: Fresh install, default directory missing
- **WHEN** the app starts with no storage override configured and the platform default
  download directory does not exist
- **THEN** the directory is created and the Settings storage warning does not appear

#### Scenario: Default directory creation fails
- **WHEN** the app attempts to create the default download directory and the operation fails
  (e.g. permissions)
- **THEN** the failure is logged and the existing "storage folder does not exist" warning
  still surfaces in Settings

#### Scenario: User-chosen override path is missing
- **WHEN** a user-configured storage override path does not exist (e.g. an unmounted
  external or network volume)
- **THEN** the app does not create the directory automatically and the existing warning
  behavior is unchanged
