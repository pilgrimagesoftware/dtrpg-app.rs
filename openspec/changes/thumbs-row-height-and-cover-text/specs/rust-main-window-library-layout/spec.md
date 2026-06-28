## MODIFIED Requirements

### Requirement: Rust library presentations MUST share browsing state
The Rust desktop app MUST use one controller-facing browsing state for list, tree, and grid presentations so mode changes preserve the current filtered and sorted result set. In the thumbs presentation, each row SHALL be tall enough to contain the thumbnail without overflow. The generative cover tile in thumbs rows SHALL display only the background colour and motif shape, without publisher, title, or product line text overlaid on the cover. The generative cover tile in grid cards MAY display text.

#### Scenario: Switching between Rust list, tree, and grid views
- **WHEN** the user switches library presentation mode
- **THEN** the same matched items, grouping, and sort order are represented in the selected GPUI presentation

#### Scenario: Thumbs row height contains the thumbnail
- **WHEN** the user views the catalog in thumbs presentation
- **THEN** each row is tall enough to fully contain the thumbnail without it overflowing into adjacent rows

#### Scenario: Generative cover in thumbs shows no text
- **WHEN** the user views the catalog in thumbs presentation
- **THEN** each thumbnail displays only a coloured background and a motif shape, with no publisher, title, or product line text rendered inside the cover tile

#### Scenario: Generative cover in grid shows text
- **WHEN** the user views the catalog in grid presentation
- **THEN** each grid card cover may display publisher, title, and product line text inside the cover tile
