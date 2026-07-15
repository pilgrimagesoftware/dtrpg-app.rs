## 1. Delegate — store user widths

- [x] 1.1 Add `user_widths: Vec<Option<gpui::Pixels>>` field to `CatalogListDelegate` in `catalog_view.rs`; initialize it to `vec![None; list_columns().len()]` in the constructor
- [x] 1.2 In `CatalogListDelegate::column()`, when `user_widths[col_ix]` is `Some(px)`, return the column with `.width(px.0)` substituted for the static default; otherwise return the column unchanged

## 2. CatalogView — capture width changes

- [x] 2.1 In `CatalogView::new()`, add a handler for `TableEvent::ColumnWidthsChanged` in the existing `catalog_list_table` subscription block: when fired, write the received widths vec into `delegate_mut().user_widths` (map each `Pixels` to `Some(px)`)
- [x] 2.2 Guard against length mismatch: only apply the update if `widths.len() == delegate.user_widths.len()`

## 3. Verification

- [x] 3.1 Run `cargo check --all-targets` — no compile errors
- [x] 3.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings
- [x] 3.3 Launch the app in list view; resize a column; change the sort order; confirm the column width is unchanged
- [x] 3.4 Resize a column; change the sidebar filter; confirm the column width is unchanged
- [x] 3.5 Resize two columns independently; change sort; confirm both widths are retained and neither affected the other
