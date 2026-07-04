## Why

`render_metadata_table` in `detail_panel_view.rs` shows a curated subset of a catalog
entry's fields (system, release year, format, file size, category, pages, added/updated
dates). `LibraryItem` carries several additional fields the table never surfaces —
`id`, `numeric_id`, `order_product_id`, `product_id`, `added_order`, and `color` — that
are only useful to a minority of users (support diagnostics, cache troubleshooting,
confirming which API record an entry maps to). There's currently no way to see these
values in the app without reading the cache file directly.

## What Changes

- Add a collapsed-by-default disclosure section below the existing metadata table (and
  below the item tier, for multi-item entries) in `render_detail_tab_content`, labeled
  something like "Advanced details".
- The disclosure section, when expanded, renders the remaining `LibraryItem` fields not
  already shown elsewhere: stable id, numeric id, order product id, product id, added
  order, and the generative cover color swatch/hex value.
- Disclosure expand/collapse state resets per tab open (not persisted across sessions or
  shared across entries) — a plain local toggle, not controller-owned state.
- Use `gpui-component`'s disclosure/collapsible primitive if one exists; otherwise a
  minimal expand/collapse `div` driven by local render state, consistent with this repo's
  gpui-component-first UI policy.
- Add new i18n keys under the `detail.*` namespace for the section label and each new
  field label.

## Capabilities

### New Capabilities

- `catalog-entry-detail-advanced-disclosure`: Defines the collapsed-by-default advanced
  details section in the catalog entry detail view, including which fields it shows and
  its default/reset behavior.

## Impact

- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: adds the disclosure section
  renderer and wires it into `render_detail_tab_content`.
- Locale files: new `detail.*` keys for the section label and advanced field labels.
- No changes to `LibraryItem`, the cache format, or any other capability's requirements —
  all fields already exist on the struct.
