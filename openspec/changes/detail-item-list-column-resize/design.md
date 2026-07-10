## Context

`render_item_tier` (`detail_panel_view.rs`) renders the multi-item entry's item list using the stateless `Table`/`TableHeader`/`TableRow`/`TableCell` primitives: a manually built header row plus one `TableRow` per file, each cell `flex_1` (equal width, non-resizable). Row selection is a workaround — a single wrapping `div` inside one `col_span(2)` `TableCell` carries the click handler for the whole row, because attaching separate `on_click` handlers to sibling cells left later rows unresponsive (see the comment at `detail_panel_view.rs:321-329`).

The catalog list view already solved the same class of problem with `DataTable<CatalogListDelegate>` (`catalog-list-column-sort-and-resize`): `TableState::col_resizable(true)` plus per-column `resizable(true)`, and row selection via `TableEvent::SelectRow` instead of hand-rolled click plumbing.

The complication specific to this change: `render_detail_tab_content` / `render_item_tier` are plain functions called fresh on every render pass from `RootView::render` (`root_view.rs:679`), taking `cx: &App` (immutable) and no `Window`. `DataTable` requires a persistent `Entity<TableState<D>>` created once with `cx.new(...)`, which needs mutable context — the catalog view gets this because `CatalogView` is itself a stateful `Entity` with a `new()` constructor. The detail tab has no equivalent persistent owner today.

## Goals / Non-Goals

**Goals:**

- Make the item list's Name, Type, and Status columns user-resizable via `DataTable`.
- Preserve current behavior: clicking a row selects it and updates the item metadata area in place; the selection is visually indicated.
- Give the item list `DataTable` a persistent home so its `TableState` (widths, selection) survives across re-renders of the detail tab.

**Non-Goals:**

- Column sorting (catalog list's `sortable(true)` + `perform_sort` is out of scope; the item list stays in file order).
- Persisting column widths across app restarts or across different entries (matches the catalog list's existing scope — in-memory only, reset per `TableState` instance).
- Changing the item metadata area or entry-tier layout.

## Decisions

### Cache the `TableState` entity in `TabsController`, keyed by entry id

`TabsController` already owns detail-tab lifecycle (`open_detail_tab`, `close_detail_tab`). Add `item_list_tables: HashMap<Arc<str>, Entity<TableState<ItemListDelegate>>>`. `render_item_tier` becomes a method-like helper that takes `tabs: &Entity<TabsController>`, `window: &mut Window`, `cx: &mut App` and:
- looks up the cached entity for `entry_id`; if present, reuses it
- if absent, creates one with `cx.new(|cx| TableState::new(delegate, window, cx).row_selectable(true).col_resizable(true).sortable(false))` and inserts it into the map (this insert needs `&mut TabsController`, done via `tabs.update(cx, |t, _| t.item_list_tables.insert(...))`)

`TabsController::close_detail_tab` removes the corresponding entry from `item_list_tables`, so the cache doesn't grow unbounded across a session.

_Alternative:_ cache in `LibraryController` instead of `TabsController`. Rejected — `TabsController` already owns tab-lifecycle cleanup (`close_detail_tab`), so cache eviction is free; `LibraryController` has no equivalent per-tab lifecycle hook.

_Alternative:_ make the detail tab its own `Entity`/`View` struct (mirroring `CatalogView`) instead of a bag of free functions. This is the architecturally cleaner long-term direction but is a much larger refactor of `detail_panel_view.rs` and `root_view.rs`'s tab-content dispatch; out of scope for a column-resize change. Left as a follow-up.

### `render_detail_tab_content` / `render_item_tier` signature changes from `cx: &App` to `(window: &mut Window, cx: &mut App)`

Required to call `cx.new(...)` for cache misses. `RootView::render` already has `window: &mut Window, cx: &mut Context<Self>` available at the call site (`root_view.rs:549`), so the caller can pass both through unchanged in kind.

### `ItemListDelegate` mirrors `CatalogListDelegate`'s read-through-controller pattern

```rust
struct ItemListDelegate {
    controller: Entity<LibraryController>,
    entry_id:   Arc<str>,
    columns:    Vec<Column>,
    user_widths: Vec<Option<Pixels>>,
}
```

`rows_count`/`render_td` read `item.files` live via `controller.read(cx).item_by_id(&entry_id)` rather than cloning the file list into the delegate — matches how `CatalogListDelegate` reads `visible_items_slice` each call, and means the row count naturally follows the current entry data without a manual invalidation step.

### Row selection: `TableEvent::SelectRow` drives `LibraryController::select_item_file`

Subscribe to the cached `TableState` entity (subscription set up once, at cache-creation time) the same way `CatalogView::new` subscribes to `catalog_list_table`:

```rust
TableEvent::SelectRow(row_ix) => {
    controller.update(cx, |ctrl, cx| ctrl.select_item_file(Arc::clone(&entry_id), *row_ix, cx));
}
```

`select_item_file` already exists and already drives the item metadata area — unchanged. On cache-creation (first render of a given entry's item list), if the controller already has a `selected_item_file` for this entry (e.g., tab was closed and reopened after a cache eviction), call `state.set_selected_row(ix, cx)` once so the visual selection matches.

### Columns: Name (resizable, flexible default width), Type (resizable, narrow default), Status (resizable, narrow default)

Same `.resizable(true)` pattern as the catalog list's `list_columns()` (`catalog_view.rs:156-166`). Default widths chosen so Name gets the majority of the available space initially, matching today's `flex_1`-on-Name-heavy visual balance, while still being fully user-adjustable.

## Risks / Trade-offs

- **[Risk]** Moving `render_item_tier` off pure-function semantics into a cache lookup adds a mutable-context requirement that ripples up through `render_detail_tab_content` and its one caller in `root_view.rs`. → Contained: only two call sites change signature, both already have `window`/mutable `cx` available.
- **[Risk]** Cache entries in `TabsController` could leak if a tab is closed via a path that skips `close_detail_tab`. → Existing `close_detail_tab` is already the single close path used by the tab strip; no new close path is introduced by this change.
- **[Trade-off]** No column-width persistence, same as the catalog list — a resized item-list column resets to default the next time the tab is closed and reopened (cache eviction on close). Acceptable per catalog list precedent; can be revisited together if/when the catalog list gains width persistence.
