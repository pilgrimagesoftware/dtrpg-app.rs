## 1. Layout Restructure

- [ ] 1.1 Change `render_detail_tab_content`'s outer container from `.flex_col()` to
      `.flex_row()`
- [ ] 1.2 Cover box becomes a fixed-width left column (`.flex_none()`), no longer wrapped
      in a full-width centered row
- [ ] 1.3 Info content (publisher, title, status icon, line, description, actions,
      metadata) becomes the right column with `.flex_1()` and its own
      `.overflow_y_scrollbar()`
- [ ] 1.4 "Refresh thumbnail" overlay button positioning is preserved relative to the
      cover in its new column

## 2. Narrow-Width Fallback

- [ ] 2.1 Below a minimum tab content width, render the current stacked (cover-above-info)
      layout instead of the row split

## 3. Build and Quality

- [ ] 3.1 `cargo check --workspace`
- [ ] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 3.3 `cargo test --workspace`

## 4. Manual Verification

- [ ] 4.1 Open a detail tab at normal width and confirm thumbnail-left / info-right layout
- [ ] 4.2 Scroll a long description and confirm the cover stays fixed in place
- [ ] 4.3 Narrow the tab below the fallback threshold and confirm it reverts to stacked
      layout without clipping the cover
