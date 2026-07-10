## Why

The detail tab's `DescriptionList`-based metadata (System, Released, Format, File Size, Category, Pages, Added, Updated, file id, download location, and the "Other details" disclosure fields) renders its values in the app's default serif body font (`Hoefler Text`), inconsistent with Advanced settings' "Cache details" values, which already use the dedicated sans-serif `VALUE_FONT` (`data/constants.rs`) to visually distinguish data from labels/prose. The detail view has by far the largest concentration of label/value metadata in the app and was never updated when `VALUE_FONT` was introduced.

## What Changes

- Apply `VALUE_FONT` to every value cell in the detail tab's `DescriptionList` usages: the primary entry-tier metadata table (System, Released, Format, File Size, Category, Pages, Added, Updated), the item-tier file metadata table (Name, Format, File Size), the file-detail disclosure (File ID, Download Location), and the "Other details" disclosure (Stable ID, Numeric ID, Order Product ID, Product ID, Added Order, Cover Color).
- Labels, section headers, and prose (descriptions, tooltips) are unaffected — only the value column changes font.
- Uses the same `VALUE_FONT` constant `settings_advanced_view.rs` already uses today; no new font or configuration surface introduced here. `settings-appearance-fonts` (a separate, not-yet-implemented proposal) will make this font user-selectable — when that lands, whichever call sites this change adds will need the same migration `settings_advanced_view.rs`'s call sites get there (see design.md).

## Capabilities

### New Capabilities

- `detail-panel-value-typography`: the detail tab's metadata values render in the app's dedicated value font, matching Advanced settings' existing "Cache details" convention.

### Modified Capabilities

- None

## Impact

- `dtrpg-ui/src/ui/views/detail_panel_view.rs`: `copyable_value` and `render_relative_date_value` (shared helpers already used by most value cells) gain `.font_family(VALUE_FONT)`; a new small helper covers the remaining plain-string value cells (System, Released, Format, File Size ×2, Category, Pages, Added Order) not already routed through those two helpers.
- No controller, data model, or persisted-state changes.
