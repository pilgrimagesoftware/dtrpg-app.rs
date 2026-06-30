## Context

`Sidebar<E: SidebarItem>` in gpui-component stores children as `Vec<E>` — a homogeneous collection where `E` must implement `SidebarItem + Collapsible + Clone`. The sidebar already has three `SidebarMenu` children (smart filters, publishers, collections). Inserting a visual divider between them requires a child value that satisfies the same type constraint.

## Goals / Non-Goals

**Goals:**
- A thin horizontal line appears between the smart-filter group and the Publishers section.
- A thin horizontal line appears between the Publishers section and the Collections section.
- The dividers use the sidebar's existing border color so they match the theme.
- No external crate changes; all code lives in `dtrpg-ui`.

**Non-Goals:**
- Animating dividers during sidebar collapse (sidebar uses `SidebarCollapsible::None`).
- Adding divider support to gpui-component's `Sidebar` itself.
- Changing the spacing between sections.

## Decision

### Wrapper enum `SidebarContent`

Add a local enum in `sidebar_view.rs`:

```rust
#[derive(Clone)]
enum SidebarContent {
    Menu(SidebarMenu),
    Separator,
}
```

Implement `Collapsible` and `SidebarItem` for `SidebarContent`:
- `Collapsible::collapsed()` on `Separator` returns `self` unchanged (separators are layout-neutral).
- `SidebarItem::render()` on `Separator` returns a thin `div` with `h(px(1.))`, full width, and `bg(cx.theme().sidebar_border)` with `my_1()` vertical margin.
- `SidebarItem::render()` on `Menu(menu)` delegates to `menu.collapsed(collapsed).render(id, window, cx)`.

Change `render_sidebar` to build `Sidebar<SidebarContent>` and pass:

```
.child(SidebarContent::Menu(lib_menu))
.child(SidebarContent::Separator)
.child(SidebarContent::Menu(pub_menu))
.child(SidebarContent::Separator)
.child(SidebarContent::Menu(col_menu))
```

**Why not modify gpui-component**: Keeps the dependency surface small; a local enum is straightforward and avoids upstream coordination.

**Why not a top border on the second/third `SidebarMenu`**: `SidebarMenu` is a `RenderOnce` struct; its `Styled` impl exposes a `StyleRefinement` but borders added to the menu's outer element may not render where expected given the list-item wrapper the sidebar adds around each child. A dedicated separator child is explicit and predictable.

## Risks / Trade-offs

- [Collapsed icon mode] The sidebar uses `SidebarCollapsible::None`, so icon-collapsed state never applies here. If that changes later, `Separator` correctly passes `collapsed` through unchanged, so it will still render — the separator stays visible when the sidebar is icon-collapsed. This is acceptable; the sidebar currently can't collapse.
- [Theme change] The separator uses `cx.theme().sidebar_border`. If the theme doesn't define `sidebar_border`, it falls back to the default (transparent). Visual QA confirms correct rendering per the manual verification tasks.
