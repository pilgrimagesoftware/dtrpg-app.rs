# libri-detail-panel Specification

## Purpose
TBD - created by archiving change implement-libri-ui-in-gpui. Update Purpose after archive.
## Requirements

### Requirement: Detail panel MUST open when a library item is selected
The detail panel MUST become visible when the user selects a library item from any catalog layout. It MUST render over the right side of the main content area without displacing the sidebar or toolbar.

#### Scenario: Selecting an item opens the detail panel
- **WHEN** the user activates a library item in the catalog
- **THEN** the detail panel slides into view showing that item's details

#### Scenario: Panel does not displace sidebar or toolbar
- **WHEN** the detail panel is open
- **THEN** the sidebar and toolbar remain fully visible and usable

### Requirement: Detail panel MUST display the item cover, publisher, title, and game line
The detail panel MUST render the item's generative cover at the top, followed by the publisher name, item title (as a heading), and game line name.

#### Scenario: Detail header shows cover and title information
- **WHEN** the detail panel is open for a selected item
- **THEN** the panel displays the generative cover, then publisher, then title, then game line in vertical order

### Requirement: Detail panel MUST display the item description
The detail panel MUST render the item's description text below the title header.

#### Scenario: Description text is displayed
- **WHEN** the detail panel is open for a selected item that has a description
- **THEN** the description paragraph is rendered below the game line

### Requirement: Detail panel MUST provide Read and Download action buttons
The detail panel MUST render a primary "Read" button and a secondary button whose label is "Downloaded" (with a checkmark) when the item status is downloaded, or "Download" (with a download icon) when the item status is cloud-only.

#### Scenario: Downloaded item shows Downloaded button
- **WHEN** the detail panel is open for an item with status "downloaded"
- **THEN** the secondary action button shows "Downloaded" with a checkmark indicator

#### Scenario: Cloud-only item shows Download button
- **WHEN** the detail panel is open for an item with status "cloud"
- **THEN** the secondary action button shows "Download" with a download icon

#### Scenario: Activating Download toggles item status
- **WHEN** the user activates the Download button for a cloud-only item
- **THEN** the item status changes to downloaded and the button updates to show "Downloaded"

### Requirement: Detail panel MUST display a metadata table
The detail panel MUST render a definition list (label/value pairs) containing: System, Category, Format, Pages, File size, Released (year), Added (date), and Status.

#### Scenario: Metadata table shows all required fields
- **WHEN** the detail panel is open for a selected item
- **THEN** the metadata table contains rows for System, Category, Format, Pages, File size, Released, Added, and Status

#### Scenario: Status row reflects current download state
- **WHEN** the item status changes between downloaded and cloud
- **THEN** the Status row in the metadata table updates to reflect the new state ("On this device" or "In the cloud")

### Requirement: Detail panel MUST be dismissible via close button and Escape key
The detail panel MUST render a close button. Activating the close button or pressing the Escape key MUST dismiss the panel and clear the item selection.

#### Scenario: Close button dismisses the panel
- **WHEN** the detail panel is open and the user activates the close button
- **THEN** the detail panel closes and no item is selected

#### Scenario: Escape key dismisses the panel
- **WHEN** the detail panel is open and the user presses the Escape key
- **THEN** the detail panel closes and no item is selected
