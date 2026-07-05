## ADDED Requirements

### Requirement: Catalog MUST render text-list rows in List layout
In List layout the catalog MUST render each library item as a row with the following columns in order: title (with kind tag), publisher, system/game line, page count, file size, date added (month and year), and a download/cloud status glyph.

#### Scenario: List layout shows a column header row
- **WHEN** the catalog is in List layout
- **THEN** a sticky header row labels the columns: Title, Publisher, System, Pages, Size, Added, and a status column with no label

#### Scenario: List rows display all required fields
- **WHEN** the catalog is in List layout and library items are loaded
- **THEN** each row shows the item title, kind tag, publisher, game line, page count, file size, date added, and status glyph

### Requirement: Catalog MUST render thumbnail rows in Thumbs layout
In Thumbs layout the catalog MUST render each library item as a row containing a cover thumbnail on the left, the item title and publisher/game-line sub-line in the center, and kind tag, dimensions summary, date added, and status glyph on the right.

#### Scenario: Thumbs rows display cover art and metadata
- **WHEN** the catalog is in Thumbs layout and library items are loaded
- **THEN** each row shows the generative cover thumbnail, title, publisher and game line, kind tag, pages/size/format summary, date added, and status glyph

### Requirement: Catalog MUST render grid cards in Grid layout
In Grid layout the catalog MUST render each library item as a card containing the generative cover at the top, the item title below, and the publisher name and status glyph in a footer row.

#### Scenario: Grid cards display cover, title, and publisher
- **WHEN** the catalog is in Grid layout and library items are loaded
- **THEN** each card shows the full generative cover image, the item title, the publisher name, and the status glyph

### Requirement: Catalog layouts MUST share a single filtered and sorted result set
All three catalog layouts MUST display the same items in the same order for a given filter, sort, and grouping state. Switching layout MUST NOT alter which items are visible or their order.

#### Scenario: Switching from List to Grid preserves matched items
- **WHEN** a search query is active and the user switches from List to Grid layout
- **THEN** the grid shows the same items that were visible in the list, in the same sort order

### Requirement: Catalog MUST support optional publisher grouping sections
When the group-by-publisher toggle is enabled, the catalog MUST organize items into sections, one per publisher, each preceded by a section header showing the publisher name and the count of items in that section.

#### Scenario: Grouping renders one section per publisher
- **WHEN** group-by-publisher is enabled
- **THEN** the catalog renders a section header followed by the publisher's items for each publisher in the filtered result set

#### Scenario: Section headers show publisher name and item count
- **WHEN** group-by-publisher is enabled and items are loaded
- **THEN** each section header displays the publisher name and the number of items in that section

### Requirement: Catalog MUST display an empty state when no items match
When the filtered result set is empty the catalog MUST replace the item list with an empty state indicator containing a search icon and a "No titles match." message.

#### Scenario: Empty state appears when search returns no results
- **WHEN** the user enters a search query that matches no library items
- **THEN** the catalog hides all item rows and shows the empty state with a search icon and "No titles match."

#### Scenario: Empty state disappears when the query is cleared
- **WHEN** the search query is cleared after showing the empty state
- **THEN** the catalog returns to displaying the full unfiltered result set

### Requirement: Catalog MUST show a download/cloud status glyph on each item
Each item in all three layouts MUST display a status glyph indicating whether the item is downloaded to the device (filled dot) or available in the cloud only (cloud icon).

#### Scenario: Downloaded items show a filled dot glyph
- **WHEN** a library item has status "downloaded"
- **THEN** the catalog renders a small filled dot glyph on that item's row or card

#### Scenario: Cloud-only items show a cloud icon glyph
- **WHEN** a library item has status "cloud"
- **THEN** the catalog renders a cloud outline icon glyph on that item's row or card
