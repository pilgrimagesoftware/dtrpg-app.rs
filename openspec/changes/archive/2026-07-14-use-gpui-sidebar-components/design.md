## Context

`sidebar_view.rs` contains `render_sidebar`, a ~340-line free function that builds the left navigation column by hand: a header div with the wordmark, a scrollable body with custom nav rows, custom collapsible section headers, and a footer with a storage summary and activity button. The collapsible state for Publishers and Collections is stored in `LibraryController` (`publishers_collapsed`, `collections_collapsed`) and toggled via controller methods.

`gpui-component` ships `Sidebar<E: SidebarItem>`, `SidebarHeader`, `SidebarFooter`, `SidebarGroup<E>`, `SidebarMenu`, and `SidebarMenuItem`. `SidebarMenuItem` handles active state, icon, label, suffix (count badge), click handler, and tree-style submenus with built-in expand/collapse state stored in `window.use_keyed_state`.

## Goals / Non-Goals

**Goals**
- Replace `render_sidebar` and its helpers (`render_nav_row`, `render_activity_button`) with `Sidebar<SidebarGroup<SidebarMenu>>`.
- Remove `publishers_collapsed`, `collections_collapsed`, `toggle_publishers_collapsed`, `toggle_collections_collapsed` from `LibraryController`.
- Keep all observable behaviour: filter selection, count badges, publisher/collection nav, activity button.

**Non-Goals**
- Adding icon-collapse mode to the sidebar (no toolbar button exists to trigger it; add if a toggle button is introduced later).
- Replacing `render_detail_panel` or `render_activity_panel` (neither is navigation; they don't map to `SidebarMenuItem`).

## Decisions

### Decision: `Sidebar<SidebarGroup<SidebarMenu>>` as the concrete type

`Sidebar<E>` is generic over a single item type. `SidebarGroup<SidebarMenu>` satisfies `E: SidebarItem + Clone` because `SidebarGroup` and `SidebarMenu` both implement those traits. Each top-level logical section (Library, Publishers, Collections) becomes one `SidebarGroup<SidebarMenu>`, with a `SidebarMenu` containing the `SidebarMenuItem` children.

_Alternatives considered_: Using `Sidebar<SidebarMenu>` (no group labels) or building a custom `SidebarItem` newtype. The group type preserves the visual section dividers and group labels that the current UI shows.

### Decision: Publishers and Collections as submenus, not controller state

`SidebarMenuItem` stores expand/collapse in `window.use_keyed_state`, which persists for the lifetime of the window — the same durability as the current controller boolean. Migrating to submenu state removes two fields and two methods from `LibraryController` with no change in user-visible behaviour.

_Alternatives considered_: Keep `publishers_collapsed` in the controller and pass it as a parameter to `.default_open()`. This would work but adds unnecessary round-trips through the controller for a purely visual state.

### Decision: `SidebarCollapsible::None`

No toolbar toggle button exists for the sidebar, so icon-collapse mode is not wired up. Setting `collapsible(SidebarCollapsible::None)` disables the feature cleanly. If a toggle button is added later, switching to `SidebarCollapsible::Icon` is a one-line change.

### Decision: Activity button in `SidebarFooter` via a custom `div`

`SidebarFooter` is a `ParentElement`, so any child element can be placed inside it. The existing activity button logic (label, colour, click handler) moves into a helper function that returns an element placed inside `SidebarFooter`. No API change needed.

## Risks / Trade-offs

- `Sidebar` uses `cx.theme()` tokens for background and border colours, which may differ visually from the current hand-rolled `ColorTokens`. The sidebar colour should be verified after migration and `ColorTokens` or theme configuration adjusted if needed.
- `SidebarMenuItem` submenu state is keyed by the `ElementId` passed to `render()`. The key is the list index from `Sidebar`'s internal `ListState`. If the number of groups changes across renders the keys are stable (they're positional), so collapse state is preserved as long as the group order does not change.

## Migration Plan

1. Remove `publishers_collapsed`, `collections_collapsed` fields and their toggle methods from `LibraryController`.
2. Update callers of `toggle_publishers_collapsed` / `toggle_collections_collapsed` (currently only `render_sidebar`).
3. Rewrite `render_sidebar` as a function returning `Sidebar<SidebarGroup<SidebarMenu>>`.
4. Move `render_activity_button` logic into the `SidebarFooter` content.
5. Delete `render_nav_row`.
6. Run `cargo check --workspace` and `cargo clippy --all-targets --all-features -- -D warnings`.
7. Manually verify all nav items, count badges, expand/collapse, and activity button in the running app.

## Open Questions

- Should the `SidebarGroup` labels ("Library", "Publishers", "Collections") be shown or hidden? The current sidebar shows "PUBLISHERS" and "COLLECTIONS" as text headers but not "Library". A hidden label (empty string) for the library group is the default assumption.
