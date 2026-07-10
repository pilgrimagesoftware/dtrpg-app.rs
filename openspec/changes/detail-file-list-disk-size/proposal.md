## Why

The detail view's file size field always shows the catalog-reported `size_mb` from the API, even for a `Downloaded` item whose file already exists on disk. That number can be stale or approximate; once a file is actually present locally, the real, authoritative size is a cheap filesystem read away and is more useful to a user deciding whether to keep it.

## What Changes

- For a `Downloaded` item whose file exists at its resolved on-disk path, the detail view's file size field(s) show the actual on-disk size instead of the catalog-reported `size_mb`.
- Items that are `Cloud` (not downloaded), or whose file is missing on disk despite a `Downloaded` status, continue to show the catalog-reported size — unchanged from today.
- Applies to both the single-file entry's top-level file size field and each row's file size field in a multi-item entry's file list.

## Capabilities

### New Capabilities

- `detail-file-size-on-disk`: resolving and displaying a downloaded file's actual on-disk size in the detail view, falling back to catalog metadata when no local file is present.

### Modified Capabilities

<!-- none -->

## Impact

- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: the single-item file size field (~`field_file_size` near the entry-level `DescriptionList`) and the per-file `render_item_metadata` field both gain an on-disk-size lookup.
- Likely a small helper (e.g. in `crate::util`) that resolves a file's on-disk path and returns its size in bytes, reused by both render sites.
- No change to `LibraryItemFile`, `LibraryController`, or any data model — this is a display-only enhancement that reads the filesystem at render time.
