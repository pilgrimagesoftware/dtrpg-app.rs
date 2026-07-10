## 1. Shared helpers

- [ ] 1.1 In `detail_panel_view.rs`, import `crate::data::constants::VALUE_FONT`
- [ ] 1.2 Add `.font_family(VALUE_FONT)` to `copyable_value`'s root `div()`
- [ ] 1.3 Add `.font_family(VALUE_FONT)` to `render_relative_date_value`'s root `div()`

## 2. New helper for plain-string values

- [ ] 2.1 Add `fn styled_value(value: impl Into<SharedString>) -> AnyElement`, mirroring `copyable_value`'s shape without the copy affordance: `div().font_family(VALUE_FONT).child(value.into()).into_any_element()`
- [ ] 2.2 Replace the item-tier `.value(file.format.to_string())` and `.value(format!("{:.1} MB", file.size_mb))` calls with `.value(styled_value(...))`
- [ ] 2.3 Replace the entry-tier `render_metadata_table` plain-string `.value(...)` calls (System/`value_or_dash(&item.line)`, Released/`item.year`, Format/`item.format`, File Size/`format!("{:.0} MB", item.size_mb)`, Category/`item.kind`, Pages/`item.pages`) with `.value(styled_value(...))`
- [ ] 2.4 Replace the "Other details" `.value(item.added_order.to_string())` call with `.value(styled_value(...))`

## 3. Verification

- [ ] 3.1 Run `cargo check --all-targets --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo +nightly fmt -- --check`, `cargo test --all-features --workspace`
- [ ] 3.2 Manually verify: open the expanded detail tab for a single-item entry; confirm every metadata value (System, Released, Format, File Size, Category, Pages, Added, Updated) renders in the sans-serif value font while labels stay in the serif body font
- [ ] 3.3 Manually verify: open a multi-item entry, select a file, expand the file-detail disclosure; confirm file id and download location values use the value font, and the copy-on-hover affordance still works
- [ ] 3.4 Manually verify: expand "Other details"; confirm stable id, numeric id, order product id, product id, added order, and cover color hex all use the value font
