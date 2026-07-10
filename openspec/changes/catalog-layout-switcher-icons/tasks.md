## 1. Layout switcher icons

- [x] 1.1 In `toolbar_view.rs`, import `gpui_component::Icon`
- [x] 1.2 Replace each `Tab::new().label(t!("toolbar.view_list"))` (and `view_thumbs`/`view_grid`) with `.icon(Icon::empty().path("icons/list.svg"))` (and `gallery-thumbnails.svg`/`layout-grid.svg`)
- [x] 1.3 Add a closure-based `.tooltip(|window, cx| Tooltip::new(t!("toolbar.view_list").to_string()).build(window, cx))` (and `view_thumbs`/`view_grid`) to each tab so the mode name stays discoverable — `Tab` has no inherent string-based `.tooltip()` like `Button`, so the closure form (`gpui::StatefulInteractiveElement::tooltip`) is used instead
- [x] 1.4 Run `cargo check --all-targets` and `cargo clippy --all-targets --all-features -- -D warnings`

## 2. Verification

- [x] 2.1 Launch the app; confirm the layout switcher renders three icons (list, thumbnails, grid) with no visible text
- [x] 2.2 Hover each icon; confirm the tooltip shows the correct localized mode name
- [x] 2.3 Click each icon; confirm the corresponding presentation is selected and the active tab is visually indicated
