## Why

The catalog thumbs view has two problems. First, each row's height (`row_text_height + 6px` — 50px comfortable, 39px compact) is shorter than the thumbnail it contains (`thumb_width × 10/7` — ~66px comfortable, ~57px compact), causing the thumbnail to overflow its row and visually bleed into adjacent rows. Second, `render_generative_cover` renders three text layers inside the cover tile (publisher at top, title+motif in the center, product line at the bottom). At thumb-list dimensions this text is illegible and clutters the cover art; the metadata is already shown in the text column to the right of the thumbnail.

## What Changes

- Add a `thumb_row_height` constant to `DensityConstants` (comfortable: 90px, compact: 72px) and increase `thumb_width` (comfortable: 60px, compact: 50px) so the thumbnail fits within its row
- Update `render_thumb_row` to use `thumb_row_height` for row height
- Add a `render_text: bool` parameter to `render_generative_cover`; when `false`, omit the publisher/title/line text children, showing only the background colour and motif
- Pass `render_text: false` from `render_thumb_row` and `render_text: true` from `render_grid_card` (grid cards are large enough to show cover text)

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- `rust-main-window-library-layout`: Thumbs view row height is determined by a dedicated density constant (not derived from the list row height); the generative cover tile in thumbs context shows only background + motif without text

## Impact

- `data/theme.rs` (`DensityConstants`): add `thumb_row_height: Pixels`; increase `thumb_width` values
- `catalog_view.rs` (`render_thumb_row`): use `density.thumb_row_height` for row `h`; pass `render_text: false` to cover render
- `catalog_view.rs` (`render_grid_card`): pass `render_text: true` to cover render
- `ui/library/cover.rs` (`render_generative_cover`): add `render_text: bool` parameter; wrap publisher/title/line children in `when(render_text, ...)` guards
