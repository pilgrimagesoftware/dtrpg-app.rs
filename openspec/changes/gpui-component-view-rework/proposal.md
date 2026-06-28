## Why

Several views still contain hand-crafted `div()`-based implementations of common UI patterns (buttons, tabs, notification banners, metadata tables) where gpui-component already provides tested, themed equivalents. Replacing them reduces maintenance surface, aligns the app's visuals with the active gpui-component theme, and applies the `gpui-component-first` policy established during the catalog list column alignment work.

## What Changes

- **Toolbar group toggle**: Replace hand-crafted `div()` toggle with `Button` (toggle variant) from gpui-component.
- **Toolbar layout switcher**: Replace hand-crafted segmented `div()` row with `TabBar::new().segmented()`.
- **Toolbar settings gear button**: Replace hand-crafted `div()` icon button with `Button::new().ghost()`.
- **Settings tab strip**: Replace hand-crafted tab `div()` row in `render_tab_strip` with `TabBar::new().pill()`.
- **Notification banner**: Replace hand-crafted warning `div()` rows with `Alert` (`Warning` variant, `banner(true)`, `on_close` handler).
- **Detail panel action buttons** (Read, Download, Show in Finder): Replace hand-crafted `div()` action buttons with `Button` (primary and outline variants).
- **Detail panel metadata table**: Replace hand-crafted two-column `div()` rows in `render_metadata_table` with `DescriptionList` (horizontal axis, bordered).
- **Settings account logout button**: Replace hand-crafted `div()` button in `render_logout_button` with `Button` (danger or primary variant).

## Capabilities

### New Capabilities

- `gpui-component-buttons`: All interactive button elements use `gpui_component::button::Button` — styled consistently via the active theme, with built-in hover, active, and disabled states.
- `gpui-component-tabs`: Tab strip and segmented layout switcher use `gpui_component::tab::TabBar` — animated indicator, active-state driven by `selected_index`, single `on_click` callback.
- `gpui-component-alert`: Notification banners use `gpui_component::Alert` in `Warning` variant — themed icon, banner layout, dismiss callback.
- `gpui-component-description-list`: Item metadata is rendered via `gpui_component::DescriptionList` — consistent label/value layout, horizontal axis, theme-aware typography.

### Modified Capabilities

## Impact

- `crates/dtrpg-ui/src/ui/views/toolbar_view.rs` — `render_group_toggle`, `render_layout_switcher`, `render_settings_button`
- `crates/dtrpg-ui/src/ui/views/settings_view.rs` — `render_tab_strip`
- `crates/dtrpg-ui/src/ui/views/notification_banner_view.rs` — entire render function
- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs` — action buttons and `render_metadata_table`
- `crates/dtrpg-ui/src/ui/views/settings_account_view.rs` — `render_logout_button`
- No new dependencies; all components are already in the `gpui-component` crate.
