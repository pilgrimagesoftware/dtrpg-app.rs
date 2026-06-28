## Why

The search control in the toolbar displays a styled placeholder `div` that shows the current query string, but it is not an interactive text input — the user cannot type into it. There is no way to enter a search query from the UI. The controller already has `set_search_query` / `clear_search_query` methods and `item_matches_query` is implemented, so the underlying filtering works; the UI just never wires text entry to those methods.

## What Changes

- Replace the static `div`-based search display in `render_search` with a `gpui_component::input::Input` widget backed by an `Entity<InputState>`
- Own the `Entity<InputState>` in `LibraryRootView` and subscribe to `InputEvent::Change` to call `LibraryController::set_search_query` on every keystroke
- Update the "✕" clear button to call `InputState::set_value("")` on the widget (which triggers the subscription and syncs the controller)
- Pass the `Entity<InputState>` down from `LibraryRootView::render` through `render_toolbar` to `render_search`

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- `rust-main-window-library-layout`: Search control changes from a display-only div to a functional text input that filters the catalog

## Impact

- `root_view.rs` (`LibraryRootView`): gains `search_input: Entity<InputState>` field; `new()` creates the `InputState` and subscribes to `InputEvent::Change`; `render()` passes `&self.search_input` to `render_toolbar`
- `toolbar_view.rs`: `render_toolbar` gains an `Entity<InputState>` parameter; `render_search` signature changes from `(query: String, entity: Entity<LibraryController>, ...)` to `(input: &Entity<InputState>, entity: Entity<LibraryController>, ...)`; the inner `div` is replaced with `Input::new(input)`; the clear button calls `input_state.update(cx, |s, window, cx| s.set_value("", window, cx))`
- No controller or data model changes
