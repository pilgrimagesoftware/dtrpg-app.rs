## Why

When a user selects a publisher from the sidebar, the toolbar title reads "Publisher" with no indication of which publisher is active. Users have to look back at the sidebar to confirm which publisher they filtered to, adding unnecessary friction.

## What Changes

- The toolbar title for the publisher filter changes from the static string `"Publisher"` to `"Publisher: <name>"` where `<name>` is the selected publisher's name (e.g., `"Publisher: Kobold Press"`).
- `section_title_for` is updated from returning `&str` to returning `String` so it can construct the dynamic label.

## Capabilities

### New Capabilities

- `publisher-filter-title`: When a publisher filter is active, the toolbar section title includes the publisher name in the format "Publisher: <name>".

### Modified Capabilities

## Impact

- `dtrpg-ui/src/ui/views/toolbar_view.rs` — `section_title_for` return type changes from `&str` to `String`; the `Publisher(_)` arm constructs `format!("Publisher: {name}")`.
- No changes to data model, controller, sidebar, or SDK.
