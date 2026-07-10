## Why

The toolbar's catalog layout switcher (`render_layout_switcher` in `toolbar_view.rs`) shows text labels — "List", "Thumbs", "Grid" — inside a segmented `TabBar`. Text labels take up more horizontal space than icons for a three-way toggle whose options are visually self-explanatory once represented as pictograms, and the app already embeds a full icon set (`crates/dtrpg-core/src/app/assets.rs`) for exactly this kind of control but nothing uses it yet.

## What Changes

- Replace the three text-labeled `Tab` children in `render_layout_switcher` with icon-only tabs: `icons/list.svg` (List), `icons/gallery-thumbnails.svg` (Thumbs), `icons/layout-grid.svg` (Grid).
- Add a tooltip to each tab carrying the existing translated label (`toolbar.view_list`, `toolbar.view_thumbs`, `toolbar.view_grid`) so the mode name remains discoverable and localized.

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- `rust-main-window-library-layout`: Layout switcher tabs render as icon-only controls with tooltips instead of text labels.

## Impact

- `crates/dtrpg-ui/src/ui/views/toolbar_view.rs`: `render_layout_switcher` swaps `.label(t!(...))` for `.icon(Icon::empty().path("icons/....svg"))` and `.tooltip(t!(...))` on each `Tab`.
- No controller, data model, or translation-key changes — existing `toolbar.view_list`/`view_thumbs`/`view_grid` strings move from label to tooltip.
