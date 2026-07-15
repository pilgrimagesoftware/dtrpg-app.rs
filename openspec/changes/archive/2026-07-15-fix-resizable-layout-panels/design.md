## Context

The `h_resizable("main-layout")` group in `LibraryRootView::render` has three panels: sidebar, catalog, and detail. `ResizablePanelGroup` places a drag handle at the left edge of every panel except the first, so handles appear on the catalog's left (sidebar/catalog boundary) and the detail's left (catalog/detail boundary). This is correct handle placement, but the current panel configuration has two problems:

1. The catalog panel has no size constraints, which is correct for flex fill, but the detail panel uses `.visible(has_detail)`. When `visible` is false, `RenderOnce` returns `div().id(...)` with no explicit `display: none` or zero `flex-basis`. The ResizableState still holds the panel's stored size, which gpui may still apply as `flex-basis`, leaving a blank column when the detail is hidden.

2. The catalog panel has no `size_range` minimum. With no lower bound, the detail panel can be dragged left until the catalog is zero-width or negative — the catalog becomes invisible behind the detail.

## Goals / Non-Goals

**Goals:**
- Catalog fills all available space; it has no self-managed size other than a minimum floor.
- Detail panel takes zero width when hidden (catalog fills to window edge).
- Catalog minimum width (280 px) is enforced; detail handle can't push past it.
- No new visible handles; existing two handles remain.

**Non-Goals:**
- Changing the sidebar's min/max range.
- Animating the detail panel in/out.
- Persisting catalog minimum (it is a hard UX floor, not a user preference).

## Decisions

### Catalog minimum via `size_range`

**Decision**: Add `.size_range(px(280.)..Pixels::MAX)` to the catalog `resizable_panel()`. The `ResizablePanel` enforces `min_w` from `size_range.start`, which prevents the detail handle from compressing the catalog below 280 px.

**Alternative considered**: Compute the detail panel's `size_range` maximum dynamically from the window width minus sidebar width minus 280 px. Rejected — this requires window-width tracking on every render and is fragile; `size_range` on the catalog is simpler and enforced by the layout engine.

### Hidden detail panel takes zero width via conditional child

**Decision**: Rather than relying on `.visible(false)` (which returns an empty div that may still hold its `flex-basis` from ResizableState), conditionally include the detail panel as a child of the `h_resizable` group only when `has_detail` is true. When false, the group has only two children (sidebar, catalog) and the catalog naturally fills to the right edge.

**Tradeoff**: When the detail panel is removed and re-added (item selected / deselected), `ResizableState` will re-initialise the panel's size from `initial_size`, discarding any drag-adjusted width. This is acceptable: the detail panel's persisted initial width from `UiPrefs` is restored each time it appears, which is the expected behavior.

**Alternative considered**: Keep three children always; set detail `flex-basis: 0` and `overflow: hidden` when hidden. Rejected — fighting the ResizableState's stored size with manual style overrides is fragile and couples our render code to internal ResizableState behavior.

### No change to handle count or position

The `h_resizable` group naturally produces handles on the left of panels 1 and 2 (catalog and detail). When detail is conditionally excluded, the group has only panels 0 (sidebar) and 1 (catalog), and only one handle appears (sidebar's right edge). This is correct: no catalog-specific handles exist at any time.

## Risks / Trade-offs

- [Detail width reset] Each time an item is selected (detail appears), its width resets to the `UiPrefs`-persisted value rather than the last drag-adjusted value from this session. → Acceptable given detail panel is transient; users primarily resize it in context and the persisted value is a reasonable default.
- [Catalog minimum vs. small windows] On a very narrow window (< 180 + 280 + 240 = 700 px), the sidebar, catalog minimum, and detail minimum together exceed available space. → `h_resizable` applies `min_w` constraints in order; behavior on extremely small windows is clamped by the layout engine and is considered an edge case.
