## Context

`render_authenticated` in `settings_account_view.rs` builds the Email row and the (conditional) API Key row independently, each as its own `div().flex().items_baseline().gap(px(6.0))` with a label `div` followed by a value `div`. Two problems follow from that structure:

1. Each row's label div sizes to its own text ("Email" vs "API Key"), so there's no shared column width to right-align against.
2. `items_baseline()` aligns flex children by their computed text baseline. The label uses the default UI font; the value uses a monospace font (`MONOSPACE_FONT`, set in the `settings-api-key-monospace` change). Different font families at the same `text_xs` size commonly have different ascent/descent ratios, so their baselines don't coincide even though `items_baseline()` is applied correctly — this is a font-metrics mismatch, not a layout bug in the flex call itself.

The codebase already has a vetted component for label/value rows: `gpui-component`'s `DescriptionList`/`DescriptionItem` (`crates/ui/src/description_list.rs`), already used in `detail_panel_view.rs` for the item metadata table. Its horizontal layout renders each row via `h_flex()`, and `gpui-component`'s `h_flex()` (`StyledExt::h_flex`) resolves to `.flex().flex_row().items_center()` — not baseline alignment — sidestepping the font-metrics mismatch entirely by centering both texts within the row's line height instead of aligning by baseline.

## Goals / Non-Goals

**Goals:**
- Email and API Key labels right-align against a shared column width so both rows' value text starts at the same horizontal position.
- Label and value text visually align within each row (same vertical center), regardless of the different font families in use.
- Keep the monospace treatment of the value text and the existing warning-color/other row styling in the section untouched.

**Non-Goals:**
- No change to the Reset API Key button's position or the existing `account-section-layout` requirement governing it.
- No change to the unauthenticated sign-in form layout.
- No new i18n keys or copy changes — only layout.

## Decisions

- **Reuse `gpui-component`'s `DescriptionList`/`DescriptionItem` (horizontal layout) for the Email/API Key rows** instead of hand-rolled `div().flex()` rows, consistent with this repo's `gpui-component`-first UI policy and the existing precedent in `detail_panel_view.rs`.
  - Configure `DescriptionList::horizontal().bordered(false).label_width(px(N))` with a fixed label width sized to fit "API Key" (the longer of the two labels) plus a small margin.
  - Pass each label as an `AnyElement` (`div().w_full().text_right().child(t!(...))`) rather than a bare string, since `DescriptionItem::new` accepts `impl Into<DescriptionText>` including `AnyElement` — this achieves right-alignment within the fixed label column without modifying the shared component.
  - Alternative considered: keep the hand-rolled rows but manually compute a shared label width and add `.text_right()` to each label div directly. Rejected in favor of `DescriptionList` because it already encodes the label-column/value-column layout this problem needs, and matches the `gpui-component`-first policy in `AGENTS.md` rather than re-deriving the same layout by hand.
- **Rely on `DescriptionList`'s internal `h_flex()` row (which centers, not baseline-aligns)** rather than trying to force matching baselines across different font families. Centering is the pragmatic fix `gpui-component` itself already uses for this exact label/value pattern elsewhere in the app.
- **Keep the monospace font override on the value text only**, applied via the same `AnyElement`-wrapped value passed into `DescriptionItem::value(...)`.

## Risks / Trade-offs

- [Risk] `DescriptionList` renders each row inside its own `h_flex()`, which adds default padding (`padding_x`/`padding_y`) even with `bordered(false)` per the component's `render()` implementation — visual spacing could shift slightly from the current hand-rolled `gap(px(6.0))` rows. → Mitigation: verify visually after the change and adjust `size`/`label_width` if the row padding looks different from the current spacing; this is a cosmetic tuning step, not a structural risk.
- [Risk] Wrapping labels in `AnyElement` with `.text_right()` bypasses `DescriptionList`'s own `DescriptionText::String` styling (text color, `text_sm()`) applied to plain string labels. → Mitigation: replicate the same text color/size (`colors.text_secondary`, `text_xs()`) explicitly on the wrapped label element so it matches the rest of the section's styling instead of picking up the component's default label styling.

## Migration Plan

- Single PR within `dtrpg-app/rust`: swap the Email/API Key row construction in `settings_account_view.rs` to use `DescriptionList`, verify visually (open Settings while signed in with and without an API key hint present).
- No data/state changes; pure presentation.
- Rollback is a straight revert if the row spacing/styling doesn't match expectations.
