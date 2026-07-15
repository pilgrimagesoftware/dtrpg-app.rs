## 1. Toolbar — group toggle

- [x] 1.1 In `toolbar_view.rs`, replace `render_group_toggle` with a `Button::new("group-toggle")` call; verify `Button` supports `.selected(grouped)` or equivalent active-state builder — if not, use `.custom(variant)` with conditional colors matching the current accent/accent-soft logic
- [x] 1.2 Remove unused color parameters from `render_group_toggle` (bg, border, accent, accent_soft become Button internals); update the call site in `render_toolbar`

## 2. Toolbar — layout switcher

- [x] 2.1 In `toolbar_view.rs`, replace `render_layout_switcher` with `TabBar::new("layout-switcher").segmented()` with three `.child(Tab::new(...))` entries (List, Thumbs, Grid); set `selected_index` from current presentation
- [x] 2.2 Wire `on_click` on the `TabBar` to map `usize` index → `CatalogPresentation` and call `entity.update(cx, |ctrl, cx| ctrl.set_presentation(mode, cx))`
- [x] 2.3 Add per-tab tooltips ("List view", "Thumbnail view", "Grid view"); remove the old render function
- [x] 2.4 Update imports in `toolbar_view.rs`: add `gpui_component::tab::{Tab, TabBar}` (and any supporting types); remove unused bg/border/text color parameters from the old function

## 3. Toolbar — settings gear button

- [x] 3.1 In `toolbar_view.rs`, replace `render_settings_button` with `Button::new("settings-gear").ghost()` (icon or glyph child) with tooltip "Settings" and `on_click` wired to `settings.toggle(cx)`
- [x] 3.2 Remove the hand-crafted function and update the call site; clean up unused color parameters

## 4. Settings — tab strip

- [x] 4.1 In `settings_view.rs`, replace `render_tab_strip` with `TabBar::new("settings-tabs").pill()` carrying three `Tab` children (Account, Storage, File Openers); `selected_index` derived from `active_tab as usize`
- [x] 4.2 Wire `on_click` on the `TabBar` to map index → `SettingsTab` constant array and call `entity.update(cx, |ctrl, cx| ctrl.set_tab(tab, cx))`
- [x] 4.3 Remove the old function body; update imports

## 5. Notification banner — Alert

- [x] 5.1 Read `alert.rs` in gpui-component to verify the `banner(true)` API, `on_close` signature, and whether arbitrary child elements can sit alongside the close button
- [x] 5.2 In `notification_banner_view.rs`, replace the inner `div()` row for each notice with `Alert::new(id, message).warning().banner(true).on_close(...)` where `on_close` calls `auth_entity.dismiss_notice(kind, cx)`
- [x] 5.3 For the action button: if `Alert` supports an action child, add `Button` as a child; otherwise place a `Button` as a sibling adjacent to the `Alert` inside the per-notice wrapper div
- [x] 5.4 Update imports; ensure `AlertVariant`, `Alert` are imported from `gpui_component`

## 6. Detail panel — action buttons

- [x] 6.1 In `detail_panel_view.rs`, replace the Read action `div()` with `Button::new("detail-read").primary().label("Read")`
- [x] 6.2 Replace the Download/Downloaded `div()` with `Button::new("detail-download").outline().label(...)` with `on_click` wired to `entity_download.update(cx, |ctrl, cx| ctrl.toggle_download(&item_id, cx))`
- [x] 6.3 Replace the Show in Finder `div()` with `Button::new("detail-reveal").outline().label(platform_reveal_label())` with `on_click` wired to `reveal_in_file_manager`; render only when `is_downloaded`
- [x] 6.4 Update imports; remove manual `h(px(36.0)).px(px(16.0)).rounded(px(8.0))` styling from former button divs

## 7. Detail panel — metadata table

- [x] 7.1 In `detail_panel_view.rs`, read the `DescriptionList` and `DescriptionItem` APIs to confirm construction pattern (chained `.item(label, value)` or builder pattern)
- [x] 7.2 Replace `render_metadata_table` with `DescriptionList::new()` in horizontal mode, building one item per metadata row (System, Category, Format, Pages, File size, Released, Status) from the `LibraryItem`
- [x] 7.3 Update imports: add `gpui_component::description_list::{DescriptionList, DescriptionItem}` (or crate-root re-export path); remove the old function

## 8. Settings account — logout button

- [x] 8.1 In `settings_account_view.rs`, replace `render_logout_button` with `Button::new("logout-btn")` in danger/destructive variant with label "Log Out" and `on_click` wired to `entity.update(cx, |ctrl, cx| ctrl.request_logout(cx))`
- [x] 8.2 Remove unused `accent` / `accent_on` parameters from the function signature and its call site

## 9. Verification

- [x] 9.1 Run `cargo check --all-targets` — no compile errors
- [x] 9.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — no new warnings introduced by this change
- [x] 9.3 Launch the app; confirm the Group toggle button visually reflects active/inactive state correctly
- [x] 9.4 Confirm the layout switcher segmented control highlights the active presentation
- [x] 9.5 Confirm the settings tab strip highlights the active tab
- [x] 9.6 Trigger a notification (e.g., set `DTRPG_AUTH_STATE_OVERRIDE=unauthenticated`) and confirm the Alert banner renders and dismisses correctly
- [x] 9.7 Open a downloaded item's detail panel; confirm Read, Download, and Show in Finder buttons render and function correctly
- [x] 9.8 Confirm the metadata DescriptionList renders all rows with correct labels and values
