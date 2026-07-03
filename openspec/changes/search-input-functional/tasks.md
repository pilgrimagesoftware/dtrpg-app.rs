## 1. LibraryRootView — own InputState and subscribe

- [x] 1.1 Add `use gpui_component::input::{InputEvent, InputState};` to `root_view.rs`
- [x] 1.2 Add `search_input: Entity<InputState>` field to the `LibraryRootView` struct
- [x] 1.3 In `LibraryRootView::new`, create the input: `let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("Search…"));`
- [x] 1.4 Subscribe to `InputEvent::Change` on `search_input` to call `ctrl.set_search_query(value, cx)`: `cx.subscribe(&search_input, move |this, state, event: &InputEvent, cx| { if let InputEvent::Change = event { let v = state.read(cx).value().to_string(); this.controller.update(cx, |ctrl, cx| ctrl.set_search_query(v, cx)); } }).detach();`
- [x] 1.5 Store `search_input` in the `Self { ... }` return value

## 2. LibraryRootView::render — pass InputState to render_toolbar

- [x] 2.1 In `LibraryRootView::render`, replace the `search_query` local (currently `snap.search_query`) with `has_query = !snap.search_query.is_empty()` and `search_input = &self.search_input`
- [x] 2.2 Update the `render_toolbar` call: replace `&search_query` with `&self.search_input` and add `has_query` if the signature requires it (see task 3.2)

## 3. toolbar_view.rs — update render_toolbar and render_search

- [x] 3.1 Add `use gpui_component::input::{Input, InputState};` to `toolbar_view.rs`
- [x] 3.2 Change `render_toolbar` signature: replace `search_query: &str` with `search_input: &Entity<InputState>` and `has_query: bool`
- [x] 3.3 In `render_toolbar`, derive the `search_input` and `has_query` values from the new parameters and pass both to `render_search`
- [x] 3.4 Change `render_search` signature to: `fn render_search(input: &Entity<InputState>, has_query: bool, entity: Entity<LibraryController>, text_tertiary: gpui::Hsla)`
- [x] 3.5 In `render_search`, replace the `div().text_sm()...child(query)` child with `Input::new(input).placeholder("Search…")`
- [x] 3.6 Update the clear button's `on_click` to call `input.update(cx, |s, window, cx| s.set_value("", window, cx))` instead of (or in addition to) `ctrl.clear_search_query`; since the subscription handles the controller sync, the direct `clear_search_query` call can be removed
- [x] 3.7 Remove the now-unused `bg`, `border`, `text_primary` parameters from `render_search` and their corresponding arguments at the call site in `render_toolbar`

## 4. Verification

- [x] 4.1 Run `cargo check --all-targets` and confirm no compile errors
- [x] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings
- [x] 4.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [x] 4.4 Launch the app; click the search field and type — confirm the catalog filters to matching titles
- [x] 4.5 Confirm the "✕" button appears when text is present; clicking it clears the field and restores the full catalog
- [x] 4.6 Confirm the placeholder "Search…" appears when the field is empty
