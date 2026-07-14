## 1. Layout Restructure

- [x] 1.1 Change `render_detail_tab_content`'s outer container from `.flex_col()` to
      `.flex_row()`. Already row-direction by default (gpui's `Style::default()` sets
      `flex_direction: Row`, and the container had no explicit `.flex_col()`); made
      `.flex_row()` explicit rather than relying on the implicit default.
- [x] 1.2 Cover box becomes a fixed-width left column (`.flex_none()`), no longer wrapped
      in a full-width centered row. Already satisfied - added `.pl(px(20.0))` so it
      doesn't hug the left edge, matching the info column's `.p(px(20.0))` inset.
- [x] 1.3 Info content (publisher, title, status icon, line, description, actions,
      metadata) becomes the right column with `.flex_1()` and its own
      `.overflow_y_scrollbar()`. Already satisfied.
- [x] 1.4 "Refresh thumbnail" overlay button positioning is preserved relative to the
      cover in its new column. Already satisfied (absolutely positioned within the
      relative cover box, unaffected by the outer row layout).

## 2. Narrow-Width Fallback

- [ ] 2.1 Below a minimum tab content width, render the current stacked (cover-above-info)
      layout instead of the row split. Dropped per user decision 2026-07-14 - not
      implementing a width fallback for this change.

## 3. Build and Quality

- [x] 3.1 `cargo check --workspace`
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [x] 4.1 Open a detail tab at normal width and confirm thumbnail-left / info-right layout.
      Verified manually 2026-07-14 (cover was left of text, but hugging the left edge -
      fixed via 1.2's padding change).
- [x] 4.2 Scroll a long description and confirm the cover stays fixed in place. Verified
      manually 2026-07-14.
- [ ] 4.3 Narrow the tab below the fallback threshold and confirm it reverts to stacked
      layout without clipping the cover. N/A - fallback dropped, see 2.1.
