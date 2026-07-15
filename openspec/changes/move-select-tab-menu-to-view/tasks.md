## 1. Menu Bar

- [ ] 1.1 In `crates/dtrpg-ui/src/ui/app/mod.rs::build_menus`, remove the "Select Tab"
      `MenuItem::submenu(...)` block (and its preceding separator) from the Window
      `Menu`'s `.items([...])` array
- [ ] 1.2 Add the same "Select Tab" `MenuItem::submenu(...)` block to the View `Menu`'s
      `.items([...])` array, after the existing "Find in Library" item
- [ ] 1.3 Confirm the `tab_label` closure and the `SelectTab0`..`SelectTab9` action
      dispatch are unchanged — only the block's position in the returned `Vec<Menu>` moves

## 2. Build and Quality

- [ ] 2.1 `cargo check --workspace`
- [ ] 2.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 2.3 `cargo +nightly fmt --all -- --check`
- [ ] 2.4 `cargo test --workspace`

## 3. Manual Verification

- [ ] 3.1 Open the app; confirm the Window menu shows Minimize, Zoom, Show Activity, Show
      Alert History — no "Select Tab" submenu and no dangling trailing separator
- [ ] 3.2 Confirm the View menu shows Full Screen, Presentation, Sort, Find in Library,
      and a "Select Tab" submenu with ten items
- [ ] 3.3 With one detail tab open, confirm the View > Select Tab submenu's position-1
      item is enabled and labeled with that tab's title, and positions 2-9 are disabled
- [ ] 3.4 Click View > Select Tab > position-1 item; confirm that tab becomes active
- [ ] 3.5 Confirm `cmd-0`..`cmd-9` shortcuts still work identically to before this change
- [ ] 3.6 Confirm the checkmark in View > Select Tab tracks the currently active tab as
      tabs are switched via the tab strip
