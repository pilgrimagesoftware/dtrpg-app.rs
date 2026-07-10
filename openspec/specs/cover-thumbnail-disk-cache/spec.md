# cover-thumbnail-disk-cache Specification

## Purpose
TBD - created by archiving change cover-cache-file-extensions. Update Purpose after archive.
## Requirements
### Requirement: Cover thumbnails are cached to disk
Downloaded cover thumbnail bytes SHALL be persisted to a disk cache so subsequent launches do not re-download a thumbnail already fetched in a prior session.

#### Scenario: Cache hit skips the network fetch
- **WHEN** a cover thumbnail is requested for an item whose bytes already exist in the disk cache
- **THEN** the cached bytes are used and no network request is made

#### Scenario: Cache miss fetches and persists
- **WHEN** a cover thumbnail is requested for an item with no cached bytes on disk
- **THEN** the bytes are fetched over the network, and the result is written to the disk cache for future launches

### Requirement: Cached cover files use a correct file extension
Each cached cover file SHALL be named with the file extension matching its actual image format, as determined by sniffing the file's content, rather than a generic or unrelated extension.

#### Scenario: JPEG bytes are cached with a .jpg extension
- **WHEN** a fetched cover thumbnail's bytes are sniffed as JPEG
- **THEN** the cached file is written with a `.jpg` extension

#### Scenario: PNG bytes are cached with a .png extension
- **WHEN** a fetched cover thumbnail's bytes are sniffed as PNG
- **THEN** the cached file is written with a `.png` extension

#### Scenario: WebP bytes are cached with a .webp extension
- **WHEN** a fetched cover thumbnail's bytes are sniffed as WebP
- **THEN** the cached file is written with a `.webp` extension

#### Scenario: GIF bytes are cached with a .gif extension
- **WHEN** a fetched cover thumbnail's bytes are sniffed as GIF
- **THEN** the cached file is written with a `.gif` extension

#### Scenario: BMP bytes are cached with a .bmp extension
- **WHEN** a fetched cover thumbnail's bytes are sniffed as BMP
- **THEN** the cached file is written with a `.bmp` extension

#### Scenario: Lookup finds the cached file regardless of format
- **WHEN** a cover thumbnail is requested for an item with a cached file on disk
- **THEN** the cache lookup locates the file by its item id independent of which of the known extensions it was written with

