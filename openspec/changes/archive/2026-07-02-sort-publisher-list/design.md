## Context

`publisher_entries` in `util/publisher.rs` currently sorts by count descending, then name
ascending for ties. The single sort comparator on line 27 is the only thing that needs to change.

`group_by_publisher` calls `publisher_entries` to establish group order and inherits the
result automatically — no changes needed there.

## Goals / Non-Goals

**Goals:**
- Change the sort key from count-descending to name-ascending (case-insensitive).

**Non-Goals:**
- Adding a user-configurable sort option.
- Changing the `PublisherEntry` data model.
- Affecting any other sorted list in the app.

## Decisions

### Decision: Case-insensitive sort using `str::to_lowercase`

Compare `a.name.to_lowercase()` against `b.name.to_lowercase()` rather than a raw
`a.name.cmp(&b.name)` to avoid ASCII sort order placing all uppercase names before lowercase
names.

**Alternatives considered:**
- `unicase::UniCase` — unnecessary external dependency for a simple sidebar sort.
- Storing a pre-lowercased key on `PublisherEntry` — premature optimization; the list is
  small and built once per catalog load.

## Risks / Trade-offs

- [Sort order change is visible immediately] → Expected; this is the intent of the change.
- [Publishers with identical names after lowercasing (impossible with the current data model)] → No mitigation needed.
