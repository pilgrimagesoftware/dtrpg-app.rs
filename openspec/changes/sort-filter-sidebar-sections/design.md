# Design

## Context

See proposal.md for motivation. Where the order comes from today:

- **Sections.** `FACET_SECTIONS` in `ui/views/filters_column.rs` is a constant array: Availability, System, Type, Location, Format, Genre, Tag. It is rendered in that order; there is no stated rule behind it.
- **Values.** `facet_counts` in `data/index/filtered_items/facets.rs` runs one SQL query per facet and orders it `COUNT(...) DESC`, then by lowercase name (Availability: `COUNT(*) DESC, status`). Counts depend on the current search, minus the field's own choices, so they change whenever any filter changes; the order follows them.
- **Where it is stored.** `Library::invalidate_cache` runs `count_facets` when the view changes and stores `Arc<Facets>`; the controller exposes it as `visible_facets()`.
- **What reads it.** After PR 528, `FiltersPaneView` copies the facets (minus hidden values) when `LibraryChanged` fires and redraws only if they differ from what it holds. Render reads the stored copy.

Governing rule: `dtrpg/.claude/rules/ui-performance.md`.

## Goals / Non-Goals

**Goals:**
- A stated, fixed order for sections and values.
- Stability while the user clicks and across loads.
- No sorting in render.

**Non-Goals:**
- User-configurable order, pinning, or "sort by count" toggles.
- The left sidebar's sections (Publishers is already alphabetical under `alphabetical-publisher-list`).
- Moving `invalidate_cache` off the UI thread. It already computes the facets there; this change adds no work to it.

## Decisions

### 1. Sort in the facet query, once, when the facets are built

Change each `ORDER BY` in `facet_counts` to `ulower(MIN(name))` then `MIN(name)` for the tie-break (Availability: an explicit `CASE status WHEN 'downloaded' THEN 0 ELSE 1 END`). The sort happens inside the query that already runs when the visible set changes and is stored with the facets. `FiltersPaneView` and render read it unchanged. Sorting in the UI would add a per-refresh `sort` on a copy; sorting in SQL costs nothing extra because the query already orders.

`ulower` is the existing registered SQL function (Unicode lowercase, no accent folding) that these queries already use. Values that start with an accented letter therefore sort after "z"; folding accents would need a new function or collation, and is left out until a catalog shows the need.

### 2. Alphabetical, not by count

By count was the reshuffle. Alphabetical is the one rule that is predictable for text values, independent of counts, and identical after a restart. The count stays on screen beside each value, so the information is not lost.

Alternatives rejected:
- *By count, but frozen while a filter is being chosen.* Needs interaction state, a freeze timer, and a rule for when to unfreeze; the order would still differ between sessions.
- *Checked values pinned to the top.* Moves the value the user just clicked; it conflicts with "does not move while interacting".
- *Frequency of use.* Needs usage tracking and persistence for little gain.

### 3. Section order

Availability, Type, System, Genre, Format, Location, Tags: what a user usually narrows by first (is it here, what kind of product) to last (where it is, my own labels). The order is a constant in `filters_column.rs` with a comment giving the rule, so a future change edits one place.

### 4. No user reordering now

YAGNI: there is no evidence the fixed order is wrong once it is stable. If reordering is wanted later, the section order belongs in `FiltersPaneView` state persisted with the other UI preferences, and the constant becomes the default.

## Risks / Trade-offs

- [Long lists (many tags) alphabetically bury the common values] → Counts are shown beside each value, and the Tags section can gain a collapsed "show more" later; not in this change.
- [Accented first letters sort after "z"] → Accepted for now; a test pins the behavior so a later fold is a deliberate change.
- [Order change surprises people used to count order] → Intended; the PR notes it.
- [Overlap with PR 528] → That PR makes the pane hold and compare the facets. With a stable order the comparison fires only on real changes. No code conflict is expected beyond the section constant.
