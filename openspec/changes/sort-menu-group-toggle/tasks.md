## 1. Sort selector — add group toggle item

- [ ] 1.1 Add `grouped: bool` parameter to `render_sort_selector` signature (after `current: SortMethod`)
- [ ] 1.2 Clone the `entity` once more for the group toggle handler (following the existing `e`, `e2`, `e3`, `e4` pattern)
- [ ] 1.3 Append `.item(PopupMenuItem::separator())` to the menu chain after the "Pages" item
- [ ] 1.4 Append `.item(PopupMenuItem::new("Group by Publisher").checked(grouped).on_click(move |_, _, cx| { e5.update(cx, |ctrl, cx| ctrl.set_grouped(!grouped, cx)); }))` to the menu chain
- [ ] 1.5 Add `.dropdown_caret(true)` to the `Button::new("sort-selector")` chain (before `.dropdown_menu`)

## 2. Toolbar — remove group toggle

- [ ] 2.1 Remove the `render_group_toggle` function from `toolbar_view.rs`
- [ ] 2.2 In `render_toolbar`, remove the `.child(render_group_toggle(...))` call and stop passing the now-unused group-toggle arguments (`bg`, `border`, `text_primary`, `accent`, `accent_soft` if they were only used there)
- [ ] 2.3 Update the `render_sort_selector` call site in `render_toolbar` to pass `grouped` as the new second argument
- [ ] 2.4 Remove any imports or variables that are now unused (confirm with `cargo check`)

## 3. Verification

- [ ] 3.1 Run `cargo check --all-targets` and confirm no compile errors
- [ ] 3.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings
- [ ] 3.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 3.4 Launch the app; confirm the sort button shows the current sort label with a chevron indicator
- [ ] 3.5 Open the sort menu; confirm sort items appear, followed by a separator, followed by "Group by Publisher" with a checkmark reflecting current state
- [ ] 3.6 Toggle "Group by Publisher" from the menu; confirm the catalog groups/ungroups correctly
- [ ] 3.7 Confirm no standalone "Group" button appears in the toolbar
