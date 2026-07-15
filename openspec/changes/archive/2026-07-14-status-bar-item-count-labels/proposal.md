## Why

The status bar's `library_summary` and `active_tab_summary` labels render bare counts
(`"{total_count} • {total_size_str}"`, `"{active_tab_label} • {active_tab_count}"`) with no
noun, so a user sees a number with no indication of what it counts. The catalog toolbar
already solved this with a `pluralize` utility (`count-pluralization` spec); the status bar
should reuse it instead of printing a naked number.

## What Changes

- `library_summary` renders `"{pluralize(total_count, "item", "items")} • {total_size_str}"`
  (e.g. `"128 items • 2.1 GB"`).
- `active_tab_summary` renders `"{active_tab_label} • {pluralize(active_tab_count, "item", "items")}"`.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `count-pluralization`: The status bar's library total and active-tab summary labels use
  `pluralize` for their item counts, consistent with the toolbar count label.

## Impact

- `crates/dtrpg-ui/src/ui/views/status_bar_view.rs`: `render_status_bar` calls
  `crate::util::pluralize::pluralize` for `library_summary` and `active_tab_summary`
  instead of formatting the raw `usize` directly.
