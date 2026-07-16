## ADDED Requirements

### Requirement: Per-file downloaded state SHALL survive reconcile against a live fetch
When reconciling the existing catalog against a live SDK fetch, a catalog
item present in both SHALL keep the `downloaded` flag of any of its files
that also appears (matched by file id) in the live item's file list, and the
item's aggregate status SHALL be recomputed from the merged file list rather
than taken from the live fetch.

#### Scenario: A downloaded item's status survives a restart's live fetch
- **WHEN** the cached catalog contains an item with `status: Downloaded` and
  all its files marked `downloaded: true`, and a live fetch on startup
  returns the same item (with the API's default `downloaded: false` on every
  file)
- **THEN** after reconcile the item's files retain `downloaded: true` and its
  status remains `Downloaded`

#### Scenario: A live file with no cached counterpart is not downloaded
- **WHEN** the live fetch returns a file id for an item that was not present
  in the cached item's file list
- **THEN** that file's `downloaded` flag is `false` after reconcile

#### Scenario: A partially-downloaded item keeps its per-file state
- **WHEN** the cached item has one file marked `downloaded: true` and one
  marked `downloaded: false`, and the live fetch returns both file ids
- **THEN** after reconcile the first file is still `downloaded: true`, the
  second is still `downloaded: false`, and the item's status is `Cloud`
