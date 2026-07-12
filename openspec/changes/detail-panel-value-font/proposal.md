## Why

The detail tab's `DescriptionList`-based metadata (System, Released, Format, File Size, Category, Pages, Added, Updated, file id, download location, and the "Other details" disclosure fields) renders both its labels and its values in the app's default serif body font (`Hoefler Text`), with nothing to visually separate a field's name from its data. This change gives the label column its own dedicated sans-serif treatment so the two read distinctly, while the value column keeps rendering in the default body font.

## What Changes

- Apply the app's sans-serif value-font role to every label cell in the detail tab's `DescriptionList` usages: the primary entry-tier metadata table (System, Released, Format, File Size, Category, Pages, Added, Updated), the item-tier file metadata table (Name, Format, File Size), the file-detail disclosure (File ID, Download Location), and the "Other details" disclosure (Stable ID, Numeric ID, Order Product ID, Product ID, Added Order, Cover Color).
- Values, section headers ("Other details"), and prose (descriptions, tooltips) are unaffected — only the label column changes font.
- Consumes the live label-font selection from `cx.global::<LibriTheme>().fonts.label_font` rather than a hardcoded constant. `settings-appearance-fonts` introduced that field (defaulting to `Gotham`) specifically for this use case; this change is sequenced to land after it, so there's no interim constant to migrate later.

## Capabilities

### New Capabilities

- `detail-panel-label-typography`: the detail tab's metadata labels render in the app's dedicated value-font role, visually distinguishing field names from their data.

### Modified Capabilities

- None

## Impact

- `dtrpg-ui/src/ui/views/detail_panel_view.rs`: a new `styled_label(label: impl Into<SharedString>, label_font_family: &str) -> AnyElement` helper wraps every `DescriptionItem::new(...)` label argument (`t!(...).to_string()` calls) across `render_metadata_table`, `render_item_metadata`, `render_file_other_details`, and `render_other_details`; `render_metadata_table`'s existing `category_label` (already an `AnyElement`) gains `.font_family(label_font_family)` directly. `render_metadata_table` gains a `label_font_family: &str` parameter since it doesn't currently take `cx`.
- No controller, data model, or persisted-state changes.
- Depends on `settings-appearance-fonts` (specifically its `LibriTheme.fonts.label_font` field) landing first — already the case, on branch `feature/settings-appearance-fonts`.
