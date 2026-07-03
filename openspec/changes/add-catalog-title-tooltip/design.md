## Context

`catalog_view.rs` renders item titles in four layouts: grid card (`render_grid_card`), flat list row (`render_td`,
the `DataTable` delegate's title column), grouped list row (`render_grouped_list_row`), and thumbnail row
(`render_thumb_row`). Each title element already has `.truncate()` applied via gpui's `TextOverflow::Truncate`
styling, which clips overflowing text with an ellipsis at render time. Available column/card widths are already
known at the call sites (`cols[0].width` for list layouts, a fixed card/thumb width for the grid and thumbnail
layouts), so truncation can be determined without a new layout pass over the whole row.

There is no existing "tooltip only when truncated" helper in `gpui` or `gpui-component`. Truncation must be
detected explicitly.

## Goals / Non-Goals

**Goals:**
- Show a tooltip with the full title only when the rendered title text does not fit in its available width.
- Reuse the existing `.tooltip()` builder pattern already used throughout the codebase (see `sidebar_view.rs`,
  `status_bar_view.rs`, `title_bar_view.rs`).
- Cover all four title render paths: grid card (`render_grid_card`), flat list row (`render_td`), grouped list row
  (`render_grouped_list_row`), thumbnail row (`render_thumb_row`).

**Non-Goals:**
- No tooltip for any other truncated text in the catalog view (publisher, line/system columns) — title only, per
  the proposal.
- No debounce/delay tuning beyond gpui's default tooltip hover delay.
- No layout changes to column widths or card sizing.

## Decisions

- **Truncation detection: measure text width vs. available width using `window.text_system().shape_line(...)`.**
  At the point each title element is constructed we already have `window` and `cx` in scope (render closures run
  inside a `Window`/`App` context). Shape the title string with the same font/size used by the `text_sm()` style,
  compare `shaped_line.width` against the known available width (column width minus the fixed-width sibling
  elements' allotment, or the card's text slot width), and only call `.tooltip(...)` when
  `shaped_line.width > available_width`.
  - Alternative considered: always attach the tooltip and rely on `.truncate()` alone with no conditional check.
    Rejected — this would show a redundant tooltip on titles that already fit, which the proposal explicitly rules
    out.
  - Alternative considered: use `cx.text_bounds()` / post-layout measurement (query the actual laid-out element
    size after paint and compare to content size). Rejected for this change — `shape_line` measurement at
    construction time is simpler, does not require a second layout pass or deferred rendering, and matches the
    width values already threaded through the row/card builder functions.
- **Tooltip content is the untruncated `item.title` string**, passed as a plain string closure to `.tooltip()`,
  consistent with existing plain-string tooltip usages (e.g. `title_bar_view.rs`).
- **Width calculation is local to each render function.** Each of the three render paths already computes or has
  access to the pixel width available to the title text (grid card text slot, list row title column). No new
  shared width-calculation utility is introduced; the `shape_line` + compare logic is applied inline at each of
  the three call sites, kept small enough to remain within the function-length guidance.

## Risks / Trade-offs

- [Risk] Font/size mismatch between the `shape_line` call and the actual rendered style could cause the measured
  width to disagree with the real rendered width, causing false positives/negatives.
  → Mitigation: use the same `text_sm()` font size and the window's current `text_style()` font when shaping, and
    verify visually against known long/short titles during implementation.
- [Risk] Re-shaping the title text on every render (once per visible row/card) adds measurement cost.
  → Mitigation: `shape_line` is a cheap, cached-by-hash operation in gpui's text system
    (`shape_line_by_hash`), and only runs for visible rows given the catalog view's existing virtualization; no
    additional caching is introduced in this change.
- [Trade-off] Determining truncation ahead of paint means it can drift slightly if window resize happens
  mid-frame; this can leave a one-frame stale truncation determination, which is acceptable for a hover-triggered
  tooltip and self-corrects on the next render.

## Migration Plan

Not applicable — additive UI behavior, no data migration or rollback concerns beyond a normal code revert.
