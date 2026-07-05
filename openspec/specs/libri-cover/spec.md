# libri-cover Specification

## Purpose
TBD - created by archiving change implement-libri-ui-in-gpui. Update Purpose after archive.
## Requirements

### Requirement: Cover MUST display a real thumbnail when one is available
The cover component MUST render the catalog item's cover thumbnail image when that image is present in the local disk cache. The cached image MUST be used without a network request.

#### Scenario: Cache hit renders the real cover image
- **WHEN** the cover thumbnail for a library item is present in the local disk cache
- **THEN** the cover renders the cached image and does not show the generative fallback

### Requirement: Cover MUST download missing thumbnails in the background
When a cover thumbnail is not in the local cache, the cover component MUST enqueue a background download of the item's cover URL without blocking the main thread or the GPUI render loop. The generative fallback MUST be shown while the download is in flight.

#### Scenario: Cache miss triggers a background download
- **WHEN** the cover thumbnail for a library item is not in the local cache
- **THEN** a background download is enqueued for the item's cover URL and the generative fallback is displayed immediately

#### Scenario: Cover updates to the real image after download completes
- **WHEN** a background cover download completes successfully
- **THEN** the cover slot updates to display the downloaded thumbnail without requiring user interaction

#### Scenario: Download does not block catalog interaction
- **WHEN** cover thumbnails are downloading in the background
- **THEN** the catalog remains fully scrollable and interactive

### Requirement: Cover MUST fall back to a generative cover when the thumbnail cannot be acquired
When a cover thumbnail is neither cached nor downloadable (network unavailable, download failed, or cover URL absent), the cover component MUST render a deterministic generative cover for the item. The generative cover MUST render synchronously on the first paint.

#### Scenario: Download failure shows the generative cover
- **WHEN** a cover thumbnail download fails or the cover URL is absent
- **THEN** the cover renders the generative fallback for the remainder of the session

#### Scenario: Generative cover is shown immediately on first paint
- **WHEN** the catalog first renders and no thumbnails are cached yet
- **THEN** every cover slot displays its generative fallback without waiting for any network activity

### Requirement: Generative cover MUST derive its background color from the library item's color field
The generative cover MUST use the hexadecimal color value stored in the item's `color` field as the cover background.

#### Scenario: Generative cover background matches the item color field
- **WHEN** the generative cover is rendered for an item with a non-empty color field
- **THEN** the cover background renders using that hex color value

### Requirement: Generative cover MUST select a luminance-aware foreground color
The generative cover MUST compute the relative luminance of the background color using the ITU-R 601 formula (`r*299 + g*587 + b*114 > 150000` on 0–255 values) and apply an ink foreground on light backgrounds and a cream foreground on dark backgrounds.

#### Scenario: Light background receives an ink foreground
- **WHEN** the item's color field produces a luminance value above the threshold
- **THEN** generative cover text and motif elements render in a dark ink color

#### Scenario: Dark background receives a cream foreground
- **WHEN** the item's color field produces a luminance value at or below the threshold
- **THEN** generative cover text and motif elements render in a light cream color

### Requirement: Generative cover MUST render a deterministic decorative motif
The generative cover MUST derive a motif variant from the hash of the item's id concatenated with its title, then take that hash modulo 4. The four variants are: 0 = circle, 1 = diamond, 2 = double horizontal rule, 3 = triangle. The same item MUST always produce the same motif.

#### Scenario: Motif is consistent across renders
- **WHEN** a generative cover is rendered multiple times for the same item
- **THEN** the same motif variant appears each time

### Requirement: Generative cover MUST display publisher, title, and game line as text layers
The generative cover MUST render the publisher name near the top, the item title in the center, and the game line name near the bottom, all using the luminance-derived foreground color.

#### Scenario: Generative cover text layers are positioned correctly
- **WHEN** the generative cover is rendered
- **THEN** the publisher appears at the top, the title in the center, and the game line at the bottom
