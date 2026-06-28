## 1. Add `set_catalog` helper to `LibraryController`

- [ ] 1.1 Add `fn set_catalog(&mut self, items: Vec<LibraryItem>, cx: &mut Context<Self>)` to `LibraryController` that assigns `self.catalog = items`, recomputes `self.section_counts`, `self.publishers`, and calls `cx.emit(LibraryChanged)`

## 2. Replace per-page update loop with accumulate-then-swap

- [ ] 2.1 Declare `let mut live_items: Vec<LibraryItem> = Vec::new();` in the spawn closure, before the receive loop
- [ ] 2.2 Replace the per-page `this.update` call (including the `first_page` / `catalog.clear()` logic) with `live_items.extend(items)` — no UI update per page
- [ ] 2.3 Remove the `first_page` flag and the `is_first` / `catalog.clear()` branches entirely
- [ ] 2.4 After the receive loop exits, on the success path, call `this.update(async_cx, |ctrl, cx| ctrl.set_catalog(live_items, cx)).ok();` before saving the cache

## 3. Update the save-cache path

- [ ] 3.1 Change the cache-save to clone from `ctrl.catalog` after the `set_catalog` call (it already does this — verify the ordering is correct after the refactor)

## 4. Verification

- [ ] 4.1 Run `cargo check --all-targets` and confirm no compile errors
- [ ] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings
- [ ] 4.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 4.4 Launch the app with an existing disk cache; verify the cached catalog is visible immediately and remains stable throughout the SDK fetch, with no flash or shrink
- [ ] 4.5 Simulate a fetch failure (e.g. network offline); verify the cached catalog remains in the UI after the error is shown in the activity panel
