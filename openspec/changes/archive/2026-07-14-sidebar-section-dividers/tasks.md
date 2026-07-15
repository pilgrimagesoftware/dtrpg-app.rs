## 1. Add `SidebarContent` wrapper enum

- [x] 1.1 In `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`, add a local private enum `SidebarContent` with two variants: `Menu(SidebarMenu)` and `Separator`; derive `Clone`
- [x] 1.2 Implement `Collapsible` for `SidebarContent`: `Menu` delegates to the inner `SidebarMenu`; `Separator` returns `self` unchanged and `is_collapsed` returns `false`
- [x] 1.3 Implement `SidebarItem` for `SidebarContent`: `Menu` delegates `menu.collapsed(collapsed).render(id, window, cx)` returning `impl IntoElement`; `Separator` renders `div().h(px(1.)).w_full().my_1().bg(cx.theme().sidebar_border)`
- [x] 1.4 Add the required imports: `gpui_component::ActiveTheme` (for `cx.theme()`), `gpui_component::Collapsible`, `gpui_component::sidebar::SidebarItem`

## 2. Update `render_sidebar` to use `SidebarContent`

- [x] 2.1 Change the three `.child(lib_menu)`, `.child(pub_menu)`, `.child(col_menu)` calls to `.child(SidebarContent::Menu(lib_menu))`, `.child(SidebarContent::Separator)`, `.child(SidebarContent::Menu(pub_menu))`, `.child(SidebarContent::Separator)`, `.child(SidebarContent::Menu(col_menu))`
- [x] 2.2 Verify the `Sidebar::new("sidebar")` call now infers `Sidebar<SidebarContent>` without explicit type annotation

## 3. Build and Lint

- [x] 3.1 Run `cargo check --workspace` — no errors
- [x] 3.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings
- [x] 3.3 Run `cargo fmt --all` — no formatting changes

## 4. Manual Verification

- [ ] 4.1 A thin horizontal line is visible between the smart-filter items and the Publishers section
- [ ] 4.2 A thin horizontal line is visible between the Publishers section and the Collections section
- [ ] 4.3 The lines use the sidebar border color (matches the vertical sidebar edge color)
- [ ] 4.4 No layout shift or height change in either the smart-filter group, publishers, or collections sections
