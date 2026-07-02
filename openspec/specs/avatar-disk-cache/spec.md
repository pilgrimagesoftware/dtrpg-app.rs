# avatar-disk-cache Specification

## Purpose
TBD - created by archiving change avatar-load-and-cache. Update Purpose after archive.
## Requirements
### Requirement: Avatar bytes served from disk cache when available
Before making a Gravatar network request, the app SHALL check for a locally cached avatar file at `{cache_dir}/dtrpg/avatar`. If the file exists and is non-empty, its bytes SHALL be used directly without a network request.

#### Scenario: Cache hit skips network request
- **WHEN** a cached avatar file exists and is non-empty
- **THEN** the avatar bytes are loaded from disk and no Gravatar HTTP request is made

#### Scenario: Cache miss falls through to network
- **WHEN** no cached avatar file exists
- **THEN** the app fetches from Gravatar and stores the result in the cache

#### Scenario: Empty cache file triggers network fetch
- **WHEN** the cached avatar file exists but is empty
- **THEN** the app fetches from Gravatar as if no cache were present

### Requirement: Successful network fetch written to disk cache
After a successful Gravatar fetch, the response bytes SHALL be written to `{cache_dir}/dtrpg/avatar`. The cache directory SHALL be created if it does not exist.

#### Scenario: Cache written after fetch
- **WHEN** a Gravatar fetch succeeds and returns image bytes
- **THEN** the bytes are written to `{cache_dir}/dtrpg/avatar`

#### Scenario: Cache directory created automatically
- **WHEN** `{cache_dir}/dtrpg/` does not exist
- **THEN** the directory is created before writing the cache file

### Requirement: Cache write failure is non-fatal
If writing the avatar cache to disk fails, the avatar SHALL still be displayed using the in-memory bytes. The failure SHALL be logged as a warning.

#### Scenario: Cache write error does not break avatar display
- **WHEN** writing to the cache file fails (e.g. permissions error)
- **THEN** the avatar is still displayed from the in-memory bytes and a warning is logged

