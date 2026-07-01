## Context

`gpui-component`'s `DescriptionList` already supports two layouts via `Axis`:
- `DescriptionList::new()` / `.horizontal()` -- label and value on the same row (current)
- `DescriptionList::vertical()` -- label above value (desired)

No new code is needed in the component library. The change is entirely in the call site.

## Goals / Non-Goals

**Goals:**
- Render metadata labels above their values in the detail panel

**Non-Goals:**
- Changing column count, sizing, or border style

## Decisions

Change `DescriptionList::new().columns(1)` to `DescriptionList::vertical().columns(1)`. The `columns(1)` call is still valid and meaningful in vertical mode -- it controls how many items share a row before wrapping. With `columns(1)` each item already occupies a full row, so vertical layout stacks every label-value pair cleanly.

## Risks / Trade-offs

- None. This is a one-line change to a single call site with no data model impact.
