## Context

`detail_panel_view.rs`'s `DescriptionList` value cells come from three sources today:

- **`copyable_value(field_id, value)`** (line 680): returns an `AnyElement` — a `div()` wrapping the value text plus a hover-revealed copy button. Used for: item-tier file name, file id, download location, and (inside "Other details") stable id, numeric id, order product id, product id, cover color hex.
- **`render_relative_date_value(item_id, slot, ts)`** (line 802): returns an `AnyElement` — a stateful `div()` showing relative time with an absolute-time tooltip. Used for: Added, Updated.
- **Plain strings passed directly to `.value(...)`**: `file.format`, `format!("{:.1} MB", file.size_mb)` (item-tier); `item.line` (via `value_or_dash`), `item.year`, `item.format`, `format!("{:.0} MB", item.size_mb)`, `item.kind`, `item.pages`, `item.added_order` (entry-tier / other-details).

`DescriptionList`'s value cell (`gpui-component`'s `description_list.rs`, confirmed by reading the component source during `settings-view-polish`'s design work) is a plain `div().flex_1()...child(value)` with no font styling — whatever font the passed-in `value` renders in is exactly what shows. `settings_advanced_view.rs`'s `stat_row` already establishes the pattern for this: wrap the value content in a `div().font_family(VALUE_FONT).child(...)`, applied to the value only, never the label.

## Goals / Non-Goals

**Goals:**

- Every `DescriptionList` value cell in the detail tab renders in `VALUE_FONT`, with zero behavior change (copy buttons, tooltips, disclosure toggles all keep working exactly as today).
- Minimize touch points by fixing the two shared helper functions once, rather than wrapping each of their ~8 call sites individually.

**Non-Goals:**

- No change to labels, section headers, prose, or anything outside `DescriptionList` value cells.
- No change to `VALUE_FONT` itself or how it's resolved — this change consumes the existing `data/constants.rs` constant exactly as `settings_advanced_view.rs` does today. Making it user-configurable is `settings-appearance-fonts`, a separate proposal.
- No change to `item_popover_view.rs`'s `DescriptionList` usage (single-click popover) — the request was scoped to "the detail view" (the expanded detail tab), not the popover. Can be a fast follow if wanted.

## Decisions

### Fix the two shared helpers in place; add one small helper for the rest

- `copyable_value`: add `.font_family(VALUE_FONT)` to its root `div()` (line 683). Covers file name, file id, download location, stable id, numeric id, order product id, product id, and cover color hex — 7 of the roughly 15 value cells — with a one-line change.
- `render_relative_date_value`: add `.font_family(VALUE_FONT)` to its root `div()` (line 810). Covers Added and Updated.
- New `fn styled_value(value: impl Into<SharedString>) -> AnyElement` (mirroring `copyable_value`'s signature shape but without the copy affordance): `div().font_family(VALUE_FONT).child(value.into()).into_any_element()`. Replaces the remaining plain-string `.value(...)` arguments: `file.format`, `format!("{:.1} MB", file.size_mb)` (item-tier), `value_or_dash(&item.line)`, `item.year.to_string()`, `item.format.to_string()`, `format!("{:.0} MB", item.size_mb)`, `item.kind.to_string()`, `item.pages.to_string()`, `item.added_order.to_string()` (entry-tier / other-details) — roughly 9 call sites, each a one-line wrap (`styled_value(...)` instead of the bare value expression).

_Alternative considered:_ set `.font_family(VALUE_FONT)` on the outer `DescriptionList` itself, relying on GPUI's ambient text-style cascade to reach the value cells. Rejected — the label cells are siblings rendered by the same component inside the same row, and the component only overrides `.text_color()` for labels, not `.font_family()`; a single outer wrapper would cascade the sans-serif font onto labels too, which should stay in the default serif per the existing `stat_row` convention (label ≠ value styling).

### Interaction with `settings-appearance-fonts`

That proposal (separate, not yet implemented) replaces the `VALUE_FONT` constant with a live `cx.global::<LibriTheme>().value_font.family` lookup and lists `settings_advanced_view.rs` as a call site to migrate. Once both changes exist, its task list should be extended to also cover `copyable_value`/`render_relative_date_value`/`styled_value` in `detail_panel_view.rs` — noted here so it isn't missed, not solved by this change (this change only needs `VALUE_FONT` to exist as *some* resolvable font name, constant or not).

## Risks / Trade-offs

- **[Risk]** `styled_value` is a very thin wrapper (single field, no copy affordance) sitting alongside `copyable_value` (same shape, plus copy) — mild duplication. → Acceptable: they serve genuinely different call sites (copyable vs. not), and merging them into one function with a `copyable: bool` flag reads worse than two small, clearly-named functions.
