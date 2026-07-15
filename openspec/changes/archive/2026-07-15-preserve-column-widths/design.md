## Context

`TableState::refresh()` is called every time `LibraryChanged` fires (sort, filter, data reload) so that column header sort indicators stay in sync. Internally, `refresh()` calls `prepare_col_groups()`, which rebuilds the `col_groups` `Vec` entirely from the values returned by `delegate.column(ix, cx)`. Our delegate's `column()` reads from the static `list_columns()` definitions, which always carry the original default widths. Any width changes the user made by dragging a column resize handle (stored only in the live `col_groups` entries) are overwritten.

`TableState` emits `TableEvent::ColumnWidthsChanged(Vec<Pixels>)` whenever the user finishes resizing a column. This gives us a hook to capture the current widths before the next refresh clobbers them.

## Goals / Non-Goals

**Goals:**
- Column widths set by the user survive any call to `TableState::refresh()`.
- No changes to `gpui-component` source; fix lives entirely in `catalog_view.rs`.
- Works for all columns, including status and reveal (fixed-width) columns.

**Non-Goals:**
- Persisting column widths across app restarts (disk/keychain storage).
- Preserving widths across view mode switches (list ↔ thumbs ↔ grid); widths reset when leaving and returning to list mode is acceptable for now.

## Decisions

### Store user widths in the delegate

**Decision**: Add `user_widths: Vec<Option<Pixels>>` to `CatalogListDelegate`. Each entry is `None` (use static default) or `Some(px)` (user-set). `column()` checks the slot and returns the user width when present.

**Rationale**: The delegate is the only component that bridges `TableState` and our domain. Storing widths here keeps the fix self-contained and means `refresh()` naturally picks up the preserved widths through the normal `column()` call path.

**Alternative considered**: Capture widths from `col_groups` before calling `refresh()` and write them back after. Rejected because `col_groups` is private to `TableState`; accessing it would require forking `gpui-component`.

### Subscribe to `ColumnWidthsChanged` in `CatalogView::new()`

**Decision**: In the `catalog_list_table` subscription block (alongside the existing `SelectRow` handler), handle `TableEvent::ColumnWidthsChanged(widths)` by updating `delegate.user_widths` through `table.update(cx, |state, cx| state.delegate_mut().user_widths = ...)`.

**Alternative considered**: Updating widths lazily inside `perform_sort`. Rejected because sort is not the only trigger for `refresh()`.

## Risks / Trade-offs

- **`delegate_mut()` availability**: `TableState` must expose `delegate_mut()`. Needs verification before implementation. If not available, use `TableState::update_delegate()` or similar.
- **Width count mismatch**: `ColumnWidthsChanged` delivers widths for all columns in order. If the column count ever changes (future feature), the stored vec may be stale. Mitigation: initialize `user_widths` to the column count at construction and ignore events with a length mismatch.

## Open Questions

- Does `TableState` expose `delegate_mut()` or a mutation callback? Check `state.rs` before implementing.
