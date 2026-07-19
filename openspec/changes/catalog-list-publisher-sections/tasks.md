## 1. Virtualized Grouped Delegate

`gpui-component`'s `TableDelegate` (backing `DataTable`) has no native row-sections API —
only `ListDelegate` (backing `List`) does, and `CatalogListDelegate` implements
`TableDelegate` to get `DataTable`'s column width/sort/resize behavior. `sections_count` /
`items_count` / `render_section_header` as originally proposed don't exist for `Table` on
either the pinned `gpui-component` revision or upstream `HEAD` (verified directly against
both). The grouped list instead gets a second `TableDelegate` (`GroupedCatalogListDelegate`)
backed by a flattened row list (`GroupedRow::Header` / `GroupedRow::Item`) built from
`group_by_publisher`, rendered through its own `DataTable` instance.

- [x] 1.1 `GroupedCatalogListDelegate::rows_count` returns the flattened row count (one
      header row per publisher group plus one row per item)
- [x] 1.2 `CatalogView::grouped_items` builds the flattened `GroupedRow` list from
      `group_by_publisher` and pushes it into `catalog_grouped_list_table`
- [x] 1.3 `GroupedCatalogListDelegate::render_td` renders publisher name + item count for
      `GroupedRow::Header` rows (column 0; other columns render blank so column widths stay
      aligned with data rows)
- [x] 1.4 Row lookups (`render_td`, `context_menu`, `TableEvent::SelectRow` /
      `DoubleClickedRow` handlers) resolve the correct row via `GroupedCatalogListDelegate::row_at`
      given the flat row index, skipping selection/context-menu actions on header rows

## 2. Remove Hand-Rolled Grouped Path

- [x] 2.1 Remove the hand-rolled `div()`-tree grouped list rendering in `catalog_view.rs`
- [x] 2.2 Remove `render_grouped_list_header` and `render_grouped_list_row`
- [x] 2.3 Grouped list view renders through `DataTable::new(&self.catalog_grouped_list_table)`
      — a second, independently virtualized `DataTable` instance (not the same instance as
      the ungrouped path, since `TableDelegate` has no sections API to multiplex both
      presentations through one delegate) sharing the same `list_columns()` definitions.
      `TableEvent::ColumnWidthsChanged` on either table now propagates its widths into the
      other table's delegate so a resize made in one presentation matches the other.

## 3. Build and Quality

- [x] 3.1 `cargo check --workspace`
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Toggle "group by publisher" in list view with a large catalog and confirm
      smooth virtualized scrolling
- [ ] 4.2 Confirm column widths/sort/resize behave identically in grouped and ungrouped
      list modes
- [ ] 4.3 Confirm each publisher section header shows the correct item count
