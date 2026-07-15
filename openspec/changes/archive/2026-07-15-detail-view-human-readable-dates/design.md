## Context

`LibraryItem` carries `added_order: u32` (a relative rank used for sort ordering) but no real timestamp. The detail panel's `render_metadata_table` builds a static vec of `(&'static str, String)` rows and renders them without interactivity. There is no "Added" row; "Released" renders `item.year.to_string()` — a bare four-digit year.

GPUI supports `.tooltip(|window, cx| AnyView)` on any `Stateful<Div>` (i.e. a div with `.id()`). The `gpui-component` `Tooltip::new(text).build(window, cx)` returns the `AnyView` required.

## Goals / Non-Goals

**Goals:**
- `LibraryItem` gains a `date_added: Option<i64>` field (Unix seconds).
- A new `util/datetime.rs` provides `format_relative` and `format_absolute` with no new crate dependencies.
- The detail panel shows an "Added" row with relative text; hovering shows the absolute date/time in a tooltip.
- Stub data is updated with synthetic timestamps spread across a realistic range.

**Non-Goals:**
- Persisting or loading real dates from the SDK/API (stub-only in this change).
- Adding a full release date beyond the existing `year: u32`.
- Tooltips on other views (catalog list, thumbs) — detail panel only.

## Decisions

### `date_added: Option<i64>` — Unix seconds, no new crate

**Decision**: Store the timestamp as a plain `i64` (Unix seconds). Format it using `std::time::SystemTime::now()` for "now", computing the elapsed duration entirely with `std::time`.

**Rationale**: Adding `chrono` or `time` for a single relative-date formatter in a UI-only crate is not worth the compile-time cost. `std::time` gives everything needed: `SystemTime::now()`, `UNIX_EPOCH`, `Duration` arithmetic, and conversion to `u64` seconds.

**Alternative considered**: `time` crate with `OffsetDateTime`. Provides nicer month/day formatting but requires a new workspace dependency. Deferred — the SDK crate will likely introduce it when real date parsing is needed.

### Month name formatting without `chrono`

**Decision**: Implement a small lookup `fn month_abbr(m: u32) -> &'static str` inside `datetime.rs` covering 1–12. Compute calendar date from epoch seconds using the Gregorian proleptic calendar algorithm (days since epoch → year/month/day via standard integer arithmetic).

**Alternative considered**: Formatting the timestamp as ISO-8601 and calling it done. Rejected — the spec requires "Jan 5" style output.

### Stub timestamp generation

**Decision**: In `stubs.rs`, compute each item's `date_added` at construction time as:

```
Some((SystemTime::now() as epoch) - (added_order as i64 * 43200))
```

43200 seconds = 12 hours per rank unit. With `added_order` values spanning ~35–412, this yields a realistic range of about 17 days to 6 months in the past.

### Tooltip element ID

**Decision**: Use `SharedString` ids of the form `"detail-added-<item.id>"` so each item's "Added" cell has a stable, unique element identity within the window.

### Row rendering change

`render_metadata_table` currently builds a `Vec<(&'static str, String)>` then iterates it. The "Added" row cannot be expressed as a plain string value — it needs a `div().id(...).tooltip(...)`. The refactor keeps the `Vec` for all static rows and appends the "Added" row as a special child after the loop.

## Risks / Trade-offs

- **Calendar arithmetic without a crate**: The Gregorian algorithm is a known pitfall. The implementation MUST be tested against known dates (unit tests in `datetime.rs`).
- **`date_added: Option<i64>`**: Callers that construct `LibraryItem::new(...)` must be updated. Currently only `stubs.rs` does this; the SDK adapter doesn't exist yet, so migration risk is low.

## Open Questions

_(none)_
