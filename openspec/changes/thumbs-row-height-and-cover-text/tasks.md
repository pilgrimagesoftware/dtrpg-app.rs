## 1. DensityConstants — add thumb_row_height and increase thumb_width

- [x] 1.1 In `data/theme.rs`, add `pub thumb_row_height: Pixels` field to `DensityConstants`
- [x] 1.2 In `DensityConstants::comfortable()`, set `thumb_row_height: px(90.0)` and change `thumb_width: 60.0`
- [x] 1.3 In `DensityConstants::compact()`, set `thumb_row_height: px(76.0)` and change `thumb_width: 50.0`

## 2. render_generative_cover — add render_text parameter

- [x] 2.1 In `ui/library/cover.rs`, add `render_text: bool` as the third parameter to `render_generative_cover` (after `width: f32, height: f32`)
- [x] 2.2 When `render_text` is `true` (grid path): keep existing publisher/motif+title/line children unchanged
- [x] 2.3 When `render_text` is `false` (thumbs path): replace the three children with a single centered motif: `div().flex_1().flex().items_center().justify_center().child(render_motif(motif, fg45))`

## 3. render_thumb_row — use thumb_row_height and suppress cover text

- [x] 3.1 In `catalog_view.rs`, in `render_thumb_row`, replace `let row_h = density.row_text_height + px(6.0);` with `let row_h = density.thumb_row_height;`
- [x] 3.2 Update the `render_generative_cover` call to pass `render_text: false`: `render_generative_cover(item, thumb_w, thumb_h, false)`

## 4. render_grid_card — pass render_text: true

- [x] 4.1 In `catalog_view.rs`, in `render_grid_card`, update the `render_generative_cover` call to pass `render_text: true`: `render_generative_cover(item, card_w, cover_h, true)`

## 5. render_grid (grouped grid) — pass render_text: true

- [x] 5.1 In `catalog_view.rs`, in `render_grid`, update the `render_generative_cover` call (inside `render_grid_card`) — this is handled by task 4.1 since `render_grid` delegates to `render_grid_card`; confirm no other direct `render_generative_cover` call sites exist

## 6. Verification

- [x] 6.1 Run `cargo check --all-targets` and confirm no compile errors
- [x] 6.2 Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new warnings
- [x] 6.3 Run `cargo test --all-features --workspace` and confirm all tests pass
- [ ] 6.4 Launch the app in thumbs view; confirm rows are taller and the thumbnail no longer overflows into adjacent rows
- [ ] 6.5 Confirm each thumbnail shows only a coloured background and centred motif shape, with no publisher/title/line text inside the cover tile
- [ ] 6.6 Switch to grid view; confirm cover tiles still show publisher, title, and line text as before
- [ ] 6.7 Confirm the right-hand text column in thumbs rows (title, publisher·line, kind+details) is unaffected
