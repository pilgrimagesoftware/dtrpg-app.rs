## 1. Actions

- [ ] 1.1 In `crates/dtrpg-ui/src/ui/actions.rs`, add a new `// Window menu tab-selection actions` block with `actions!(libri, [SelectTab0, SelectTab1, SelectTab2, SelectTab3, SelectTab4, SelectTab5, SelectTab6, SelectTab7, SelectTab8, SelectTab9]);`

## 2. Key bindings

- [ ] 2.1 In `crates/dtrpg-ui/src/ui/app/mod.rs`'s `setup()`, add `KeyBinding::new("cmd-0", SelectTab0, None)` through `KeyBinding::new("cmd-9", SelectTab9, None)` to the `cx.bind_keys([...])` call
- [ ] 2.2 Grep `ui/app/mod.rs` for any existing `cmd-0`..`cmd-9` bindings to confirm no collision before adding

## 3. Menu construction

- [ ] 3.1 Add `TabsSnapshot` import to `ui/app/mod.rs` and change `build_menus`'s signature to `pub fn build_menus(state: &ViewMenuState, tabs: &TabsSnapshot) -> Vec<Menu>`
- [ ] 3.2 In `build_menus`, build the ten tab-selection `MenuItem`s from `tabs.open_tabs`: for each position `0..=9`, look up `tabs.open_tabs.get(position)` (position `0` and `1` both resolve to `open_tabs.get(0)`, i.e. Catalog); if `Some(target)`, build an enabled `MenuItem::action(label, SelectTab<n>)` with a label derived the same way `tab_strip_view.rs` derives tab labels (`t!("tabs.catalog_tab")` for `TabTarget::Catalog`, `tabs.titles.get(id)` truncated via `truncate_with_ellipsis` for `TabTarget::Detail`, falling back to `t!("tabs.detail_tab_fallback")`); if `None`, build `MenuItem::action(fallback_label, SelectTab<n>).disabled(true)`
- [ ] 3.3 Append the ten tab-selection items to the existing `Menu::new(t!("menu.window_title")...)` block in `build_menus`, after the existing Show Alert History item
- [ ] 3.4 Update both call sites of `build_menus` (`setup()` in `ui/app/mod.rs`, and the `LibraryChanged` subscription in `root_view.rs`) to pass a `TabsSnapshot` — `setup()` uses `TabsController::new().snapshot()` equivalent (only Catalog open at startup, before `LibraryRootView` constructs its own `tabs` entity) or reads the same default; `root_view.rs` reads `self.tabs.read(cx).snapshot()`

## 4. Menu rebuild on tab change

- [ ] 4.1 In `root_view.rs`, change the existing `cx.subscribe(&tabs, |_this, _ctrl, _event: &TabsChanged, cx| { cx.notify(); })` handler to also call `cx.set_menus(build_menus(&self.last_menu_state.unwrap_or_default(), &tabs_ctrl.read(cx).snapshot()))` (reusing the last-known `ViewMenuState`, since tab changes don't affect presentation/sort checkmarks) before `cx.notify()`

## 5. Action handlers

- [ ] 5.1 In `root_view.rs`, add a small local helper (e.g. `fn tab_target_at(snapshot: &TabsSnapshot, position: usize) -> Option<TabTarget>`, or inline `snapshot.open_tabs.get(position).cloned()`) used by all ten handlers to resolve position -> `TabTarget`
- [ ] 5.2 Add ten `.on_action` handlers (`SelectTab0`..`SelectTab9`) to the view-builder chain (alongside `ReloadCatalog`/`AddCollection`/etc.), each cloning `self.tabs`, resolving the target for its fixed position (`0` and `1` -> position `0`, `n` -> position `n - 1` for `n` in `2..=9`), and calling `tabs.update(cx, |ctrl, cx| ctrl.activate(target, cx))` when `Some`; no-op when `None`

## 6. Localization

- [ ] 6.1 Add `tabs.select_tab_fallback` (or reuse `menu.window_title`-style naming, e.g. `menu.window_select_tab_empty`) to `crates/dtrpg-ui/i18n/en.yaml`, `de.yaml`, and `fr.yaml` for the disabled/no-tab-at-position menu item label
- [ ] 6.2 Confirm no new label is needed for occupied positions (they reuse each open tab's existing title/`tabs.catalog_tab`/`tabs.detail_tab_fallback` strings)

## 7. Verify

- [ ] 7.1 `cargo build --workspace --all-features`
- [ ] 7.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 7.3 `cargo test --workspace --all-features`
- [ ] 7.4 `cargo fmt --all -- --check`

## 8. Manual verification

- [ ] 8.1 Launch the app, open two catalog items as detail tabs (three tabs total: Catalog, detail 1, detail 2)
- [ ] 8.2 Press `cmd-0` from a detail tab and confirm the Catalog tab activates
- [ ] 8.3 Press `cmd-1` and confirm the Catalog tab activates
- [ ] 8.4 Press `cmd-2` and `cmd-3` and confirm each activates the corresponding detail tab
- [ ] 8.5 Press `cmd-4` (no tab at that position) and confirm nothing happens
- [ ] 8.6 Open the Window menu and confirm items for positions 0-3 are enabled and labeled with the correct tab titles, and positions 4-9 are visibly disabled
- [ ] 8.7 Close one detail tab, reopen the Window menu, and confirm the corresponding position's item is now disabled and the following positions have not shifted incorrectly
- [ ] 8.8 Click an enabled Window-menu tab-selection item directly and confirm it activates the same tab as its `cmd-<n>` shortcut
