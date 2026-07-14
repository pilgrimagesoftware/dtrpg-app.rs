## 1. Catalog scroll fix

- [x] 1.1 In `catalog_view.rs`, add `use gpui_component::scroll::ScrollableElement;` to the imports
- [x] 1.2 In `render_catalog`, change `.overflow_y_hidden()` on the `root` div to `.overflow_y_scrollbar()`

## 2. Sidebar scroll fix

Superseded: `sidebar_view.rs` no longer builds a raw div with `.overflow_y_hidden()` for
the scrollable body — it was migrated to `gpui_component::sidebar::Sidebar`, which has
its own internal `.vertical_scrollbar(&list_state)` handling. The sidebar already
scrolls by construction; there is no manual overflow toggle left to make.

- [x] 2.1 No `ScrollableElement` import needed in `sidebar_view.rs` — the `Sidebar` component owns its own scroll behavior
- [x] 2.2 No manual `.overflow_y_scrollbar()` call needed — confirmed via `gpui_component::sidebar::mod.rs` (`vertical_scrollbar(&list_state)` in the pinned dependency revision)

## 3. Verify

- [x] 3.1 Run `cargo check --all-targets` and confirm no errors
- [x] 3.2 Manually launch the app and confirm the catalog scrolls in list, thumbs, and grid layouts
- [x] 3.3 Confirm the sidebar publisher list scrolls when it exceeds the sidebar height
