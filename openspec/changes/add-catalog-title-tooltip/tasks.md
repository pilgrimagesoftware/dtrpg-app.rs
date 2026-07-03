## 1. Truncation detection helper

- [ ] 1.1 Add a small helper in `catalog_view.rs` that shapes a title string via
      `window.text_system().shape_line(...)` at the `text_sm()` font/size and returns whether the shaped width
      exceeds a given available width in pixels.
- [ ] 1.2 Confirm the helper's font/size matches the actual rendered `text_sm()` style so measured truncation
      agrees with visual truncation.

## 2. Grid card title tooltip

- [ ] 2.1 Locate the grid card title render path and determine the pixel width available to the title text slot.
- [ ] 2.2 Apply the truncation-detection helper and conditionally attach `.tooltip(...)` with the full
      `item.title` when truncated.

## 3. Flat list row title tooltip

- [ ] 3.1 Locate the flat list row title render path and determine the pixel width available to the title column.
- [ ] 3.2 Apply the truncation-detection helper and conditionally attach `.tooltip(...)` with the full
      `item.title` when truncated.

## 4. Grouped list row title tooltip

- [ ] 4.1 In `render_grouped_list_row`, determine the pixel width available to the title text (the `flex_1`
      title/badge slot minus the badge width).
- [ ] 4.2 Apply the truncation-detection helper and conditionally attach `.tooltip(...)` with the full
      `item.title` when truncated.

## 5. Verification

- [ ] 5.1 Manually verify: long titles show a tooltip with the full title on hover in all three layouts (grid,
      flat list, grouped list).
- [ ] 5.2 Manually verify: short titles that fit fully show no tooltip in all three layouts.
- [ ] 5.3 Run `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt --all -- --check`.
