## 1. Data Model

- [ ] 1.1 Add `LibraryItemFile` struct (id, name, format, size_mb, download_state)
- [ ] 1.2 Add `files: Vec<LibraryItemFile>` to `LibraryItem`, mapped from SDK `OrderProductFile`
- [ ] 1.3 Verify whether library-list SDK responses include `files`, or whether a per-entry detail
      fetch is required; adjust loading strategy accordingly
- [ ] 1.4 Confirm on-disk library cache handles the new field for previously cached entries
      (default to empty vec, no forced re-fetch)

## 2. Detail Tab Item List

- [ ] 2.1 Render a `DataTable`/`Table`-based persistent item list in `render_detail_tab_content`
      for entries with `files.len() > 1`
- [ ] 2.2 Wire item list row selection to update an item metadata area in place
- [ ] 2.3 Implement the empty/prompt state for the item metadata area when no item is selected
- [ ] 2.4 Collapse item metadata inline into the entry tier for single-item entries (no item list)

## 3. Multi-File Open Handling

- [ ] 3.1 Add a helper that opens/focuses the expanded detail tab and surfaces the item list for
      `OpenError::MultipleFilesRequireSelection`
- [ ] 3.2 Replace the warning-only handling at all five `MultipleFilesRequireSelection` call sites
      in `catalog_view.rs` and `detail_panel_view.rs` with the new helper

## 4. Catalog Browsing Indicator

- [ ] 4.1 Add item-count badge rendering to catalog list rows for multi-item entries
- [ ] 4.2 Add item-count badge rendering to catalog grid tiles for multi-item entries

## 5. Popover Verification

- [ ] 5.1 Confirm `item_popover_view.rs` renders only entry-level summary information for
      multi-item entries, with no item list or selection control

## 6. Verification

- [ ] 6.1 Test single-item entry: detail tab shows entry and item metadata inline, no item list
- [ ] 6.2 Test multi-item entry: detail tab shows item list; selecting each item updates metadata
- [ ] 6.3 Test item-count badge appears only on multi-item entries in list and grid
- [ ] 6.4 Test triggering open on a multi-item entry from the catalog view opens/focuses the item
      list instead of only logging a warning
- [ ] 6.5 Test item list scrollability with an entry that has many items
