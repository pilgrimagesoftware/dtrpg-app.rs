## Context

`render_activity_button` in `sidebar_view.rs` (line ~221) applies `.text_xs()` to the containing div. The symbols `↻`, `●`, and `○` are Unicode characters that rely on font rendering at the right size to be clearly recognisable. At `text_xs` (≈11–12px) they are visually noisy and hard to distinguish quickly.

## Goals / Non-Goals

**Goals:**
- Symbols and count text render at `text_sm` (≈13–14px) for better legibility.
- No layout regressions — padding and click area stay the same.

**Non-Goals:**
- Replacing the Unicode symbols with an icon component (out of scope for this change).
- Adjusting padding, colour, or count badge styling.

## Decisions

**Change `.text_xs()` to `.text_sm()` only.** No other properties need adjustment — the button's fixed padding (`px(18)`, `pb(11)`) already provides enough space at the larger text size.

## Risks / Trade-offs

- Low risk. Single-property change with no layout impact.
