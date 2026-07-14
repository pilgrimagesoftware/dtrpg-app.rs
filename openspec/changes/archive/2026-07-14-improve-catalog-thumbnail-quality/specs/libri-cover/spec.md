## ADDED Requirements

### Requirement: Small-context cover URL resolution MUST prefer pre-generated thumbnails over full-size images
When resolving a catalog item's cover URL for small render contexts (grid card, thumb row), the app MUST prefer the 140px thumbnail, then the 100px thumbnail, before falling back to the full-size image or WebP image. The order MUST be: 140px thumbnail, then 100px thumbnail, then full-size image, then WebP image — the first present field wins.

#### Scenario: 140px thumbnail is present
- **WHEN** a catalog item's product metadata includes a 140px thumbnail
- **THEN** the resolved small-context cover URL uses the 140px thumbnail, regardless of whether larger images are also present

#### Scenario: 140px thumbnail absent, 100px thumbnail present
- **WHEN** a catalog item's product metadata has no 140px thumbnail but has a 100px thumbnail
- **THEN** the resolved small-context cover URL uses the 100px thumbnail, regardless of whether larger images are also present

#### Scenario: Only larger images are present
- **WHEN** a catalog item's product metadata has neither pre-generated thumbnail, but has a full-size image
- **THEN** the resolved small-context cover URL uses the full-size image

#### Scenario: No image field is present
- **WHEN** a catalog item's product metadata has none of the four image fields populated
- **THEN** the resolved small-context cover URL is absent, and the cover falls back to the generative cover per the existing fallback requirement

### Requirement: Detail-context cover URL resolution MUST prefer full-size images over pre-generated thumbnails
When resolving a catalog item's cover URL for the detail panel, the app MUST prefer the full-size image, then the WebP image, before falling back to the 140px or 100px pre-generated thumbnails. The order MUST be: full-size image, then WebP image, then 140px thumbnail, then 100px thumbnail — the first present field wins.

#### Scenario: Full-size image is present
- **WHEN** a catalog item's product metadata includes a full-size image
- **THEN** the resolved detail-context cover URL uses the full-size image, regardless of whether smaller thumbnails are also present

#### Scenario: Full-size image absent, WebP image present
- **WHEN** a catalog item's product metadata has no full-size image but has a WebP image
- **THEN** the resolved detail-context cover URL uses the WebP image, regardless of whether the pre-generated thumbnails are also present

#### Scenario: Only pre-generated thumbnails are present
- **WHEN** a catalog item's product metadata has neither a full-size nor a WebP image, but has a 140px thumbnail
- **THEN** the resolved detail-context cover URL uses the 140px thumbnail

#### Scenario: No image field is present
- **WHEN** a catalog item's product metadata has none of the four image fields populated
- **THEN** the resolved detail-context cover URL is absent, and the detail panel falls back to the small-context cover URL

### Requirement: Detail panel MUST fetch its cover lazily, only when opened
The app MUST NOT eagerly download the detail-context cover image for every catalog item on catalog load. The detail-context cover image MUST be fetched only when a detail tab is opened for that item, and only if it is not already cached or already in flight.

#### Scenario: Opening a detail tab for the first time
- **WHEN** a user opens the detail tab for an item whose detail-context cover has not yet been fetched or cached
- **THEN** the app enqueues a fetch of the detail-context cover URL

#### Scenario: Opening a detail tab a second time
- **WHEN** a user opens the detail tab for an item whose detail-context cover is already cached
- **THEN** the app does not re-fetch the detail-context cover

#### Scenario: Detail-context cover not yet available
- **WHEN** the detail tab renders before its detail-context cover fetch has completed
- **THEN** the detail panel renders the item's small-context cover (if cached) rather than the generative placeholder
