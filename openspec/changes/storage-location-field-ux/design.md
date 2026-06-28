## Context

`settings_storage_view.rs` renders the storage settings section as a vertical stack: path field row, then a separate buttons row, then a note. The buttons ("Change…" and "Show in Finder/Explorer/Files") are full-text labels in styled `div` elements. `SettingsController` holds the path but performs no background checks; `SettingsSnapshot` has no existence flag. The theme already provides `warning_bg` and `warning_text` color tokens.

## Goals / Non-Goals

**Goals:**
- Collapse the path + action buttons into one horizontal row
- Replace button text with icons and move labels to tooltips
- Add a `storage_path_exists: bool` field to `SettingsSnapshot`
- Spawn a background existence check in `SettingsController` after every path change and on init
- Render a conditional warning row beneath the path row using theme warning colors

**Non-Goals:**
- Making the path field editable (it remains read-only; picker is the only write path)
- Fetching or creating the missing directory automatically
- Caching or debouncing beyond the existing `cx.spawn` + `entity.update` pattern already used in the codebase

## Decisions

### Background check via `cx.spawn` + `entity.update`

`SettingsController::check_storage_path_exists` spawns a background task using `cx.spawn(async move { ... })`, checks `std::path::Path::exists()`, then calls `entity.update(cx, |ctrl, cx| ctrl.set_storage_path_exists(exists, cx))` to write the result back and notify. This matches the existing avatar byte fetch pattern in the same controller.

### `storage_path_exists` initialized to `true`, set to the check result

Defaulting to `true` prevents a flash of the warning on startup before the check completes. The controller calls `check_storage_path_exists` in `new()` so the actual state is resolved quickly.

### Icon choices — Unicode glyphs, no SVG asset dependency

"Change…" → `"📂"` (open folder) or `"…"` — use `"…"` (ellipsis + folder implied) or a simple `"⏏"`. Prefer `"📂"` for clarity.
"Show in Finder/Explorer/Files" → `"↗"` (external link/reveal), consistent with the reveal icon already used in catalog rows.

Use the same Unicode glyph approach already established in the codebase (catalog rows use `"↗"`, avatar uses `"👤"`, etc.).

### Tooltip via GPUI `tooltip()`

Both icon buttons are `div` elements with `.id()`, `.cursor_pointer()`, `.on_click()`, and `.tooltip(|w,cx| Tooltip::new("...").build(w,cx))`. This matches the existing unauthenticated avatar button in `toolbar_view.rs`.

### Layout: path field takes `flex_1()`, buttons are `flex_none()`

Row: `div().flex().items_center().gap(px(8.0))` with the path display taking `flex_1().min_w_0()` and each icon button as a fixed-size square.

### Warning row: conditional child

`render_storage_section` receives `storage_path_exists: bool` from the snapshot. When `false`, it appends a warning row styled with `warning_bg` / `warning_text` from the theme.

## Risks / Trade-offs

- **Check race on rapid path changes**: If the user changes the path twice quickly, both background tasks race. The second task overwrites the first — acceptable, since only the current path matters.
- **`storage_path_exists` defaulting to `true`**: Paths that were deleted before app launch will not show a warning until the background check completes (typically < 100ms). Acceptable UX for a settings panel.
- **Unicode emoji rendering**: `"📂"` may render differently across platforms. If visual inconsistency is a problem, switch to the ASCII `"…"` for change and `"↗"` for reveal (matching existing catalog row usage).
