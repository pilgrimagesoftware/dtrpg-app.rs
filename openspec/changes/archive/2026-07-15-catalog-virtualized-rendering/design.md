## Context

The current `render_catalog` function in `catalog_view.rs` is a pure free function that takes a `&LibrarySnapshot` and builds the complete catalog DOM. All three layouts (list, thumbs, grid) iterate over every item in `snapshot.items` and produce a child element for each one. GPUI then measures, lays out, and paints every element every frame — even those scrolled out of view.

With a typical catalog of 1 000 items, this means O(1 000) GPUI element nodes per render. Because GPUI re-renders on every user interaction (cursor move, key press, scroll), the main thread is constantly doing O(n) layout work proportional to the total catalog size. The result is visible frame drops and input lag after a large catalog loads.

GPUI provides `uniform_list` (in `gpui::elements::uniform_list`) specifically for this case. It takes an item count and a render closure, tracks scroll offset, and calls the closure with only the visible `Range<usize>` — typically 20-40 rows on screen. Frame layout cost becomes O(visible rows).

## Goals / Non-Goals

**Goals:**
- Reduce per-frame GPUI layout cost from O(total items) to O(visible rows) for list and thumbs layouts.
- Reduce per-frame layout cost for grid layout from O(total items) to O(visible rows × items-per-row).
- Maintain scrollability and all existing item interactions (click to open, context menu).
- Maintain the existing visual appearance of each layout.

**Non-Goals:**
- Virtualizing grouped mode (grouped by publisher). Group headers break the uniform-height requirement. Grouped mode keeps existing non-virtualized rendering.
- Changing the `LibraryController` or any service/data layer code.
- Adding any new user-visible features or controls.

## Decisions

### CatalogView becomes a GPUI View entity

`uniform_list` requires a `UniformListScrollHandle` to track scroll position. This handle must persist across renders and must be stored somewhere with view lifetime. A free function cannot hold state; a GPUI `View` can.

`CatalogView` becomes a struct implementing `gpui::Render`. It stores:
- `controller: Entity<LibraryController>` — accessed inside the `uniform_list` render closure to read items by index
- `scroll_handle: UniformListScrollHandle` — one per layout type (or one shared, reset on layout switch)

`LibraryRootView` (or `LibraryView`, wherever `render_catalog` is currently called) creates a `CatalogView` entity via `cx.new(|_| CatalogView { ... })` and renders it with `catalog_view.render(cx)` or by including it as a child element. If `LibraryRootView` rebuilds the snapshot on every render anyway, it passes the snapshot to `CatalogView::render` as a parameter rather than storing it.

**Alternative**: Store `UniformListScrollHandle` directly on `LibraryRootView`. Rejected — mixes scroll state for the catalog list into the root view, which already manages login state, sidebar, toolbar, and activity panel. Keeping scroll state in `CatalogView` is cleaner.

**Alternative**: Use `gpui_component`'s `VirtualList` (variable-height). The variable-height path is more complex, requires a `VirtualListScrollHandle`, and the catalog layouts all have uniform row height per layout mode. `uniform_list` is sufficient and simpler.

### Items are read from the controller by index inside the uniform_list closure

The `uniform_list` render closure receives `(range: Range<usize>, window: &mut Window, cx: &mut App) -> Vec<R>`. Inside it, `CatalogView` calls `cx.read_entity(&self.controller, |ctrl, _| ctrl.visible_items_slice(range.clone()))` to get only the items it needs.

This avoids cloning the full visible items Vec on every render — only the slice for visible rows is cloned.

**Alternative**: Capture a `Arc<Vec<LibraryItem>>` snapshot in the render method, then index into it inside the closure. This requires the `Arc` to live for `'static` (the closure is `'static + Fn(...)`). A captured `Entity<LibraryController>` is `'static` and satisfies this requirement cleanly.

`LibraryController` needs a `pub fn visible_items_slice(&self, range: Range<usize>) -> Vec<LibraryItem>` method that returns `self.visible_items()[range].to_vec()`. This builds on the visible-items cache from the `improve-catalog-load-performance` change (if applied) or calls `visible_items()` directly.

### Grid layout uses row-based chunking

The grid renders multiple items per row. Rather than treating each item as a `uniform_list` row, each visual row (a group of N cards) is one `uniform_list` item. This preserves uniform height per list item.

- `items_per_row` is computed from available width and card width (existing logic stays).
- `row_count = items.len().div_ceil(items_per_row)` is the item count passed to `uniform_list`.
- Inside the render closure, range `row_start..row_end` maps to items `(row_start * items_per_row)..(row_end * items_per_row).min(items.len())`.

**Challenge**: `uniform_list` needs `item_count` at construction time, before layout is done, so `items_per_row` must be derived from a fixed assumption (e.g., the last known available width or a default column count). If the window is resized the item count may change, requiring a redraw. GPUI's normal invalidation handles this: `CatalogView::render` is called again, `uniform_list` is reconstructed with the new count.

### Grouped mode is excluded

Grouped by publisher renders headers between groups. These headers have different heights than item rows. `uniform_list` requires all items to have the same height. Rather than building a flat "row items" enum (header vs. item row) which complicates the render path significantly, grouped mode falls back to the existing `div()` + `for_each` approach. This is acceptable because users are more likely to see large item counts in ungrouped views.

## Risks / Trade-offs

- [Risk] `visible_items_slice` calls `visible_items()` which may clone the full `Vec` before slicing. If the `improve-catalog-load-performance` cache change has been applied, subsequent calls within the same render are cached. If not, each `uniform_list` render call pays the O(n) filter cost. → Mitigation: `CatalogView` calls `visible_items()` once at the top of `render()`, stores the count, and the closure captures `Entity<LibraryController>` and reads the slice; within one render pass the cache is valid.
- [Risk] `items_per_row` for grid mode is not known until GPUI's layout pass, but `uniform_list` needs `item_count` before layout. → Mitigation: Use a stored `last_known_width` on `CatalogView`, updated via a `ContentMask` or `Bounds` inspection in the previous render. On first render, assume a default column count of 4 (a reasonable minimum). A single off-by-one render is invisible to the user.
- [Risk] `UniformListScrollHandle` state is per-layout. Switching between list/thumbs/grid could jump the scroll position. → Mitigation: Use separate scroll handles for each layout mode, or reset the handle on layout-mode change. The simplest implementation: one `UniformListScrollHandle` per `CatalogView`, reset to top when layout mode changes.
- [Risk] The existing `render_catalog` call sites may pass parameters that need to be threaded into `CatalogView`. → Mitigation: `CatalogView::render` receives the same `LibrarySnapshot` and `cx`; click handlers are closures that capture `Entity<LibraryController>` or emit events, unchanged from current code.
