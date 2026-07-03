## 1. Delegate Sections API

- [ ] 1.1 `CatalogListDelegate::sections_count` returns the current `PublisherGroup` count
      when grouped, `1` when ungrouped
- [ ] 1.2 `CatalogListDelegate::items_count(section)` returns the item count for that
      publisher group
- [ ] 1.3 `CatalogListDelegate::render_section_header` renders publisher name + item count
- [ ] 1.4 Row lookups (`render_td` or equivalent) resolve the correct item given
      `(section, row)` instead of a flat index

## 2. Remove Hand-Rolled Grouped Path

- [ ] 2.1 Remove the `(CatalogPresentation::List, true)` match arm in `catalog_view.rs`
- [ ] 2.2 Remove `render_grouped_list_header` and `render_grouped_list_row`
- [ ] 2.3 Grouped list view renders through the same `DataTable::new(&self.catalog_list_table)`
      call as the ungrouped path

## 3. Build and Quality

- [ ] 3.1 `cargo check --workspace`
- [ ] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Toggle "group by publisher" in list view with a large catalog and confirm
      smooth virtualized scrolling
- [ ] 4.2 Confirm column widths/sort/resize behave identically in grouped and ungrouped
      list modes
- [ ] 4.3 Confirm each publisher section header shows the correct item count
