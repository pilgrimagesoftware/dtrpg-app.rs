## 0. Prerequisite

- [x] 0.1 Confirm `settings-appearance-fonts` has landed (i.e. `LibriTheme.fonts.label_font: SharedString` exists and is resolvable via `cx.global::<LibriTheme>()`) before starting section 1 — confirmed on branch `feature/settings-appearance-fonts`

## 1. Label helper

- [x] 1.1 In `detail_panel_view.rs`, add `fn styled_label(label: impl Into<SharedString>, label_font_family: &str) -> AnyElement`: `div().font_family(label_font_family).child(label.into()).into_any_element()`
- [x] 1.2 Add `.font_family(label_font_family)` directly to `render_metadata_table`'s existing `category_label` (already a `div()...into_any_element()`)

## 2. Wire the live font family through

- [x] 2.1 Give `render_metadata_table` a new `label_font_family: &str` parameter; update its caller (`render_detail_tab_content`) to resolve `cx.global::<LibriTheme>().fonts.label_font` and pass it down
- [x] 2.2 `render_item_metadata`, `render_file_other_details`, and `render_other_details` already take `cx: &App` — resolve `cx.global::<LibriTheme>().fonts.label_font` locally in each rather than adding a parameter

## 3. Replace label call sites

- [x] 3.1 In `render_metadata_table`, replace the `t!(...).to_string()` label arguments for System, Released, Format, File Size, Pages, Added, Updated with `styled_label(..., label_font_family)`
- [x] 3.2 In `render_item_metadata`, replace the Name, Format, File Size label arguments with `styled_label(..., label_font_family)`
- [x] 3.3 In `render_file_other_details`, replace the File ID and Download Location label arguments with `styled_label(..., label_font_family)`
- [x] 3.4 In `render_other_details`, replace the Stable ID, Numeric ID, Order Product ID, Product ID, Added Order, and Cover Color label arguments with `styled_label(..., label_font_family)`

## 4. Verification

- [x] 4.1 Run `cargo check --all-targets --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo +nightly fmt -- --check`, `cargo test --all-features --workspace`
- [ ] 4.2 Manually verify: open the expanded detail tab for a single-item entry; confirm every metadata label (System, Released, Format, File Size, Category, Pages, Added, Updated) renders in the sans-serif label font while values stay in the serif body font
- [ ] 4.3 Manually verify: open a multi-item entry, select a file, expand the file-detail disclosure; confirm the file id and download location labels use the label font, and the copy-on-hover affordance on the values still works
- [ ] 4.4 Manually verify: expand "Other details"; confirm the stable id, numeric id, order product id, product id, added order, and cover color labels all use the label font, and their values remain in the body font
- [ ] 4.5 Manually verify: change the label font in Settings > Appearance; confirm detail-tab labels update live to match
