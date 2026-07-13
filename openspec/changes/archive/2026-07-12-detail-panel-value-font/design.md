## Context

`detail_panel_view.rs`'s `DescriptionList` items come from `DescriptionItem::new(label).value(value)`. Reading `gpui-component`'s `description_list.rs` (pinned rev `be4c5d3`) directly: both `label` and `value` accept `impl Into<DescriptionText>`, an enum with `From` impls for `&str`/`String`/`SharedString`/`Text`/`AnyElement`. The component's own rendering only sets `.text_color(cx.theme().description_list_label_foreground)` on the label cell's wrapping `div()` — no `.font_family()` override on either cell, so whatever is passed in renders in whatever font that content carries.

Every label call site today passes a plain `t!("detail.field_x").to_string()` — a bare `String` with no font styling, so it renders in the ambient body font. One label (`render_metadata_table`'s `category_label`) is already built as a small `div()` (icon + text) and converted with `.into_any_element()`.

`settings-appearance-fonts` has since landed (branch `feature/settings-appearance-fonts`) and, in the course of implementation, grew a fourth font role — `cx.global::<LibriTheme>().fonts.label_font: SharedString`, defaulting to `Gotham` — specifically anticipating this change's need to style labels independently from values (see that change's design.md: "Independent of `VALUE_FONT_OPTIONS` so labels and values can be styled with different sans-serif fonts"). This change reads that field directly rather than `value_font`, which now genuinely means "value," and rather than a hardcoded constant.

## Goals / Non-Goals

**Goals:**

- Every `DescriptionList` label cell in the detail tab renders in the live value-font role, with zero behavior change to values, copy buttons, tooltips, or disclosure toggles.
- Minimize touch points: one small helper wrapping each label argument, rather than restyling every value cell.

**Non-Goals:**

- No change to values — they keep rendering in the default body font, exactly as today.
- No change to section headers ("Other details") or prose — only `DescriptionList` label cells are affected.
- No change to `item_popover_view.rs`'s `DescriptionList` usage (single-click popover) — scoped to the expanded detail tab, matching the original scoping decision. Can be a fast follow if wanted.
- No new font constant or configuration surface — this change is a pure consumer of `LibriTheme.fonts.label_font`, introduced by `settings-appearance-fonts`.

## Decisions

### Add one small label helper; touch `category_label` directly

- New `fn styled_label(label: impl Into<SharedString>, label_font_family: &str) -> AnyElement`: `div().font_family(label_font_family).child(label.into()).into_any_element()`. Replaces every `t!(...).to_string()` argument passed to `DescriptionItem::new(...)` — 18 call sites across `render_metadata_table` (7: System, Released, Format, File Size, Pages, Added, Updated), `render_item_metadata` (3: Name, Format, File Size), `render_file_other_details` (2: File ID, Download Location), and `render_other_details` (6: Stable ID, Numeric ID, Order Product ID, Product ID, Added Order, Cover Color).
- `render_metadata_table`'s `category_label` already builds its own `div()` (icon + text) before `.into_any_element()`; it gains `.font_family(label_font_family)` directly rather than being routed through `styled_label`, since it isn't a bare string.
- `copyable_value` and `render_relative_date_value` (the value-side helpers) are untouched — values keep the default body font.

### Threading the live font family

`render_item_metadata`, `render_file_other_details`, and `render_other_details` already take `cx: &App` and can read `cx.global::<LibriTheme>().fonts.label_font` directly (it's a plain `SharedString`, coercible to `&str`). `render_metadata_table` currently takes only `(item, storage_root_path)` with no `cx`; rather than threading a `cx: &App` parameter into a function that has no other use for GPUI context, it gains a `label_font_family: &str` parameter, resolved by its caller (`render_detail_tab_content`, which already has `cx`) and passed down.

### Interaction with `settings-appearance-fonts`

That change already anticipated this one: it added a dedicated `label_font` role (rather than this change reusing `value_font`) precisely so `detail_panel_view.rs`'s labels could be styled independently from any view using `value_font`. This change only needs `LibriTheme.fonts.label_font` to exist and be resolvable — it doesn't modify `settings-appearance-fonts` itself.

## Risks / Trade-offs

- **[Risk]** Flipping which column (label vs. value) gets the sans-serif treatment departs from `settings_advanced_view.rs`'s existing "Cache details" convention (value gets the sans font there). → Accepted: this is a deliberate, scoped decision for the detail tab specifically, not a renaming of the shared convention. The two views can carry different label/value emphasis; nothing forces them to match.
- **[Risk]** Sequencing after `settings-appearance-fonts` means this change can't land independently until that one merges. → Accepted: avoids introducing a throwaway local constant that immediately needs migrating, per the user's preference to keep the two changes separate but ordered.
