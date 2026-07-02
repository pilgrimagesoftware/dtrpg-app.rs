## Why

The detail panel already has a ✕ close button, but it does not show up in practice. Two independent problems compound: it used `bg(hover)` — a light surface-toned color (e.g., `#EDE6D6` in Parchment) — which is illegible against arbitrary cover colors, and, more critically, it was fully occluded. GPUI paints sibling children in child-list order regardless of `absolute()` positioning — `absolute()` only affects layout position, not paint order — and the close button's `.child(...)` call was added to the panel's child list before the cover image and scrollable body, so those later-added children painted over it. The button never rendered, for any cover color or theme.

## What Changes

- The close button background changes from `colors.hover` to `colors.scrim`, a semi-transparent dark color defined consistently across all themes (e.g., `rgba(30,22,10,0.26)` in Parchment, `rgba(0,0,0,0.45)` in Ink). This ensures sufficient contrast against any cover color.
- The ✕ glyph color changes from `colors.text_secondary` (a mid-tone matched to the surface) to `colors.accent_on` (the light/surface color used for text drawn on dark backgrounds). In all four themes, `accent_on` is a near-white that reads clearly on a dark semi-transparent background.
- The close button's `.child(...)` block moves to the end of the panel's child chain, immediately before `.into_any_element()`, so it paints last and sits on top of the cover image and scroll body instead of being covered by them.
- No structural changes otherwise — the button remains absolutely positioned in the top-right corner at the same size (24 px circle).

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

<!-- none — this is a pure visual fix to an existing control; no requirement-level behavior changes -->

## Impact

- `dtrpg-ui/src/ui/views/detail_panel_view.rs` — two token swaps on the close button div (`hover` → `scrim` for background, `text_secondary` → `accent_on` for glyph color) and a reorder of the close button's position in the panel's child chain so it paints last
- No other files change
