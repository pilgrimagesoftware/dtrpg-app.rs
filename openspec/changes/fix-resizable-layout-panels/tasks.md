## 1. Fix Catalog Panel Configuration

- [x] 1.1 In `LibraryRootView::render` (`root_view.rs`), add `.size_range(px(280.)..Pixels::MAX)` to the catalog `resizable_panel()` so the catalog has an enforced minimum width of 280 px; leave no `initial_size` on the catalog (pure flex fill)
- [x] 1.2 Import `gpui::Pixels` in `root_view.rs` if not already present (needed for `Pixels::MAX`)

## 2. Fix Hidden Detail Panel

- [x] 2.1 Remove the always-present third `resizable_panel()` child (the detail panel) from the `h_resizable` group
- [x] 2.2 Conditionally add the detail `resizable_panel()` as a child only when `has_detail` is true, using `.when(has_detail, |group| group.child(detail_panel_child))` or an equivalent conditional child pattern; when false the group has only two children (sidebar and catalog) so the catalog fills to the window right edge

## 3. Build and Lint

- [x] 3.1 Run `cargo check --workspace` — no errors
- [x] 3.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings

## 4. Manual Verification

- [ ] 4.1 With no item selected, catalog fills all space from sidebar to window right edge with no blank column on the right
- [ ] 4.2 Selecting an item causes the detail panel to appear from the right; catalog compresses; no gap at window edge
- [ ] 4.3 Dragging the detail panel's left handle leftward stops when the catalog reaches approximately 280 px; catalog remains visible
- [ ] 4.4 Deselecting the item hides the detail panel and catalog returns to full width immediately with no gap
- [ ] 4.5 Sidebar drag handle still controls sidebar width correctly; catalog fills the remainder
