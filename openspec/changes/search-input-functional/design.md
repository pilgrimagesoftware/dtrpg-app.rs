## Context

`render_search` in `toolbar_view.rs` renders a purely visual `div` that shows `query: String` as text. It has no focus, no keyboard handling, and no text mutation. The controller's `set_search_query` / `clear_search_query` methods exist but are never called from the UI (only `clear_search_query` is reachable via the "✕" button when a query is already set — which can never happen since no query can be entered).

`gpui_component::input::{Input, InputEvent, InputState}` provides a fully functional text input widget. `InputState` is an `Entity` that owns the text buffer and emits `InputEvent::Change` on every edit. `Input::new(&entity)` renders it. This pattern is already used in `login_view.rs`.

`InputState::new(window, cx)` requires a `&mut Window` and `&mut Context<Self>`, so it must be created during view construction — it cannot be created inside a free render function. `LibraryRootView` is the natural owner.

## Goals / Non-Goals

**Goals:**
- Wire the search field to a live `InputState` so the user can type
- Subscribe to `InputEvent::Change` in `LibraryRootView` to call `ctrl.set_search_query` on every keystroke
- Update the clear button to call `input_state.update(cx, |s, window, cx| s.set_value("", window, cx))` (the subscription handles the controller sync)
- Pass `&Entity<InputState>` from `LibraryRootView::render` → `render_toolbar` → `render_search`

**Non-Goals:**
- Debouncing (the controller already filters on a Vec in memory; no network call is involved)
- Highlighting matched text in results
- Scope selector (title-only vs. all fields)

## Decisions

### `InputState` lives in `LibraryRootView`

`LibraryRootView::new` already creates all long-lived entities (`controller`, `settings`, `catalog_view`, etc.). Adding `search_input: Entity<InputState>` follows the same pattern. The subscription wiring (`InputEvent::Change` → `set_search_query`) is also placed there, matching how `login_view.rs` wires `InputEvent::Change` → `ctrl.set_api_key`.

### Clear button uses `input_state.set_value("")` not `ctrl.clear_search_query`

Calling `set_value("")` on the `InputState` triggers `InputEvent::Change`, which the subscription turns into `ctrl.set_search_query("".into(), cx)`. This keeps a single code path for query updates and avoids the input widget and controller falling out of sync.

The `search_query` snapshot field in the `LibraryController` snapshot is still used to decide whether to show the "✕" button (non-empty query → show clear), so the existing `has_query` check remains valid.

### `render_search` drops `query: String`, `bg`, `border`, `text_primary`, `text_tertiary` parameters

The `Input` widget renders its own border, background, and text styling from the active theme. The outer wrapper `div` (icon + input + clear) retains the existing flex layout but the inner content becomes `Input::new(input).placeholder("Search…")`. The `bg`, `border`, and color parameters are no longer needed.

The clear button div still needs `text_tertiary` for its "✕" glyph color and `entity` for the clear-on-click handler. So `render_search` final signature is:

```rust
fn render_search(
    input: &Entity<InputState>,
    has_query: bool,
    entity: Entity<LibraryController>,
    text_tertiary: gpui::Hsla,
)
```

`has_query` is derived from `snap.search_query.is_empty()` in `render_toolbar` and passed in to control the clear button's visibility (same as before, but now the `query` string itself is not needed since `Input` renders its own text).

### `render_toolbar` removes the unused `search_query: &str` parameter

`render_toolbar` currently receives `search_query: &str` from the snapshot and passes it to `render_search`. After this change it instead receives `search_input: &Entity<InputState>` and passes that. The `has_query` bool is still derived from the snapshot.

## Risks / Trade-offs

- **`render_toolbar` signature change**: The function already has many parameters. Adding `&Entity<InputState>` in place of `&str` is a like-for-like swap.
- **`Input` widget styling**: `Input` uses the gpui-component theme internally. It may not match the existing pill-shaped border style exactly. If the visual difference is unacceptable, `Input` supports style overrides via `.style()`; but accept the default first and adjust in a follow-up if needed.
- **`window` parameter**: `render_toolbar` and `render_search` currently pass no `window`. The clear button's `on_click` receives `window: &mut Window` as its second arg (currently ignored with `_`). Using `window` inside `on_click` to call `input_state.update(...)` is fine — `on_click(|_, window, cx| { input_state.update(cx, |s, cx| s.set_value("", window, cx)); })`. No signature changes to `render_toolbar` or `render_search` are needed for window.
