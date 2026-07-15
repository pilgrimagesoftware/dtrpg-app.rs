## Why

The detail panel's metadata table currently shows the publication year as a bare number ("2023") and has no "Date Added" row at all. When a user wants to know when they purchased a title or how long ago something was released, they get no meaningful answer. Dates should be human-readable, contextually relative ("3 days ago", "last week"), and expose the precise absolute value on hover.

## What Changes

- Add a `date_added: Option<i64>` field to `LibraryItem` (Unix timestamp seconds) populated from the DriveThruRPG API's purchase/add date; stub data uses synthetic timestamps relative to startup time.
- Add `util/datetime.rs` with two pure functions: `format_relative(ts: i64) -> String` and `format_absolute(ts: i64) -> String`. No new dependencies — uses `std::time` only.
- Update `render_metadata_table` in `detail_panel_view.rs`:
  - Add an "Added" row showing the relative label; each value cell is a stateful div with a `.tooltip(...)` that renders the absolute date/time string via `Tooltip::new(...).build(...)`.
  - Update the "Released" row to show the year as-is (it's already a bare year — tooltip isn't meaningful without a full date).
- Populate `date_added` in the stub data with synthetic timestamps spread across a realistic recency range.

## Capabilities

### New Capabilities

- `detail-view-date-added`: The detail panel displays a human-readable "Added" date with relative text and an absolute-date tooltip.

### Modified Capabilities

_(none — no existing spec requirement changes)_

## Impact

- `crates/dtrpg-ui/src/data/library.rs`: `LibraryItem` struct, `LibraryItem::new()`
- `crates/dtrpg-ui/src/util/mod.rs`: expose new `datetime` module
- `crates/dtrpg-ui/src/util/datetime.rs`: new file
- `crates/dtrpg-ui/src/ui/views/detail_panel_view.rs`: `render_metadata_table`
- No new crate dependencies
