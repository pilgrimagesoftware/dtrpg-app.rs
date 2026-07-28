## Why

The toolbar's sort control is a bare text button ("Title", "Publisher", etc.) with only a
dropdown caret — nothing in its own appearance signals it's a *sort* control specifically,
unlike the layout switcher next to it, which uses icons per option. A leading icon makes the
control's purpose recognizable at a glance, consistent with the rest of the toolbar.

## What Changes

- Add a leading sort icon to the toolbar's sort dropdown button, shown regardless of which sort
  method or direction is currently active.
- No change to the dropdown's menu contents, available sort methods, or click behavior.

## Capabilities

### New Capabilities

*(none)*

### Modified Capabilities

- `libri-toolbar`: The "Toolbar MUST provide a sort dropdown" requirement gains a visual
  requirement that the control display a leading sort icon alongside its label.

## Impact

- `dtrpg-ui/src/ui/views/toolbar_view.rs`: `render_sort_selector` gains a leading `.icon(...)`
  on the `Button` (mirroring the existing `Tab::icon` usage in `render_layout_switcher` just
  below it).
- Uses an existing icon asset already present under `assets/icons/` (e.g.
  `list-sort-ascending.svg` or an equivalent generic sort glyph) — no new asset needed unless
  none of the existing ones read well at button-icon size.
- No controller, persistence, or spec-level behavior changes beyond the visual requirement.
