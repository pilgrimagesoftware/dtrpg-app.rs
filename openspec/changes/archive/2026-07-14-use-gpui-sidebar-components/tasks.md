## 1. Controller Cleanup

- [x] 1.1 Remove `publishers_collapsed` and `collections_collapsed` fields from `LibraryController`
- [x] 1.2 Remove `toggle_publishers_collapsed` and `toggle_collections_collapsed` methods from `LibraryController`
- [x] 1.3 Remove `publishers_collapsed()` and `collections_collapsed()` accessors if they exist
- [x] 1.4 Update `render_sidebar` call site(s) to drop the now-removed collapsed parameters

## 2. Sidebar Imports

- [x] 2.1 Add `gpui_component::sidebar::{Sidebar, SidebarCollapsible, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem}` imports to `sidebar_view.rs`
- [x] 2.2 Remove imports that are no longer needed after the rewrite (e.g. `ScrollableElement`, manual div/flex helpers that are replaced)

## 3. Rewrite `render_sidebar`

- [x] 3.1 Replace the outer `div()` container with `Sidebar::new("sidebar")` using `.collapsible(SidebarCollapsible::None)` and `.side(Side::Left)`
- [x] 3.2 Build `SidebarHeader` containing the "Libri" wordmark div and pass it via `.header(...)`
- [x] 3.3 Build the Library `SidebarGroup<SidebarMenu>` with four `SidebarMenuItem` entries (All Titles, Recently Added, On This Device, In the Cloud), each with `.active(...)`, `.suffix(count)`, and `.on_click(...)`
- [x] 3.4 Build the Publishers `SidebarGroup<SidebarMenu>` with a single `SidebarMenuItem` "Publishers" parent that uses `.click_to_toggle(true)` and `.children([per-publisher items])`; each child item has `.active(...)`, `.suffix(count)`, and `.on_click(...)`
- [x] 3.5 Build the Collections `SidebarGroup<SidebarMenu>` the same way as Publishers
- [x] 3.6 Build `SidebarFooter` containing the storage stats line and activity button element, pass it via `.footer(...)`
- [x] 3.7 Delete the `render_nav_row` helper function
- [x] 3.8 Move activity button rendering into a standalone helper (or inline in footer) and delete the old `render_activity_button` helper

## 4. Build and Lint

- [x] 4.1 Run `cargo check --workspace` — no errors
- [x] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings` — no warnings

## 5. Manual Verification

- [ ] 5.1 Sidebar renders with "Libri" wordmark header
- [ ] 5.2 All four smart-filter items are visible with correct counts
- [ ] 5.3 Clicking each smart-filter item changes the active highlight and updates the catalog
- [ ] 5.4 Publishers section header is present and clicking it expands/collapses the publisher list
- [ ] 5.5 Clicking a publisher item filters the catalog to that publisher
- [ ] 5.6 Collections section header is present and clicking it expands/collapses the collection list
- [ ] 5.7 Clicking a collection item filters the catalog to that collection
- [ ] 5.8 Footer shows total count and storage size
- [ ] 5.9 Activity button shows correct state and clicking it toggles the activity panel
- [ ] 5.10 App startup does not produce any new panics or warnings in the console
