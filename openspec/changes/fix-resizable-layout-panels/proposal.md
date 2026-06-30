## Why

The resizable three-column layout introduced in `adopt-gpui-component-primitives` has incorrect panel behavior: the catalog panel does not fill available space correctly, the hidden detail panel still consumes layout width (preventing the catalog from reaching the window edge), and there is no minimum width constraint on the catalog, allowing the detail panel to grow until the catalog is invisible.

## What Changes

- Remove any independent size constraints from the catalog (middle) panel so it is a pure flex-fill panel with no self-managed handles; the only handles relevant to the catalog are the sidebar's right-edge handle (on the catalog's left) and the detail panel's left-edge handle (on the detail's left).
- Fix the hidden detail panel so it takes zero layout width when no item is selected; the catalog fills the full space between the sidebar and the right window edge in that state.
- Add a minimum width constraint to the catalog panel (e.g. 280 px) so that when the detail panel is visible and the user drags its handle left, the catalog always remains at least minimally visible. When the detail handle would push the catalog below its minimum, the handle stops - the catalog's right edge stays exposed.
- Ensure the detail panel's left handle is the sole control for the catalog/detail boundary; no extra handles appear on the catalog's right side.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `rust-main-window-library-layout`: The three-column resizable layout SHALL define explicit rules for how the catalog (multi-item) and detail (single-item) panels relate: catalog fills available space; detail is hidden or sized by its own left handle; catalog has an enforced minimum width.

## Impact

- `crates/dtrpg-ui/src/ui/views/root_view.rs`: adjust `resizable_panel()` configuration for the catalog and detail panels in `LibraryRootView::render`.
- `crates/dtrpg-ui/src/data/ui_prefs.rs`: no API change; persisted widths continue to work.
- No controller, service, or event changes required.
