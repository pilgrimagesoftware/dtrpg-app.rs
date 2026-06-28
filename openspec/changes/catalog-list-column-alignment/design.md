## Context

The list view has two rendering paths:

- **Ungrouped** (`CatalogPresentation::List, false`): `render_list_header` is a direct sibling of `uniform_list` in a flex-col parent. The header sits outside the scroll container; rows render inside it. Any difference between how the scroll container's content bounds are computed and how the sibling header's bounds are computed causes persistent column misalignment. This is the root structural problem — it cannot be reliably fixed by matching pixel widths manually.

- **Grouped** (`CatalogPresentation::List, true`): header and rows share the same flex-col parent (the group wrapper `div()`), so their available widths are identical. The only pre-existing misalignment was the `render_status` call having no fixed outer width, which caused `flex_1` to absorb extra space and shift every fixed-width column after it. That fix was applied in a previous attempt and is structurally correct for this case.

## Goals

- Replace the ungrouped list path with `DataTable<CatalogListDelegate>` from gpui-component. `DataTable` derives both header and row widths from the same `Column` definitions, eliminating the layout-context mismatch entirely.
- Replace the grouped list path with gpui-component's `Table` / `TableHeader` / `TableRow` / `TableHead` / `TableBody` / `TableCell` components, which carry proper table styling and make column width sharing explicit.
- Define column widths in one place (`fn list_columns() -> Vec<Column>`), used by both paths.
- Delete `render_list_header` and `render_list_row`.
- Revert the status-column-wrapper applied in the previous attempt (no longer needed).

## Non-Goals

- Adding sort interactivity through DataTable sort callbacks (tracked in `sort-menu-group-toggle`)
- Column resizing or reordering in the grouped path
- Dynamic title column width computed from viewport (future enhancement)
- Changing `render_status` itself (also used by thumb view)

## Implementation

### Shared column definitions

Add `fn list_columns() -> Vec<Column>` near the top of `catalog_view.rs`:

```rust
fn list_columns() -> Vec<Column> {
    vec![
        Column::new("title",     "Title / Kind").width(300.).min_width(150.).resizable(true),
        Column::new("publisher", "Publisher")   .width(130.).resizable(false),
        Column::new("system",    "System")      .width(110.).resizable(false),
        Column::new("pages",     "Pages")       .width(60.) .resizable(false),
        Column::new("size",      "Size")        .width(60.) .resizable(false),
        Column::new("added",     "Added")       .width(80.) .resizable(false),
        Column::new("status",    "")            .width(24.) .resizable(false).selectable(false),
        Column::new("reveal",    "")            .width(28.) .resizable(false).selectable(false),
    ]
}
```

The title column uses a fixed default of 300px (resizable). This replaces the previous `flex_1` which is not expressible in `DataTable`'s pixel-based column model.

### CatalogListDelegate

New struct (in `catalog_view.rs` or extracted to `catalog_view_delegate.rs`):

```rust
struct CatalogListDelegate {
    controller: Entity<LibraryController>,
    storage_root: PathBuf,
    columns: Vec<Column>,
}
```

- Colors are accessed via `dtrpg_ui::theme::theme(cx)` in render methods — no need to store a snapshot.
- `rows_count()`: `self.controller.read(cx).visible_items().len()`
- `column(ix)`: `self.columns[ix].clone()`
- `render_td(row_ix, col_ix)`: reads the item from `controller.read(cx).visible_items()[row_ix]`; renders the appropriate cell element per column.
- `render_tr(row_ix)`: default (no special per-row styling; selection is handled via `TableEvent`).

### CatalogView struct changes

- Add `catalog_list_table: Entity<TableState<CatalogListDelegate>>`
- In `new()`, create the delegate and `cx.new(|window, cx| TableState::new(delegate, window, cx))`.
- Subscribe to `TableEvent::SelectRow(row_ix)` on `catalog_list_table` to call `controller.select_item(item_id, cx)`.
- Keep `scroll_handle` — still used by thumbs and grid `uniform_list` paths.

### Ungrouped list render arm

Replace the current:
```
root.px(pad_side)
    .child(render_list_header(...))
    .child(uniform_list("catalog-list", ...).track_scroll(...).flex_1().min_h_0())
```

With:
```
root.px(pad_side)
    .child(
        DataTable::new(&self.catalog_list_table)
            .flex_1()
            .min_h_0()
            .with_size(Size::Size(density.row_text_height))
            .bordered(false)
            .scrollbar_visible(true, false)
    )
```

### Grouped list render arm

For each group, emit:
1. `render_group_header(...)` (unchanged)
2. A `Table` block built from `list_columns()` widths:

```rust
Table::new()
    .child(
        TableHeader::new().child(
            TableRow::new()
                .children(list_columns().into_iter().map(|c| TableHead::new().w(c.width).child(c.name)))
        )
    )
    .child(
        TableBody::new()
            .children(group.items.into_iter().map(|item| {
                TableRow::new()
                    .h(density.row_text_height)
                    .children(render_list_table_cells(&item, &colors, &density, ...))
            }))
    )
```

Add `fn render_list_table_cells(...)` returning an iterator of `TableCell` elements, one per column. This replaces `render_list_row`.

## Decisions

### Why DataTable for ungrouped, Table for grouped

`DataTable` is virtualized (uses `uniform_list` internally) and requires uniform row heights. Grouped mode interleaves group-header rows (different height) between data rows, which breaks `uniform_list`. The non-virtualized `Table` component is the right tool for grouped mode.

### Why 300px fixed width for title column

`DataTable` columns require explicit pixel widths. `flex_1` has no equivalent. A 300px resizable column is a reasonable default for a title field; the user can drag to widen or narrow. Computing it dynamically from the viewport requires subscribing to window resize events and updating the delegate — a worthwhile future improvement, deferred to keep this change small.

### Why delete render_list_header / render_list_row

Both are replaced by the delegate's `render_td` (for DataTable) and `render_list_table_cells` (for Table). Keeping dead code would be misleading.

## Risks / Trade-offs

- **Title column fixed width**: 300px may feel narrow on very wide windows. Resizability mitigates this.
- **DataTable theming**: DataTable carries gpui-component's table theme (background, hover state, border color). The visual appearance will change slightly — it should match the app's theme, but may differ from the custom-styled rows.
- **Row selection in DataTable**: Selection state is managed by `TableState` internally. The existing `on_click` handler on each row is replaced by subscribing to `TableEvent::SelectRow`.
