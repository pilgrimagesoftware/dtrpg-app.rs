## Why

The publisher list in the sidebar is currently sorted by item count descending, which makes it
hard to find a specific publisher quickly. Alphabetical order lets users scan for a name the
same way they would in any index or table of contents.

## What Changes

- `publisher_entries` sorts its result alphabetically by publisher name instead of by count descending.
- The grouped catalog view inherits the new order since it uses `publisher_entries` for sequencing.

## Capabilities

### New Capabilities

- `alphabetical-publisher-list`: Publisher entries in the sidebar and grouped catalog are ordered
  case-insensitively by name A → Z.

### Modified Capabilities

## Impact

- `dtrpg-ui`: `util/publisher.rs` — sort comparator in `publisher_entries`.
- Sidebar publisher list and grouped-by-publisher catalog view order both change.
- No API, SDK, or data model changes.
