## Context

`render_thumb_row` derives its row height as `density.row_text_height + px(6.0)`:
- Comfortable: `44 + 6 = 50px`
- Compact: `33 + 6 = 39px`

The thumbnail height is `thumb_width × 10/7`:
- Comfortable: `46 × 10/7 ≈ 65.7px` — 15px taller than the 50px row
- Compact: `40 × 10/7 ≈ 57px` — 18px taller than the 39px row

Because the row's flex container has `items_center()` but no `overflow_hidden()`, the thumbnail overflows above and below the row boundary, visually intersecting the rows above and below it.

`render_generative_cover` renders publisher text (top), a motif + title (center), and product line (bottom) as children of the cover tile. At thumb dimensions (46×66px comfortable), this text is ~7–8px and unreadable. The metadata is already duplicated in the text column to the right.

`render_generative_cover` is called from both `render_thumb_row` (no text wanted) and `render_grid_card` (text at 158×226px+ is legible and useful).

## Goals / Non-Goals

**Goals:**
- Add `thumb_row_height: Pixels` to `DensityConstants` sized to contain the thumbnail
- Increase `thumb_width` values so the thumbnail is meaningfully larger
- Update `render_thumb_row` to use `thumb_row_height`
- Add `render_text: bool` to `render_generative_cover`; guard text children behind it
- Pass `false` from `render_thumb_row` and `true` from `render_grid_card`

**Non-Goals:**
- Changing the right-side metadata text column in `render_thumb_row` (title, publisher·line, kind+details still appear there)
- Changing the grid card cover layout beyond the parameter addition
- Adding actual downloaded cover images (deferred to a future change)

## Decisions

### `thumb_row_height` as a separate density constant

`row_text_height` drives the list view row height. Tying the thumbs row to `row_text_height + 6` means any change to list density also changes the thumbs view. A dedicated `thumb_row_height` gives independent control and makes the intent explicit.

Target values chosen so `thumb_h = thumb_width × 10/7` fits within `thumb_row_height` with a few pixels of breathing room:

| Density     | `thumb_width` | `thumb_h` (computed) | `thumb_row_height` | Padding each side |
|-------------|--------------|----------------------|--------------------|-------------------|
| Comfortable | 60px         | ≈85.7px              | 90px               | ~2px              |
| Compact     | 50px         | ≈71.4px              | 76px               | ~2px              |

### `render_text: bool` parameter on `render_generative_cover`

Adding a boolean parameter is the minimal change that lets the two call sites (thumbs vs. grid) opt in or out of cover text without duplicating the function. The parameter is positional and the function is private to the crate, so there is no API-compat concern.

When `render_text` is `false`, the three text children (publisher `div`, center motif+title block, line `div`) are replaced with a single centered motif so the background + shape still fills the tile attractively.

Specifically, when `render_text = false`:
- Remove the top publisher div and bottom line div
- Keep only the motif in a centered container, so the cover is: solid background + centered symbol

When `render_text = true` (grid): behavior unchanged.

## Risks / Trade-offs

- **`compact` thumbnail at 50px wide**: still small but materially larger than 40px; if the user wants even larger covers this can be adjusted without further structural changes.
- **Cover text loss in thumbs**: metadata is shown in the right-hand text column, so no information is lost; the cover becomes a colour-coded visual identifier only.
