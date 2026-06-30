## Why

`sidebar_view.rs` is a hand-rolled flex column that re-implements navigation rows, collapsible section headers, active state, count badges, and scrolling from scratch. `gpui-component` ships a `Sidebar` component (`Sidebar`, `SidebarMenu`, `SidebarMenuItem`, `SidebarGroup`, `SidebarHeader`, `SidebarFooter`) that covers all of these, including built-in icon-collapsed mode and animated expand/collapse transitions.

## What Changes

- Replace `render_sidebar` in `sidebar_view.rs` with a `Sidebar<SidebarGroup<SidebarMenu>>` instance.
- Map the four smart-filter rows (All Titles, Recently Added, On This Device, In the Cloud) to `SidebarMenuItem` entries with `.suffix()` count badges and `.active()` state.
- Map the Publishers section to a `SidebarMenuItem` with `.click_to_toggle(true)` and per-publisher child menu items, replacing the `publishers_collapsed` field and `toggle_publishers_collapsed` method in `LibraryController`.
- Map the Collections section identically, replacing `collections_collapsed` and `toggle_collections_collapsed`.
- Replace the wordmark header div with `SidebarHeader`.
- Replace the storage stats + activity-button footer div with `SidebarFooter`.
- Drop the hand-rolled `render_nav_row` and `render_activity_button` helpers.
- The right detail panel (`render_detail_panel`) and the activity overlay (`render_activity_panel`) are out of scope: their content is not navigation and doesn't map to `SidebarMenuItem`.

## Capabilities

### New Capabilities

- `sidebar-nav`: Left navigation sidebar built on the gpui-component `Sidebar` component, with smart-filter items, collapsible publisher and collection submenus, count badges, and a wordmark header and storage footer.

### Modified Capabilities

<!-- none -->

## Impact

- `dtrpg-ui`: `sidebar_view.rs` (replaced), `controllers/library.rs` (`publishers_collapsed`, `collections_collapsed` fields and their toggle methods removed).
- Callers of `render_sidebar` (main window view) need updating if the function signature changes.
- No new crate dependencies — `gpui_component::sidebar` is already available via the existing `gpui-component` dependency.
