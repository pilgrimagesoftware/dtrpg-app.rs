## 1. Controller state

- [x] 1.1 Add `advanced_details_open: HashMap<Arc<str>, bool>` to `LibraryController`,
      initialized empty alongside `selected_item_file`.
- [x] 1.2 Add `is_advanced_details_open(&self, entry_id: &str) -> bool` (defaults to
      `false` when absent).
- [x] 1.3 Add `toggle_advanced_details(&mut self, entry_id: Arc<str>, cx: &mut Context<Self>)`
      that flips the stored bool (inserting `true` if absent) and calls `cx.notify()`.

## 2. Advanced details section rendering

- [x] 2.1 Add `render_advanced_details(item: &LibraryItem, entity: Entity<LibraryController>,
      colors: &ColorTokens, cx: &App) -> impl IntoElement` to `detail_panel_view.rs`, using
      `gpui_component::collapsible::Collapsible` with the entry id's open state from
      `LibraryController`.
- [x] 2.2 Render the collapsed header as a clickable row (label + chevron icon that flips
      direction based on open state) wired to `toggle_advanced_details`.
- [x] 2.3 Render the expanded content as a `DescriptionList` with rows for stable id,
      numeric id, order product id, product id, and added-order value.
- [x] 2.4 Add a color swatch (small `div` with `.bg(...)` parsed from `item.color`) next to
      the hex value row, reusing the same color-parsing path `render_generative_cover` uses.
- [x] 2.5 Wire `render_advanced_details` into `render_detail_tab_content` as the final
      child of the scrollable content column, after the existing metadata table/item tier.

## 3. Localization

- [x] 3.1 Add `detail.advanced_details_heading` and per-field labels
      (`detail.field_stable_id`, `detail.field_numeric_id`, `detail.field_order_product_id`,
      `detail.field_product_id`, `detail.field_added_order`, `detail.field_cover_color`) to
      the locale files.

## 4. Tests

- [x] 4.1 Unit test: `is_advanced_details_open` defaults to `false` for an entry id never
      toggled.
- [x] 4.2 Unit test: `toggle_advanced_details` flips state on repeated calls for the same
      entry id and doesn't affect other entry ids.
- [x] 4.3 Unit test or snapshot: color swatch parsing falls back gracefully for a malformed
      `item.color` value (matching `render_generative_cover`'s existing fallback).
