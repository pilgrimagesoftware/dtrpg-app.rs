## 1. Sort Dropdown Icon

- [x] 1.1 In `dtrpg-ui/src/ui/views/toolbar_view.rs`'s `render_sort_selector`, add `.icon(Icon::empty().path("icons/arrow-down-up.svg"))` to the `Button::new("sort-selector")` chain, alongside the existing `.label(label)`, `.ghost()`, `.dropdown_caret(true)`, `.tooltip(...)` calls.

## 2. Verification

- [x] 2.1 Run `cargo check -p dtrpg-ui` and confirm zero errors
- [x] 2.2 Run `cargo test -p dtrpg-ui` and confirm all existing tests pass
- [x] 2.3 Run the app and confirm the sort dropdown button shows a leading sort icon alongside its label, unchanged across every sort method, sort direction, and layout mode
