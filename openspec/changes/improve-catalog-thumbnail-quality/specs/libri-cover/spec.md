## ADDED Requirements

### Requirement: Cover URL resolution MUST prefer higher-resolution sources over small pre-generated thumbnails
When resolving a catalog item's cover URL from the API's product metadata, the app MUST prefer the full-size image, then the WebP image, before falling back to the 140px or 100px pre-generated thumbnails. The order MUST be: full-size image, then WebP image, then 140px thumbnail, then 100px thumbnail — the first present field wins.

#### Scenario: Full-size image is present
- **WHEN** a catalog item's product metadata includes a full-size image
- **THEN** the resolved cover URL uses the full-size image, regardless of whether smaller thumbnails are also present

#### Scenario: Full-size image absent, WebP image present
- **WHEN** a catalog item's product metadata has no full-size image but has a WebP image
- **THEN** the resolved cover URL uses the WebP image, regardless of whether the pre-generated thumbnails are also present

#### Scenario: Only pre-generated thumbnails are present
- **WHEN** a catalog item's product metadata has neither a full-size nor a WebP image, but has a 140px thumbnail
- **THEN** the resolved cover URL uses the 140px thumbnail

#### Scenario: Only the smallest thumbnail is present
- **WHEN** a catalog item's product metadata has only the 100px thumbnail populated
- **THEN** the resolved cover URL uses the 100px thumbnail

#### Scenario: No image field is present
- **WHEN** a catalog item's product metadata has none of the four image fields populated
- **THEN** the resolved cover URL is absent, and the cover falls back to the generative cover per the existing fallback requirement
