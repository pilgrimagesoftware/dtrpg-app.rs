## Context

The app's views were written before the `gpui-component-first` policy was established. Many interactive elements — buttons, tab strips, notification banners, metadata tables — were hand-rolled with `div()` trees rather than using gpui-component's themed equivalents. The pattern was codified during the `catalog-list-column-alignment` change, which also demonstrated that hand-crafted layouts produce alignment and styling bugs that gpui-component avoids by design.

All target components (`Button`, `TabBar`, `Alert`, `DescriptionList`) are already present in the `gpui-component` crate (already a dependency). No new crates are needed.

## Goals / Non-Goals

**Goals:**

- Replace every hand-crafted interactive button with `Button` from gpui-component.
- Replace the toolbar layout switcher and settings tab strip with `TabBar` (segmented and pill variants).
- Replace the notification banner with `Alert` (Warning variant, banner mode).
- Replace the detail panel metadata key-value rows with `DescriptionList`.
- Leave non-interactive structural divs (layout wrappers, spacers, panels) unchanged.

**Non-Goals:**

- Reworking the sidebar nav rows — `SidebarMenu`/`SidebarMenuItem` from gpui-component have a `SidebarItem` render trait that requires Window + App at render time; adapting the current stateless `render_sidebar` function is a larger refactor tracked separately.
- Replacing the settings modal card with `Dialog` — the current absolute-positioned overlay with custom sizing is sufficient; Dialog adds little beyond chrome.
- Touching the login view — it already uses `Input`, `Button`, and loading state correctly.
- Touching `catalog_view.rs` — just reworked with `DataTable`.

## Decisions

### `render_group_toggle` → `Button` toggle

The group toggle is a stateful `bool` displayed as a pill button with accent fill when active. `Button::new().ghost()` supports `.selected(grouped)` which drives the visual active state via the component's own hover/active/selected styling. This removes the manual `if grouped { accent_soft } else { bg }` color logic.

_Alternative:_ `Switch` — visually a toggle switch with a sliding animation. Rejected; a pill button matches the existing visual language in the toolbar better than a horizontal switch control.

### `render_layout_switcher` → `TabBar::new("layout-switcher").segmented()`

The layout switcher is a three-segment button group: List / Thumbs / Grid. `TabBar` with `segmented()` variant renders exactly this pattern. `selected_index` is derived from the current `CatalogPresentation`. `on_click` dispatches `set_presentation(mode, cx)` indexed from the fixed mode order.

_Alternative:_ Keep the hand-crafted row of divs with manual active styling. Rejected because the hand-crafted version cannot participate in TabBar's animated indicator and does not respect theme hover states.

### `render_tab_strip` (settings) → `TabBar::new("settings-tabs").pill()`

The settings tab strip is three named tabs. `TabBar` with `pill()` variant matches the intended rounded-pill tab style. `selected_index` derived from `active_tab as usize`. `on_click` dispatches `set_tab(TABS[ix], cx)`.

### `render_settings_button` → `Button::new("settings-gear").ghost()`

A 30×30 ghost icon button. `Button` with `.ghost()` and the `⚙` glyph (or `Icon::new(IconName::Settings)` if available) covers this without manual border/bg handling.

### Notification banner → `Alert::new("notice-{kind:?}", message).warning().banner(true).on_close(...)`

`Alert` has a `Warning` variant and a `banner(true)` mode which renders a full-width horizontal strip — exactly the notification banner's current layout. The `on_close` handler maps to the dismiss action. The action button (e.g., "Set Up Account") can be included via Alert's child mechanism.

One banner row per notice → one `Alert` per notice, stacked in a `div().flex().flex_col()` wrapper.

_Risk:_ Alert reads colors from `cx.theme()` (via `ActiveTheme`), not from `LibriTheme`. If the app's GPUI theme is not fully wired up, the Alert colors may not match. **Mitigation:** Accept the theme-driven color — it is the correct long-term approach. If theme registration is incomplete, colors will fall back to gpui-component defaults (still legible).

### Detail panel action buttons → `Button`

- **Read**: `Button::new("detail-read").primary().label("Read")` — primary variant, full width.
- **Download / Downloaded**: `Button::new("detail-download").outline().label(...)` — outline variant, toggles label.
- **Show in Finder**: `Button::new("detail-reveal").outline().label(platform_reveal_label())` — outline, only rendered when downloaded.

`Button` handles its own hover, active, disabled states. The manual `div().h(px(36.0)).px(...)...` builder chains are removed.

### Metadata table → `DescriptionList`

`DescriptionList` with `Axis::Horizontal` renders key → value pairs in a two-column layout. The current `render_metadata_table` is a manual loop over `(label, value)` pairs — a direct structural match. `DescriptionList::new().horizontal().bordered(true)` is the target form.

### Settings account logout button → `Button::new("logout-btn").danger().label("Log Out")`

The `danger()` variant (or `destructive()` if that is the variant name in this version of gpui-component) conveys destructive intent better than a plain accent-colored button.

## Risks / Trade-offs

- **Theme color drift**: Components using `cx.theme()` may render slightly differently from `LibriTheme` hand-crafted colors if the GPUI theme registration does not match. → Accept; the goal is to converge on theme-driven colors.
- **`TabBar` `on_click` index mapping**: The `on_click` callback receives a `usize` index. The mapping to `CatalogPresentation` or `SettingsTab` must match the `.child()` insertion order. → Fix the order as a constant array and document it.
- **`Alert` action child**: Not all Alert variants support arbitrary child elements in the same row as the close button. If the action button cannot be inlined, use Alert for the message/dismiss and place the action button as a sibling. → Verify during implementation.

## Open Questions

- Does the gpui-component `Button` expose a `.selected(bool)` builder that visually activates the button (for the group toggle)? If not, use `.custom(variant)` with conditional colors. → Check `button/mod.rs` during implementation.
- Is the `Alert` `on_close` callback sufficient for the action button, or does Action need to be a separate `Button` sibling? → Read `alert.rs` render output during implementation.
