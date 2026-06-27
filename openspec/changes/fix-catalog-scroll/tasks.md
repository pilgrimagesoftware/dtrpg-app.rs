## 1. Catalog scroll fix

- [ ] 1.1 In `catalog_view.rs`, add `use gpui_component::scroll::ScrollableElement;` to the imports
- [ ] 1.2 In `render_catalog`, change `.overflow_y_hidden()` on the `root` div to `.overflow_y_scrollbar()`

## 2. Sidebar scroll fix

- [ ] 2.1 In `sidebar_view.rs`, add `use gpui_component::scroll::ScrollableElement;` to the imports
- [ ] 2.2 In `render_sidebar`, change `.overflow_y_hidden()` on the scrollable body div to `.overflow_y_scrollbar()`

## 3. Verify

- [ ] 3.1 Run `cargo check --all-targets` and confirm no errors
- [ ] 3.2 Manually launch the app and confirm the catalog scrolls in list, thumbs, and grid layouts
- [ ] 3.3 Confirm the sidebar publisher list scrolls when it exceeds the sidebar height
