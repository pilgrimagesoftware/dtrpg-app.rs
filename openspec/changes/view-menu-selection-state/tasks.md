## 1. Extract menu construction

- [x] 1.1 Add `ViewMenuState { presentation, sort, sort_direction, grouped }` to `dtrpg-ui/src/ui/app/mod.rs`
- [x] 1.2 Extract the inline `cx.set_menus([...])` call in `setup` into `pub fn build_menus(state: &ViewMenuState) -> Vec<Menu>`
- [x] 1.3 Call `build_menus(&ViewMenuState::default())` from `setup`

## 2. Checkmarks

- [x] 2.1 Apply `.checked(...)` to the three Presentation items based on `state.presentation`
- [x] 2.2 Apply `.checked(...)` to the four Sort items, normalizing `SortMethod::Custom { col_key }` (from column-header clicks) back to the corresponding named variant
- [x] 2.3 Apply `.checked(...)` to the Ascending/Descending items based on `state.sort_direction`
- [x] 2.4 Apply `.checked(...)` to the Group toggle based on `state.grouped`

## 3. Keep the menu bar live

- [x] 3.1 In `root_view.rs`'s existing `LibraryChanged` subscription, read the controller's current presentation/sort/sort_direction/grouped and call `cx.set_menus(build_menus(...))`

## 4. Verify

- [x] 4.1 Run `cargo test --all-features --workspace`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check`; all pass
- [ ] 4.2 Manually launch the app, open the View menu, switch presentation modes, and confirm the checkmark moves to the newly active item
- [ ] 4.3 Sort via the menu and via a column header click; confirm the menu checkmark tracks both
- [ ] 4.4 Toggle "Group by Publisher" and confirm the checkmark appears/disappears accordingly
