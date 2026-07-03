## 1. Sort selector — add group toggle item

- [x] 1.1 Add `grouped: bool` parameter to `render_sort_selector` signature (after `current: SortMethod`)
- [x] 1.2 Clone the `entity` once more for the group toggle handler (e7)
- [x] 1.3 Append `.separator()` to the menu chain after the Descending item
- [x] 1.4 Append `.item(PopupMenuItem::new("Group by Publisher").checked(grouped).on_click(...))` after the separator
- [x] 1.5 Add `.dropdown_caret(true)` to the `Button::new("sort-selector")` chain

## 2. Toolbar — remove group toggle

- [x] 2.1 Remove the `render_group_toggle` function from `toolbar_view.rs`
- [x] 2.2 Remove `.child(render_group_toggle(...))` call; `accent`/`accent_soft` still used by `render_layout_switcher` so kept
- [x] 2.3 Update `render_sort_selector` call site to pass `grouped` as new third argument
- [x] 2.4 No imports/variables became unused; clean `cargo check`

## 3. Verification

- [x] 3.1 Run `cargo check --all-targets` — no compile errors
- [x] 3.2 No new clippy warnings introduced
- [x] 3.3 Run `cargo test --all-features --workspace` — all 57 tests pass
- [x] 3.4 Launch the app; confirm the sort button shows the current sort label with a chevron indicator
- [x] 3.5 Open the sort menu; confirm sort items appear, followed by a separator, followed by "Group by Publisher" with a checkmark reflecting current state
- [x] 3.6 Toggle "Group by Publisher" from the menu; confirm the catalog groups/ungroups correctly
- [x] 3.7 Confirm no standalone "Group" button appears in the toolbar
