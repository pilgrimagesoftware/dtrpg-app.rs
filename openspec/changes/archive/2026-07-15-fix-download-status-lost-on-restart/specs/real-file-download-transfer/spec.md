## ADDED Requirements

### Requirement: A successful download SHALL persist the catalog to the on-disk cache immediately
`LibraryController` SHALL write the updated catalog to the on-disk cache as
soon as a dispatched download completes successfully, rather than waiting for
the next live fetch to run `save_catalog_cache`.

#### Scenario: Downloaded status survives a restart without a live fetch
- **WHEN** a download completes successfully and the app is quit and
  relaunched before the on-disk cache would otherwise be rewritten (e.g. the
  cache is still within its freshness window and the auto-load policy skips
  the live fetch)
- **THEN** the item is loaded from the on-disk cache with `status:
  Downloaded` and its downloaded file's `downloaded` flag set to `true`

#### Scenario: A cancelled download does not trigger a cache write
- **WHEN** a dispatched download is cancelled before completion
- **THEN** `save_catalog_cache` is not called as a result of that download
