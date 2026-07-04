## Context

`render_detail_tab_content` and its helpers (`render_metadata_table`, `render_item_tier`)
in `detail_panel_view.rs` are free functions, not entity methods — they take `&LibraryItem`,
`&ColorTokens`, and (for the item tier) an `Entity<LibraryController>` used only to read/
write the selected-item-file map. There is no per-view local state today; anything that
must survive a re-render and reflect user interaction is owned by `LibraryController` and
keyed by entry id (see `selected_item_file: HashMap<Arc<str>, Arc<str>>`).

`gpui-component` provides `Collapsible` (`gpui_component::collapsible::Collapsible`), which
takes an `open: bool` and toggles via a caller-supplied `on_click` — the open/closed value
itself must live somewhere the caller controls and can mutate through `cx`.

## Goals / Non-Goals

**Goals:**
- Show `LibraryItem` fields not already covered by `render_metadata_table` or the item
  tier, behind a collapsed-by-default section.
- Keep the section's open/closed state consistent with how the rest of this view already
  manages per-entry interaction state.
- Follow the gpui-component-first UI policy: use `Collapsible`, not a hand-rolled
  expand/collapse `div`.

**Non-Goals:**
- No new fields on `LibraryItem` — every value the section shows already exists on the
  struct.
- No persistence of the open/closed state across app restarts or across switching to a
  different entry.
- No change to the item tier's existing selection state or the single-item metadata
  table's existing rows.

## Decisions

- **State ownership**: add `advanced_details_open: HashMap<Arc<str>, bool>` to
  `LibraryController`, mirroring `selected_item_file`, with `is_advanced_details_open`/
  `toggle_advanced_details` accessor methods. A local `bool` field on the render function
  isn't viable — `detail_panel_view.rs` has no entity of its own, and GPUI re-renders from
  scratch on each frame, so the toggle must live on the `LibraryController` entity already
  threaded through this view to survive re-render and drive `cx.notify()`.
  Missing entries default to closed (collapsed), matching "collapsed by default" and
  "resets when not explicitly reopened" from the proposal.
- **Placement**: render the disclosure as the last child in `render_detail_tab_content`'s
  scrollable content column, after `render_metadata_table`/`render_item_tier`, so it never
  competes for above-the-fold space with the primary metadata.
- **Component**: use `Collapsible` with the header row (`"Advanced details"` label +
  chevron) as its default child and the field list as its `.content(...)`, per the
  Collapsible usage pattern already documented in `gpui-component`'s docs.
- **Field list rendering**: reuse `DescriptionList`/`DescriptionItem` (already used by
  `render_metadata_table` and `render_item_metadata`) for visual consistency rather than
  introducing a second layout primitive for the same kind of label/value data.
- **Color field**: render `item.color` as a small swatch (a fixed-size `div` with
  `.bg(color)`) next to its hex string value, since a bare hex string doesn't let a user
  visually confirm which generative cover color it is.

## Risks / Trade-offs

- [Adding a second per-entry `HashMap` to `LibraryController` grows its state surface] →
  mirrors the existing `selected_item_file` pattern exactly, so it's a well-understood
  shape rather than a new one; both maps are small (one entry per open detail tab) and
  cleared the same way (`clear_item_selection`-equivalent behavior can be extended if the
  team wants a shared "reset per-entry detail state" helper later, out of scope here).
- [Parsing `item.color`'s hex string for the swatch could panic on malformed input] →
  the existing generative cover renderer already parses this same field for the cover
  background; reuse that parsing path (or its fallback) rather than adding a second,
  potentially-panicking parser.

## Open Questions

- Should the advanced details section also be collapsed into `render_item_metadata` for
  multi-item entries (per-file), or does the current placement — one disclosure per
  entry, listed once beside the item tier — cover the SDK-level "nerdy details" the
  proposal is targeting? Current design assumes entry-level only, since none of the
  candidate fields (`id`, `numeric_id`, `order_product_id`, `product_id`, `added_order`,
  `color`) are per-file.
