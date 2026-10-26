# Proposal

## Why

The catalog's content views (List, Thumbs, Grid, and their grouped forms) change abruptly. Changing the filter, sort, or search swaps every visible row for different content in one frame, so the view flickers and items jump. Switching presentation replaces the whole tree. Covers pop in over a placeholder when they arrive. A short, calm transition on each of these reads as continuity instead of a glitch.

## What Changes

- Add short opacity transitions to the catalog content area when the set of visible items changes (filter, sort, search, grouping, data swap) and when the presentation switches.
- Fade a cover in over its placeholder when it arrives, only on arrival; a cover that is already cached shows at once.
- Fade content in when it replaces the loading skeleton or an empty state, instead of cutting.
- Keep scroll position stable across a change that keeps the same items on show.
- Read the operating system's reduce-motion setting at startup and when it changes, and make every transition in this change collapse to its end state when it is on.
- No transition runs for ordinary scrolling, hover, selection, or per-frame work. Item-level move/enter/exit animation (reorder, insert, remove) is out of scope; see design.md.
- Add ADR 0011 recording the approach (transition the region, not the rows).

## Capabilities

### New Capabilities

- `catalog-content-transitions`: which catalog content changes animate, how long, and what must not animate; scroll stability across a change.
- `reduced-motion`: the app follows the OS reduce-motion setting; animations end in their final state with no animation frames.

### Modified Capabilities

None. Existing catalog specs (`catalog-virtualized-rendering`, `catalog-loading-state`, `list-loading-skeleton`, `catalog-scroll`) keep their requirements; the new specs add to them.

## Impact

- `crates/dtrpg-ui`: `ui/views/catalog_view/` (`render.rs`, `grid.rs`, `thumb_row.rs`, `loading_skeleton.rs`, `empty_status.rs`), `ui/library/cover.rs` (`CoverCache`), the controller's thumbnail path, app startup (reduce-motion), and a new small `motion` module for durations and the transition helper.
- Dependencies: none new for the transitions (gpui's `with_animation`, `AnimationExt`, and `App::reduce_motion`). Reading the macOS setting needs `objc2-app-kit`, which the crate does not list yet; `objc2` and `objc2-foundation` are present.
- Performance: governed by `dtrpg/.claude/rules/ui-performance.md`. The frame-cost budget and how it is verified are in design.md.
- Overlap: `simplify-catalog-status-indicators` also edits catalog row and card rendering; land order is noted in design.md.
