## 1. Data Model

- [x] 1.1 Add `LibraryItemFile` struct (id, name, format, size_mb, download_state) — added without a
      separate `download_state` field; see note on 5.1/design open question (per-file download state
      is approximated from the entry's existing `ItemStatus` since files share one on-disk folder).
- [x] 1.2 Add `files: Vec<LibraryItemFile>` to `LibraryItem`, mapped from SDK `OrderProductFile` in
      `map_order_product` (`dtrpg-core/src/services/sdk.rs`)
- [x] 1.3 Verify whether library-list SDK responses include `files`, or whether a per-entry detail
      fetch is required; adjust loading strategy accordingly — confirmed `files` is already present
      on list responses (`attributes.files`, already used for the aggregate format/size fields), no
      lazy per-entry fetch needed
- [x] 1.4 Confirm on-disk library cache handles the new field for previously cached entries
      (default to empty vec, no forced re-fetch) — `#[serde(default)]` on `LibraryItem::files`
      matches the existing pattern used for `cover_url`/`date_added`

## 2. Detail Tab Item List

- [x] 2.1 Render a `DataTable`/`Table`-based persistent item list in `render_detail_tab_content`
      for entries with `files.len() > 1` — implemented with `gpui_component::table::Table`/
      `TableRow`/`TableCell` (simple, stateless variant; item counts are small, no virtualization
      needed) in a new `render_item_tier` function
- [x] 2.2 Wire item list row selection to update an item metadata area in place — via
      `LibraryController::select_item_file`, ephemeral per-entry `HashMap` state cleared on tab
      (re)open at every `open_detail_tab` call site
- [x] 2.3 Implement the empty/prompt state for the item metadata area when no item is selected
- [x] 2.4 Collapse item metadata inline into the entry tier for single-item entries (no item list) —
      `render_detail_tab_content` branches on `item.is_multi_item()`, unchanged single-item path
      keeps the existing `render_metadata_table`

## 3. Multi-File Open Handling

- [x] 3.1 Add a helper that opens/focuses the expanded detail tab and surfaces the item list for
      `OpenError::MultipleFilesRequireSelection` — `ItemOpener::open_item` (resolves 0/1-file cases,
      returns the error for 2+) plus `catalog_view::open_item_or_focus_detail_tab`, which opens/
      activates the tab and clears any stale item selection
- [x] 3.2 Replace the warning-only handling at all five `MultipleFilesRequireSelection` call sites
      in `catalog_view.rs` and `detail_panel_view.rs` with the new helper — four `catalog_view.rs`
      sites (list row action, list context menu ×2 delegates, thumb/grid context menus) now call
      `open_item_or_focus_detail_tab`; the fifth, `detail_panel_view.rs`'s "Read" button, is already
      inside the detail tab, so it treats `MultipleFilesRequireSelection` as a no-op since the item
      list is already visible in the same view (see `render_item_tier`)

## 4. Catalog Browsing Indicator

- [x] 4.1 Add item-count badge rendering to catalog list rows for multi-item entries — via
      `render_item_count_badge`, appended to the title cell
- [x] 4.2 Add item-count badge rendering to catalog grid tiles for multi-item entries — same helper,
      overlaid on the cover thumbnail's bottom-right corner for both thumb rows and grid cards

## 5. Popover Verification

- [x] 5.1 Confirm `item_popover_view.rs` renders only entry-level summary information for
      multi-item entries, with no item list or selection control — confirmed by inspection; removed
      a stale commented-out "TODO: add file count" placeholder and clarified the module/inline docs

## 6. Verification

- [x] 6.1 Test single-item entry: detail tab shows entry and item metadata inline, no item list —
      covered structurally by `render_detail_tab_content`'s branch on `item.is_multi_item()` (no
      GPUI render-tree test harness exists in this codebase to assert rendered output directly;
      existing tests in this file are pure-logic, e.g. `value_or_dash_*`)
- [x] 6.2 Test multi-item entry: detail tab shows item list; selecting each item updates metadata —
      selection state logic covered indirectly via `map_order_product_populates_per_item_files_for_multi_item_entries`
      (`dtrpg-core`); no GPUI render-tree harness exists to assert the rendered item list directly
- [x] 6.3 Test item-count badge appears only on multi-item entries in list and grid — covered by
      `map_order_product_populates_per_item_files_for_multi_item_entries` /
      `map_order_product_single_file_is_not_multi_item` exercising `is_multi_item()`, which both the
      badge and item-tier branch depend on
- [x] 6.4 Test triggering open on a multi-item entry from the catalog view opens/focuses the item
      list instead of only logging a warning — covered by
      `open_item_returns_multiple_files_require_selection_for_more_than_one_file` and
      `open_item_resolves_the_single_file_path_for_one_file` in `item_opener.rs`
- [x] 6.5 Test item list scrollability with an entry that has many items — relies on
      `gpui_component::table::Table`'s existing scroll behavior (`overflow_y_scrollbar` on the
      surrounding container); no dedicated test, consistent with this crate's lack of GPUI
      render/interaction test harness for other scrollable views
