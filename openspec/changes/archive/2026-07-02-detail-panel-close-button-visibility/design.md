## Context

The close button sits absolutely at `top(px(12.0)).right(px(12.0))` over the generative cover image. The cover background is derived from the item's `color` field — it can be any hue and any lightness. The current button styling uses:

```rust
.bg(hover)           // e.g., Parchment: #EDE6D6 — a near-opaque light beige
.text_color(text_secondary)  // e.g., Parchment: #5B5346 — a mid warm-brown
```

`hover` in every theme is a light, near-opaque surface color. Against a dark or saturated cover, a light opaque circle would be visible — but against a light cover (many TTRPG products use cream or gold backgrounds), the circle disappears entirely. The mid-tone `text_secondary` glyph is similarly unreliable.

The `scrim` token exists for exactly this use case: a dark semi-transparent overlay that reads against any cover background. In all four themes it is a dark color at 26–45 % opacity, providing enough tinting to make a white glyph legible without fully blocking the cover below. `accent_on` is the light, high-contrast color always used for text on accent/dark backgrounds.

## Goals / Non-Goals

**Goals:**
- Make the close button reliably visible on any generative cover color in all four themes.

**Non-Goals:**
- Changing the button's size, position, shape, or behavior.
- Adding hover or pressed states (those can be a follow-on).
- Adding a text label to the button.

## Decisions

### Use scrim for background, accent_on for glyph

`colors.scrim` and `colors.accent_on` are the two tokens already used for "legible UI element on top of a potentially dark, colored surface." Using them here requires no new tokens or hard-coded colors.

**Alternatives considered:**
- Hard-coded `rgba(0,0,0,0.4)` — works but ignores the theme system; may not fit warm-toned themes as well as the theme-tuned `scrim`.
- `bg(surface)` with full opacity — always visible but opaque, creates a white/light disc that looks like a design error rather than a system control.
- Keep `hover` and add a border — a border helps slightly but still insufficient on matching-hued covers.

## Risks / Trade-offs

- [Risk] `scrim` on Parchment is only 26 % opacity, which may be too light on covers with very similar hue to the scrim color. → Acceptable — even at 26 %, a dark tint over any background makes a white ✕ legible; this is the existing behavior of, e.g., system media close buttons.
- No other risks — two-token swap, no behavior change, no new state.
