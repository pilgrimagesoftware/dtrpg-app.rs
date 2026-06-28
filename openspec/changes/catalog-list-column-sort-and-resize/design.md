## Context

The catalog list view uses `DataTable<CatalogListDelegate>` from gpui-component. `DataTable`'s `TableState` has `sortable` and `col_resizable` options that default to `true` but were explicitly disabled (`sortable(false)`, `col_resizable(false)`) in the original implementation. The sort model in `util/sort.rs` covers four named methods with no direction. The toolbar sort selector is a dropdown with four items and no direction controls.

`TableDelegate::perform_sort(col_ix, sort: ColumnSort, window, cx)` is called by `TableState` when the user clicks a column header. `ColumnSort` cycles: `Default` → `Descending` → `Ascending` → `Default`. The column header renders sort icons automatically (`SortAscending`, `SortDescending`, `ChevronsUpDown`) based on the column's `sort: Option<ColumnSort>` field in `col_groups`. `col_groups` is rebuilt by `TableState::refresh(cx)`, which calls `delegate.column(ix, cx)` for each column — so returning the correct `ColumnSort` from `column()` is the hook for syncing toolbar → header.

## Goals / Non-Goals

**Goals:**

- Enable column resize and header-click sort in `DataTable`.
- Add `SortDirection` and `SortMethod::Custom` to the sort model.
- Sync sort state bidirectionally: column header → controller, toolbar selector → column header indicators.
- Replace inline kind text in the title cell with a short text badge.
- Rename the "Title / Kind" column to "Title".
- Add direction items + "Custom" entry to the toolbar sort dropdown.

**Non-Goals:**

- Persisting column widths across app restarts.
- Server-side or API sorting.
- Adding sort to the grouped list view (which uses raw flex rows, not DataTable).
- Adding a dedicated "Sort By" column beyond the six data columns already in the DataTable.

## Decisions

### `SortDirection` enum + `sort_items` signature

Add to `util/sort.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection { #[default] Ascending, Descending }
```

`sort_items` gains a `direction: SortDirection` parameter. All named sort comparisons flip their comparison when `Descending`. The previous hardcoded descending direction for `PageCount` is removed in favour of the explicit parameter.

### `SortMethod::Custom { col_key: &'static str }`

`Custom` is a new variant. Named variants (Title, Publisher, DateAdded, PageCount) continue to sort the full item list by their existing logic. `Custom` carries the column key string (e.g. `"size"`, `"added"`, `"system"`) so that `sort_items` can dispatch to the correct comparator.

### Column → SortMethod mapping

When `perform_sort` fires, map `col_ix` to either a named `SortMethod` (if a named entry exists) or `Custom`:

| col_ix | key         | SortMethod           |
|--------|-------------|----------------------|
| 0      | `title`     | `SortMethod::Title`  |
| 1      | `publisher` | `SortMethod::Publisher` |
| 2      | `system`    | `SortMethod::Custom { col_key: "system" }` |
| 3      | `pages`     | `SortMethod::PageCount` |
| 4      | `size`      | `SortMethod::Custom { col_key: "size" }` |
| 5      | `added`     | `SortMethod::DateAdded` |

When `ColumnSort::Default` is returned (third click, "unsort"), set `SortMethod::Title` with `SortDirection::Ascending` to restore the default sort.

### Toolbar → header sync via `column()` + `refresh()`

`CatalogListDelegate::column(col_ix, cx)` reads `controller.snapshot()` and returns the column with `sort` set to the current `ColumnSort` indicator:

```rust
let is_sorted = match &snap.sort {
    SortMethod::Title         => key == "title",
    SortMethod::Publisher     => key == "publisher",
    SortMethod::DateAdded     => key == "added",
    SortMethod::PageCount     => key == "pages",
    SortMethod::Custom { col_key } => key == *col_key,
};
col.sort = Some(if is_sorted {
    match snap.sort_direction {
        SortDirection::Ascending  => ColumnSort::Ascending,
        SortDirection::Descending => ColumnSort::Descending,
    }
} else {
    ColumnSort::Default
});
```

`CatalogView::new()` subscribes to `LibraryChanged` and calls `self.catalog_list_table.update(cx, |state, cx| state.refresh(cx))`. This rebuilds `col_groups` from `column()`, propagating the controller sort state to the column header indicators. Column widths set by the user are lost on refresh (see Risks).

### `ColumnSort::Default` → no-sort handling

When `ColumnSort::Default` is returned to `perform_sort` (user clicked the sorted column a third time), treat it as "reset to default sort": set `SortMethod::Title`, `SortDirection::Ascending`. This avoids an "unsorted" state where the DataTable displays data in insertion order, which is confusing.

### Kind badge

Replace the kind text `div()` in `render_td(col=0)` with a small badge `div()`:

```rust
fn kind_badge(kind: &str) -> &'static str {
    match kind {
        s if s.contains("Core") => "CR",
        s if s.contains("Supplement") => "SUP",
        s if s.contains("Adventure") => "ADV",
        s if s.contains("Map") => "MAP",
        s if s.contains("Token") => "TOK",
        s if s.contains("PDF") || s.contains("Bundle") => "PDF",
        _ => "OTH",
    }
}
```

Style: `text_2xs`, `px(3)`, `py(1)`, `rounded(px(3))`, `bg(colors.surface_alt)`, `text_color(colors.text_tertiary)`, `flex_none()`. Same function used in grouped list rows.

### Toolbar sort dropdown

Add items after existing four:
1. `PopupMenuItem::separator()`
2. `PopupMenuItem::new("Custom").checked(snap.sort == Custom).disabled(true)` — read-only indicator
3. `PopupMenuItem::separator()`
4. `PopupMenuItem::new("Ascending").checked(direction == Ascending).on_click(...)` → `ctrl.set_sort_direction(Ascending, cx)`
5. `PopupMenuItem::new("Descending").checked(direction == Descending).on_click(...)` → `ctrl.set_sort_direction(Descending, cx)`

"Custom" is `disabled(true)` so the user cannot select it manually — it only appears as a visual indicator. Selecting Ascending/Descending from the menu changes direction for whichever sort method is currently active.

## Risks / Trade-offs

- **Column widths lost on `refresh()`**: calling `state.refresh(cx)` after every `LibraryChanged` event (including toolbar sort changes) will reset user-resized column widths back to their defaults, since `col_groups` is rebuilt from `Column::width`. → Accept for now; persisting widths is a separate future feature. The user must re-resize after changing sort via the toolbar.
- **`ColumnSort::Default` cycle step**: gpui-component's cycle includes Default (no sort). We skip it by forcing a fallback to Title/Ascending. This overrides the component's natural "clear sort" affordance. → Acceptable; an unsorted state is not useful in a library catalog.
- **`sort_items` with `Custom`**: comparators for `system` (line), `size` (size_mb), and `added` (added_order) must be added to `sort_items`. Each is straightforward (`a.line.cmp(&b.line)`, `a.size_mb.partial_cmp(&b.size_mb)`, `a.added_order.cmp(&b.added_order)`).
- **`col_key: &'static str` in `SortMethod::Custom`**: Using `&'static str` avoids heap allocation but requires the match in `perform_sort` to assign a literal string per col_ix. This is fine given the fixed column set.
