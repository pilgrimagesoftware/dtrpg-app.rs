## Context

Both `catalog_view.rs` and `sidebar_view.rs` set `.overflow_y_hidden()` on their scroll containers. This correctly constrains layout height (preventing the container from expanding to fit all children) but clips overflow content without making it scrollable. The result is that items beyond the visible area are permanently hidden.

The `gpui_component` crate provides a `ScrollableElement` extension trait that adds `.overflow_y_scrollbar()` to GPUI `div` elements. This method is already used in `activity_panel_view.rs`.

## Goals / Non-Goals

**Goals:**
- Make the catalog content area and the sidebar body scrollable.

**Non-Goals:**
- Virtualized / windowed rendering (rendering only visible rows). The full item list is already rendered; this change just makes it reachable. Virtualization is a separate performance concern.
- Changing scroll behavior for any other panel (detail panel, settings overlay, etc.).

## Decisions

### Use `.overflow_y_scrollbar()` from `gpui_component::scroll::ScrollableElement`

Replace `.overflow_y_hidden()` with `.overflow_y_scrollbar()` at both sites. The extension trait is already a dependency via `gpui_component`; adding the import is the only additional change.

**Why**: Consistent with the existing pattern in `activity_panel_view.rs`. No new dependencies.

**Alternative**: Native GPUI `overflow_y_scroll()` (if it exists). Rejected in favor of the component library method, which also renders a visible scrollbar indicator, consistent with the rest of the UI.

## Risks / Trade-offs

- [Risk] Rendering all catalog items at once is O(n) DOM nodes. For catalogs with thousands of items this may cause sluggishness. → Mitigation: out of scope for this fix; a virtualized list (`uniform_list`) is a separate future change.
