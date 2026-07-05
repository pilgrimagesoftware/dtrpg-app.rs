## Why

The Account section's Email and API Key rows (`render_authenticated` in `settings_account_view.rs`) each build their own `div().flex().items_baseline()` row with a label and a monospace value. Labels aren't aligned to a shared column width, so "Email" and "API Key" start at different horizontal positions rather than right-aligning against a common edge. Additionally, `items_baseline()` aligns by each text run's typographic baseline, but the label (default font) and value (monospace font) resolve to different baseline metrics at the same font size, so the two texts visibly sit at different heights within the same row despite the `items_baseline()` intent.

## What Changes

- Email and API Key rows share a fixed-width label column with labels right-aligned against that column's edge, so both labels line up flush against the value column regardless of "Email" vs. "API Key" text length.
- Replace `items_baseline()` with `items_center()` (matching `gpui-component`'s `h_flex()` convention, which already resolves to `items_center()`) so label and value visually center within the row height instead of relying on cross-font baseline metrics that don't match.
- No visual change to the row's spacing, colors, or the monospace treatment of the value text (kept from the prior `settings-api-key-monospace` change).

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `account-section-layout`: Adds a requirement for the Email/API Key info rows' label column alignment and label/value vertical alignment; does not change the existing Reset API Key button positioning requirement already documented for this capability.

## Impact

- `dtrpg-ui/src/ui/views/settings_account_view.rs`: `render_authenticated`'s Email and API Key row construction changes from independent `items_baseline()` rows to a shared right-aligned label column with `items_center()` alignment, likely by reusing `gpui-component`'s `DescriptionList`/`DescriptionItem` (horizontal layout, `bordered(false)`, right-aligned label content) rather than hand-rolled `div().flex()` rows, consistent with this codebase's existing use of `DescriptionList` in `detail_panel_view.rs`.
- No changes to `SettingsController`, data models, or other settings sections.
