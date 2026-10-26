# Proposal

## Why

The filters column lists facet values (Availability, System, Type, Location, Format, Genre, Tags) with a count each. Values are ordered by count, highest first, and the counts are recomputed on every change to the search. Checking a box changes the counts of the other facets, so values trade places under the user's pointer as they click. Section order is a fixed list that follows no stated rule. The column looks random and moves while in use.

## What Changes

- Order the sections in the filters column by a stated rule: Availability, Type, System, Genre, Format, Location, Tags (what the user narrows by first to what they narrow by last).
- Order the values in each section alphabetically (case-insensitive), with the count shown beside each value. Availability keeps its natural order: Downloaded, then Cloud.
- A value keeps its place when counts change or when it is checked; values are not pinned or moved to the top.
- The order is the same on every launch and after every load.
- The order is not user-configurable in this change.
- The sorted lists are built when the facets are computed and stored; rendering reads them as they are.

## Capabilities

### New Capabilities

- `filter-column-order`: the order of sections and of the values inside each section in the filters column, and its stability while the user interacts.

### Modified Capabilities

None. No existing spec covers the filters column's facets.

## Impact

- `crates/dtrpg-core`: `data/index/filtered_items/facets.rs` (the `ORDER BY` of each facet query) and its tests.
- `crates/dtrpg-ui`: `ui/views/filters_column.rs` (the section list order and its test).
- Builds on PR 528 (`FiltersPaneView`), which holds the facets and refreshes them only when they change; with a stable order, that comparison stops firing on count reshuffles alone. If 528 is not merged when this lands, rebase onto it.
- Performance: governed by `dtrpg/.claude/rules/ui-performance.md`. No new work in render; see design.md.
