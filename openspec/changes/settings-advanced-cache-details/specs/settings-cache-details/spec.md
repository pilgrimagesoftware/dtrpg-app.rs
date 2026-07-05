## ADDED Requirements

### Requirement: Advanced settings displays cache data counts by type
The Advanced settings section SHALL display counts of cached data broken down by type: the number of cached catalog/collections metadata items, the number of cached cover thumbnails, and whether the avatar image is cached.

#### Scenario: Viewing cache details with populated cache
- **WHEN** the user opens the Advanced settings section and the app cache directory contains metadata, cover thumbnails, and a cached avatar
- **THEN** the section displays the metadata item count, the cover thumbnail count, and an indicator that the avatar is cached

#### Scenario: Viewing cache details with an empty cache
- **WHEN** the user opens the Advanced settings section and the app cache directory is empty or missing
- **THEN** the section displays zero counts for metadata and cover thumbnails and an indicator that the avatar is not cached

### Requirement: Cache counts refresh after clearing the cache
The Advanced settings section SHALL reflect updated (zeroed) cache counts after the user clears the cache.

#### Scenario: Counts after Clear Cache
- **WHEN** the user confirms "Clear cache" and the cache directory is deleted
- **THEN** the displayed metadata count, cover count, and avatar-cached indicator all reflect the now-empty cache on the next render

### Requirement: Advanced settings provides a button to open the cache folder
The Advanced settings section SHALL provide a button that reveals the app cache directory in the OS's native file manager.

#### Scenario: Opening the cache folder
- **WHEN** the user clicks "Open cache folder" in Advanced settings
- **THEN** the OS's native file manager (Finder on macOS, Explorer on Windows, the default file manager on Linux) opens showing the app cache directory, creating the directory first if it does not yet exist
