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

### Requirement: Advanced settings displays cache-related timeout and cooldown values
The Advanced settings section SHALL display the app's cache-related timeout and cooldown constants: catalog cache staleness window, manual reload cooldown, item availability check cooldown, item check batch cooldown, item check batch timer interval, and thumbnail retry cooldown. These are fixed, read-only values — the section provides visibility, not an editing control.

#### Scenario: Viewing timing details
- **WHEN** the user opens the Advanced settings section
- **THEN** the section displays each of the six timing values in a human-readable form (e.g. "7 days", "60 seconds", "5 minutes")

### Requirement: Every cache detail data point has a label and an explanatory description
Each data point in the Advanced settings "Cache details" area (both counts and timing values) SHALL be presented with a concise short label and a one-line explanatory description of what the value means, shown as accompanying text or a tooltip.

#### Scenario: Viewing a data point's description
- **WHEN** the user views any cache detail data point (a count or a timing value) in Advanced settings
- **THEN** a short label identifies the data point and a one-line description explains what it represents, visible either as text beneath the label or via a tooltip
