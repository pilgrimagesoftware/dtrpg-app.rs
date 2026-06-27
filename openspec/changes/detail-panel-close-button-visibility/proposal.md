## Why

The detail panel already has a ✕ close button, but it is effectively invisible in practice. It uses `bg(hover)` — a light surface-toned color (e.g., `#EDE6D6` in Parchment) — as its background and renders over the generative cover image, which can be any hue or lightness. The button is neither discoverable nor reliably legible against arbitrary cover colors.

## What Changes

- The close button background changes from `colors.hover` to `colors.scrim`, a semi-transparent dark color defined consistently across all themes (e.g., `rgba(30,22,10,0.26)` in Parchment, `rgba(0,0,0,0.45)` in Ink). This ensures sufficient contrast against any cover color.
- The ✕ glyph color changes from `colors.text_secondary` (a mid-tone matched to the surface) to `colors.accent_on` (the light/surface color used for text drawn on dark backgrounds). In all four themes, `accent_on` is a near-white that reads clearly on a dark semi-transparent background.
- No structural changes — the button remains absolutely positioned in the top-right corner at the same size (24 px circle).

## Capabilities

### New Capabilities

<!-- none -->

### Modified Capabilities

<!-- none — this is a pure visual fix to an existing control; no requirement-level behavior changes -->

## Impact

- `dtrpg-ui/src/ui/views/detail_panel_view.rs` — two token swaps on the close button div: `hover` → `scrim` for background, `text_secondary` → `accent_on` for glyph color
- No other files change
