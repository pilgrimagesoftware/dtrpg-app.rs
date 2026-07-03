## 1. Truncation detection helper

- [x] 1.1 Add a small helper in `catalog_view.rs` that shapes a title string via
      `window.text_system().shape_line(...)` at a given font size/weight and returns whether the shaped width
      exceeds a given available width in pixels.
- [x] 1.2 Add `text_sm_size`/`text_xs_size` helpers that resolve `.text_sm()`/`.text_xs()` font sizes against the
      window's rem size, so measured truncation agrees with visual truncation.

## 2. Grid card title tooltip

- [x] 2.1 In `render_grid_card`, thread `window: &Window` through and determine the pixel width available to the
      title text slot (`card_w` minus horizontal padding).
- [x] 2.2 Apply the truncation-detection helper and conditionally attach `.tooltip(...)` with the full
      `item.title` when truncated.

## 3. List row title tooltip

- [x] 3.1 In `render_list_item_cell` (col_ix 0, shared by the ungrouped and grouped list `TableDelegate`
      implementations), thread `window: &Window` through and determine the pixel width available to the title
      text (column width minus the kind-badge slot and gap).
- [x] 3.2 Apply the truncation-detection helper and conditionally attach `.tooltip(...)` with the full
      `item.title` when truncated. Update both `render_td` call sites to pass `window` instead of `_window`.

## 4. Thumbnail row title tooltip

- [x] 4.1 In `render_thumb_row`, thread `window: &Window` through and determine the pixel width available to the
      title text (viewport width minus catalog side padding, cover width, and gap — an approximation since the
      row's actual layout width isn't known synchronously; documented as a risk in design.md).
- [x] 4.2 Apply the truncation-detection helper and conditionally attach `.tooltip(...)` with the full
      `item.title` when truncated.

## 5. Verification

- [ ] 5.1 Manually verify: long titles show a tooltip with the full title on hover in all three layouts (grid,
      list, thumbnail). Not yet run — requires a windowed manual check, not available in this environment.
- [ ] 5.2 Manually verify: short titles that fit fully show no tooltip in all three layouts. Not yet run — same
      constraint as 5.1.
- [x] 5.3 Run `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt --all -- --check`.
