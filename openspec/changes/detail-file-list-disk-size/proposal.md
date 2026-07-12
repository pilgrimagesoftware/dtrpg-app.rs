## Why

The detail view's file size field always shows the catalog-reported `size_mb` from the API, even for a `Downloaded` item whose file already exists on disk. That number can be stale or approximate; once a file is actually present locally, the real, authoritative size is a cheap filesystem read away and is more useful to a user deciding whether to keep it.

## What Changes

- The detail view's file size field(s) always show the catalog-reported size, followed by the real on-disk size in parentheses (e.g. `"12.0 MB (11.8 MB on disk)"`) whenever a `Downloaded` item's file exists at its resolved on-disk path — the on-disk figure supplements the catalog figure rather than replacing it.
- Items that are `Cloud` (not downloaded), or whose file is missing on disk despite a `Downloaded` status, show only the catalog-reported size — unchanged from today.
- For a multi-file entry, the top-level file size field shows the *combined* catalog size across all of the entry's files (and the combined on-disk size, when resolvable), with a label that reflects it's a total rather than a single file's size.
- The multi-item entry's file list (Items table) gains a "Size" column showing each row's own catalog size plus its on-disk size, in the same format as the top-level field.
- Applies to the single-file entry's top-level file size field, the multi-file entry's top-level (combined) file size field, the Items table's new Size column, and each selected file's metadata panel.

## Capabilities

### New Capabilities

- `detail-file-size-on-disk`: resolving and displaying a downloaded file's actual on-disk size in the detail view (and the Items table), alongside catalog metadata, including combined totals for multi-file entries.

### Modified Capabilities

<!-- none -->

## Impact

- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: the single-item file size field (`render_metadata_table`), the per-file `render_item_metadata` field, and the Items table (`render_item_tier`) all gain an on-disk-size lookup and a combined-catalog-size computation for multi-file entries.
- `crates/dtrpg-ui/src/util/file_size.rs`: helper that resolves a file's on-disk path and returns its size in bytes, reused by all render sites; a formatting helper builds the `"{catalog} ({on_disk} on disk)"` composite string.
- `crates/dtrpg-ui/i18n/{en,fr,de}.yaml`: new keys for the "total file size" label, the "(X on disk)" suffix, and the Items table's "Size" column header.
- No change to `LibraryItemFile`, `LibraryController`, or any data model — this is a display-only enhancement that reads the filesystem at render time.
