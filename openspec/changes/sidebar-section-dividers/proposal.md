## Why

The sidebar presents three distinct groups (smart filters, publishers, collections) without visual separation, making the boundaries between sections unclear at a glance. Adding thin horizontal dividers between the groups improves scannability.

## What Changes

- Add a horizontal divider rule between the four smart-filter items (All Titles, Recently Added, On This Device, In the Cloud) and the Publishers section.
- Add a horizontal divider rule between the Publishers section and the Collections section.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `rust-library-ui-implementation`: Sidebar SHALL render horizontal dividers between the smart-filter group, publishers group, and collections group.

## Impact

- `crates/dtrpg-ui/src/ui/views/sidebar_view.rs`: insert divider elements between `SidebarGroup` children in `render_sidebar`.
- No controller, data model, or event changes.
